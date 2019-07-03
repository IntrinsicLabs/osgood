use futures::stream::Stream;
use futures::sync::{mpsc, oneshot};
use futures::{future, Future};

use hyper::body;
use hyper::{
    header::{HeaderMap, HeaderName},
    Body, Request, Response, StatusCode,
};

use glob::Pattern;

use path_clean::clean;

use super::config::Policy;
use super::osgood_v8::wrapper::*;
use super::osgood_v8::V8;
use super::ResponseResult;

use tokio::runtime::current_thread;
use tokio_threadpool;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str;

use tokio;

#[macro_use]
pub mod logging;

mod fetch;
mod headers;
mod inbound;
mod internal;
mod policies;
mod timers;

/// The size of the MPSC channel buffer (in addition to the number of channel senders).
static BUFFER_SIZE: usize = 1024;

type ResponseResultSender = oneshot::Sender<ResponseResult>;
pub type Message = (Request<Body>, ResponseResultSender);

static PREAMBLE: &str = include_str!("../js/dist/preamble.js");

lazy_thread_local!(CONTEXT, set_context, Local<Context>);
lazy_thread_local!(FETCH_TX, set_fetch_tx, mpsc::Sender<Message>);
lazy_thread_local!(MODULE_MAP, set_module_map, HashMap<i32, std::string::String>);

thread_local! {
    static MODULE_CACHE: RefCell<HashMap<PathBuf, Persistent<V8::Module>>> = RefCell::new(HashMap::new());
}

thread_local! {
    static WORKER_ID: RefCell<usize> = RefCell::new(0);
}

thread_local! {
    static ISOLATE_SPAWN_HANDLE: RefCell<Option<current_thread::Handle>> = RefCell::new(None);
}

thread_local! {
    static TEARDOWN_SENDER: RefCell<Option<mpsc::Sender<()>>> = RefCell::new(None);
}

/// A single instance of a worker.
pub struct Worker {
    pub id: usize,
    pub sender: mpsc::Sender<Message>,
    origin: std::string::String,
    pattern: Pattern,
    method: std::string::String,
}

impl Worker {
    /// Create a new worker with the given handler script.
    pub fn new(
        origin: &str,
        method: &str,
        pattern: Pattern,
        handler: &str,
        handler_filename: &str,
        policies: Vec<Policy>,
        route: &str,
        num_instances: usize,
    ) -> Worker {
        // TODO: Once we add support for multiple origins, we should add origin to the name
        let name = format!("{} {}", method, pattern);
        let (inbound_tx, inbound_rx) = mpsc::channel(BUFFER_SIZE); // for inbounds
        let (outbound_tx, outbound_rx) = mpsc::channel(BUFFER_SIZE); // for outbounds
        let handler = handler.to_owned();
        let origin_str = origin.to_owned();
        let handler_filename = handler_filename.to_owned();
        let route = route.to_owned();

        let worker_isolate_pool = tokio_threadpool::Builder::new()
            .after_start(move || {
                // Copy everything as needed into this thread
                let name = name.clone();
                let handler = handler.clone();
                let handler_filename = handler_filename.clone();
                let route = route.clone();
                let outbound_tx = outbound_tx.clone();
                let policies = policies.clone();

                // Channel for sending/receiving the current_thread handle from the sidecar thread
                let (handle_tx, handle_rx) = std::sync::mpsc::channel::<current_thread::Handle>();

                // Channel for sending/receiving isolate teardown signals
                let (teardown_tx, teardown_rx) = mpsc::channel::<()>(0);

                TEARDOWN_SENDER.with(|s| {
                    *s.borrow_mut() = Some(teardown_tx.clone());
                });

                // Each Tokio thread has its own complementary, possibly-blocking isolate instance
                std::thread::spawn(move || {
                    super::NAME.with(|n| {
                        *n.borrow_mut() = name.to_string();
                    });
                    policies::set_policies(policies);
                    let mut runtime = current_thread::Runtime::new().unwrap();

                    let isolate = Isolate::new();
                    isolate.enter();
                    set_module_map(HashMap::new());

                    handle_scope!({
                        let mut context = Context::new();
                        context.enter();
                        set_context(context);
                        set_fetch_tx(outbound_tx);

                        make_globals(context, &route);
                        run_module(context, PREAMBLE, "preamble.js", None)
                            .expect("Preamble failed to execute");
                        internal::run_internal_module(context, "internal:index.js")
                            .expect("Bootstrap failed to execute");

                        match run_module(
                            context,
                            &handler,
                            &handler_filename,
                            Some(&handler_filename),
                        ) {
                            Err(errstr) => {
                                log_worker_error!("Worker failed to start due to thrown error:");
                                log_worker_error!("{}", errstr);
                                teardown_tx.clone().try_send(()).unwrap();
                            }
                            Ok(module) => {
                                log_info!("Worker started");

                                let mut exports = module.get_exports(context).unwrap();

                                let default_export = exports.get(context, "default");
                                if default_export.is_function() {
                                    let mut global = context.global();
                                    global.set_private(
                                        context,
                                        "worker_handler",
                                        default_export.to_function(),
                                    );
                                } else {
                                    log_worker_warning!("Worker did not export a default handler");
                                }

                                handle_tx.send(runtime.handle()).unwrap();
                            }
                        }
                        runtime.block_on(teardown_rx.into_future()).unwrap();
                        runtime.run().unwrap();

                        context.exit();
                        CONTEXT.with(|c| {
                            *c.borrow_mut() = None;
                        });
                    });

                    log_trace!("Tearing down worker");
                    isolate.exit();
                    isolate.dispose();
                });

                ISOLATE_SPAWN_HANDLE.with(|h| {
                    *h.borrow_mut() = match handle_rx.recv() {
                        Ok(handle) => Some(handle),
                        _ => None,
                    }
                });
            })
            .name_prefix(format!("{}-{}-", method, pattern))
            .pool_size(num_instances)
            .build();

        let _ = worker_isolate_pool
            .spawn_handle(future::lazy(move || {
                ISOLATE_SPAWN_HANDLE.with(|h| {
                    let handle = h.borrow_mut().clone();

                    handle
                        .and_then(|h| h.status().ok())
                        .ok_or(Err::<(), ()>(()))
                })
            }))
            .wait()
            .and_then(|_| {
                // Link up the inbound connection queue to the worker threadpool
                tokio::spawn(future::lazy(move || {
                    inbound_rx.for_each(move |message: Message| {
                        log_trace!("Inbound fetch");
                        let origin_str = origin_str.clone();
                        worker_isolate_pool.spawn(future::lazy(move || {
                            ISOLATE_SPAWN_HANDLE
                                .with(|h| {
                                    let handle = h.borrow_mut().clone().unwrap();
                                    handle.spawn(process_incoming_message(message, &origin_str))
                                })
                                .unwrap();
                            future::ok(())
                        }));
                        future::ok(())
                    })
                }));

                // Link up the outbound connection queue to the hyper threadpool
                tokio::spawn(future::lazy(move || {
                    outbound_rx
                        .for_each(move |(req, tx): Message| {
                            log_trace!("Outbound fetch");
                            if req.uri().scheme_str().unwrap() == "https" {
                                hyper::rt::spawn(fetch::fetch_https_outbound(req, tx));
                            } else {
                                hyper::rt::spawn(fetch::fetch_http_outbound(req, tx));
                            }
                            future::ok(())
                        })
                        .map_err(|e| error!("{:?}", e))
                }));

                Ok(())
            });

        let id = WORKER_ID.with(|i| {
            let id = *i.borrow();
            *i.borrow_mut() = id + 1;
            id
        });

        Worker {
            id,
            sender: inbound_tx,
            origin: origin.to_string(),
            pattern,
            method: method.to_string(),
        }
    }

    pub fn matches(&self, origin: &str, method: &str, route: &str) -> bool {
        let formatted_route = match &route.find('?') {
            Some(idx) => &route[0..*idx],
            None => route,
        };
        self.origin == origin && self.method == method && self.pattern.matches(&formatted_route)
    }
}

fn get_context() -> Local<Context> {
    CONTEXT.with(|c| {
        c.borrow_mut()
            .clone()
            .expect("CONTEXT should have been set by now")
    })
}

fn get_module_map() -> HashMap<i32, std::string::String> {
    MODULE_MAP.with(|m| {
        m.borrow_mut()
            .clone()
            .expect("MODULE_MAP should have been set by now")
    })
}

fn process_incoming_message(
    message: Message,
    origin: &str,
) -> Box<Future<Item = (), Error = ()> + Send> {
    let origin = origin.to_owned();
    Box::new(future::lazy(move || {
        current_thread::spawn(inbound::handle_inbound(message, &origin));
        future::ok(())
    }))
}

fn make_globals(mut context: Local<Context>, route: &str) {
    let mut global = context.global();
    let mut obj = Object::new();

    global.set("self", global);
    obj.set("_route", route);
    obj.set_extern_method(context, "sendError", inbound::send_error);
    obj.set_extern_method(context, "startResponse", inbound::start_response);
    obj.set_extern_method(context, "writeResponse", inbound::write_response);
    obj.set_extern_method(context, "stringResponse", inbound::string_response);
    obj.set_extern_method(context, "setFetchHandler", fetch::set_fetch_handler);
    obj.set_extern_method(context, "setTimerHandler", timers::set_timer_handler);
    obj.set_extern_method(
        context,
        "setIncomingReqHeadHandler",
        inbound::set_inbound_req_head_handler,
    );
    obj.set_extern_method(context, "setTimeout", timers::set_timeout);
    obj.set_extern_method(context, "setInterval", timers::set_interval);
    obj.set_extern_method(context, "clearTimer", timers::clear_timer);
    obj.set_extern_method(context, "_log", log);
    obj.set_extern_method(context, "_error", error);
    obj.set_extern_method(context, "_fetch", fetch::start_fetch);
    if let Ok(_var) = std::env::var("DEBUG") {
        obj.set_extern_method(context, "debug", debug);
    }
    obj.set_extern_method(context, "getPrivate", get_private);
    global.set("_bindings", obj);
}

fn run_module(
    context: Local<Context>,
    source: &str,
    name: &str,
    path: Option<&str>,
) -> Result<Local<Module>, std::string::String> {
    let src = v8_str!(source);
    let name = v8_str!(name);
    let mut module = Module::compile(*src, *name)?;
    if let Some(path) = path {
        let mut modmap = get_module_map();
        let id = module.get_hash();
        modmap.insert(id, path.into());
        set_module_map(modmap.clone());
    }
    module.instantiate(context, Some(resolve_module))?;
    module.evaluate(context)?;
    Ok(module)
}

extern "C" fn resolve_module(
    context: V8::Local<V8::Context>,
    specifier: V8::Local<V8::String>,
    referrer: V8::Local<V8::Module>,
) -> V8::MaybeLocal<V8::Module> {
    let specifier = PathBuf::from(Local::from(specifier).as_rust_string());
    let full_path = if specifier.is_absolute() {
        specifier
    } else {
        let referrer_id = Local::from(referrer).get_hash();
        let modmap = get_module_map();
        let base_path = modmap
            .get(&referrer_id)
            .expect("Referrer module must be set in module map");
        let base_path = PathBuf::from(base_path);
        let base_path = base_path.parent().unwrap();
        let base_path = PathBuf::from(base_path);
        PathBuf::from(clean(base_path.join(specifier).to_str().unwrap()))
    };

    let maybe_module = MODULE_CACHE.with(|cache| {
        cache.borrow().get(&full_path).and_then(|persistent| {
            let local: Local<V8::Module> = persistent.into();
            Some(local.into())
        })
    });
    if let Some(maybe_module) = maybe_module {
        maybe_module
    } else {
        let full_path_str = full_path.to_str().unwrap();
        match std::fs::read_to_string(&full_path) {
            Ok(module) => {
                let maybe_module = run_module(
                    Local::from(context),
                    module.as_str(),
                    full_path_str,
                    Some(full_path_str),
                );

                match maybe_module {
                    Ok(local_module) => {
                        MODULE_CACHE.with(|cache| {
                            let mut cache = cache.borrow_mut();
                            let persistent: Persistent<V8::Module> = local_module.into();
                            cache.insert(full_path.clone(), persistent);
                        });
                        local_module.into()
                    }
                    Err(err) => {
                        log_worker_error!("{}: {}", full_path_str, err);
                        Module::empty_and_throw(&err.to_string())
                    }
                }
            }
            Err(err) => {
                log_worker_error!("{}: {}", full_path_str, err);
                Module::empty_and_throw(&err.to_string())
            }
        }
    }
}

#[v8_fn]
fn log(args: FunctionCallbackInfo) {
    println!(
        "[{}] {}",
        logging::color_name(),
        args.get(0).unwrap().as_rust_string()
    );
}

#[v8_fn]
fn error(args: FunctionCallbackInfo) {
    eprintln!(
        "[{}] {}",
        logging::color_name(),
        args.get(0).unwrap().as_rust_string()
    );
}

#[v8_fn]
fn debug(args: FunctionCallbackInfo) {
    log_debug!("JS debug: {}", args.get(0).unwrap().as_rust_string());
}

#[v8_fn]
fn get_private(args: FunctionCallbackInfo) {
    let ret = Private::for_api(&args.get(0).unwrap().as_rust_string());
    args.set_return_value(&ret);
}
