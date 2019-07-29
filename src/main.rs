#![deny(clippy::all)]

#[macro_use]
extern crate osgood_v8;
#[macro_use]
extern crate osgood_v8_macros;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

use futures::sink::Sink;
use futures::sync::oneshot;
use futures::{future, Future};

use hyper::header::HeaderValue;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};

use hyper_staticfile;
use tokio;

use std::cell::RefCell;
use std::path::Path;
use std::string;
use std::sync::Arc;

#[macro_export]
macro_rules! lazy_thread_local {
    ($name:ident, $set_fn:ident, $type:path) => {
        thread_local! {
            static $name: std::cell::RefCell<Option<$type>> = std::cell::RefCell::new(None);
        }

        fn $set_fn(thing: $type) {
            $name.with(|i| {
                *i.borrow_mut() = Some(thing);
            });
        }
    };
}

mod config;
mod worker;

use config::*;
use worker::Worker;

thread_local! {
    static NAME: RefCell<string::String> = RefCell::new("------".to_string());
}

type FutureResponse = Box<Future<Item = Response<Body>, Error = std::io::Error> + Send>;
type ResponseResult = Result<Response<Body>, string::String>;

fn main() {
    let (v8_flags, options) = parse_args();
    osgood_v8::wrapper::platform_init(&v8_flags);

    tokio::run(future::lazy(move || {
        pretty_env_logger::init();

        let config_file = options.value_of("APPFILE").unwrap();
        let config = Config::new(config_file);
        if let Err(err) = config {
            log_osgood_error!("{} failed to load due to:", config_file);
            log_osgood_error!("{}", err);
            std::process::exit(1);
        }
        let config = config.unwrap();
        let addr = config.addr;
        let origin = (&config.origin.origin).clone();
        let port = addr.port();
        let static_routes = config.origin.static_routes.clone();
        let workers = Arc::new(make_workers(config).unwrap());

        // Define the HTTP service
        let service = move || {
            let workers = workers.clone();
            let static_routes = static_routes.clone();

            service_fn(move |mut req: Request<Body>| -> FutureResponse {
                let method = req.method().to_string();
                // TODO: need to change protocol based on http vs https
                let mut origin = format!(
                    "http://{}",
                    req.headers()[hyper::header::HOST].to_str().unwrap()
                );
                if port != 443 && port != 80 && !origin.ends_with(format!(":{}", port).as_str()) {
                    origin = format!("{}:{}", origin, port);
                }
                let route = req.uri().to_string();
                log_info!("{} {} {}", req.method(), origin, route);
                let mut worker = None;
                for a_worker in workers.iter() {
                    if a_worker.matches(&origin, &method, &route) {
                        worker = Some(a_worker);
                        break;
                    }
                }
                let worker = match worker {
                    Some(worker) => worker,
                    None => {
                        for static_route in static_routes.iter() {
                            let route_prefix = &static_route.route_prefix;
                            let directory = &static_route.directory;
                            let default_file = &static_route.index;
                            let clean_html_ext = static_route.clean_html_ext;

                            if !req.uri().path().starts_with(route_prefix) {
                                continue;
                            }
                            let original_path = req.uri().path();
                            let mut path = req.uri().path().replace(route_prefix, "");

                            let mut is_redirect = None;

                            let mut clean_url = false;
                            if clean_html_ext {
                                let reversed_path =
                                    path.trim_end_matches('/').chars().rev().collect::<String>();
                                if reversed_path.is_empty() {
                                    clean_url = false;
                                } else {
                                    let last_slash_index = match reversed_path.find('/') {
                                        Some(index) => index,
                                        None => panic!("This should never happen."),
                                    };
                                    let last_slash_index = path.len() - last_slash_index - 1;
                                    let (_, file_name) = path.split_at(last_slash_index);

                                    if file_name.find('.').is_none() {
                                        clean_url = true;
                                    }

                                    if file_name.ends_with(".html") {
                                        let body = "404\n";
                                        let resp = Response::builder()
                                            .status(StatusCode::NOT_FOUND)
                                            .body(body.into())
                                            .unwrap();
                                        return Box::new(future::ok(resp));
                                    }
                                }
                            }

                            match default_file {
                                Some(file) => {
                                    if clean_url {
                                        let clean_path =
                                            path.trim_start_matches('/').trim_end_matches('/');
                                        let directory_path = Path::new(directory);
                                        let try_path_index =
                                            directory_path.join(clean_path).join("index.html");
                                        let try_path_file =
                                            directory_path.join(clean_path.to_owned() + ".html");
                                        let req_ends_with_slash = original_path.ends_with('/');

                                        if try_path_index.is_file() {
                                            path = path.trim_end_matches('/').to_owned()
                                                + "/index.html";
                                            if !req_ends_with_slash {
                                                is_redirect = Some(original_path.to_owned() + "/");
                                            }
                                        } else {
                                            path = path.trim_end_matches('/').to_owned() + ".html";
                                            if req_ends_with_slash && try_path_file.is_file() {
                                                is_redirect = Some(
                                                    original_path.trim_end_matches('/').to_string(),
                                                );
                                            }
                                        }
                                    } else if path.ends_with('/') {
                                        path = path + file;
                                    } else if path.is_empty() {
                                        path = string::String::from("/") + file;
                                    }
                                }
                                None => {
                                    if clean_url {
                                        let directory_path = Path::new(directory);
                                        let clean_path =
                                            path.trim_start_matches('/').trim_end_matches('/');
                                        let try_path =
                                            directory_path.join(clean_path.to_owned() + ".html");
                                        if path.ends_with('/') && try_path.is_file() {
                                            is_redirect = Some(
                                                original_path.trim_end_matches("/").to_string(),
                                            );
                                            println!("{:?}", is_redirect);
                                        }

                                        path = path.trim_end_matches('/').to_owned() + ".html";
                                    } else {
                                        if path.ends_with('/') {
                                            path = path.trim_end_matches('/').to_string();
                                        }
                                        let directory_path = Path::new(directory);
                                        let clean_path = path.trim_start_matches('/');
                                        let try_path = directory_path.join(clean_path);
                                        if path.is_empty() || !try_path.is_file() {
                                            let body = format!("404\n");
                                            let resp = Response::builder()
                                                .status(StatusCode::NOT_FOUND)
                                                .body(body.into())
                                                .unwrap();
                                            return Box::new(future::ok(resp));
                                        }
                                    }
                                }
                            }

                            if let Some(redirect_path) = is_redirect {
                                let redirect_uri =
                                    format!("{}{}", origin, redirect_path).to_string();
                                let body = format!("301: moved to {}\n", redirect_uri);
                                let mut response = Response::builder()
                                    .status(StatusCode::MOVED_PERMANENTLY)
                                    .body(body.into())
                                    .unwrap();
                                response.headers_mut().insert(
                                    "Location",
                                    HeaderValue::from_str(&redirect_uri).unwrap(),
                                );
                                return Box::new(future::ok(response));
                            }

                            *req.uri_mut() = path.parse().unwrap();
                            let static_result = hyper_staticfile::resolve(&directory, &req);

                            return Box::new(static_result.map(move |result| {
                                match result {
                                    hyper_staticfile::ResolveResult::Found(_, _) => {
                                        let mut response = hyper_staticfile::ResponseBuilder::new()
                                            .build(&req, result)
                                            .unwrap();
                                        let content_type =
                                            mime_guess::guess_mime_type(req.uri().path());
                                        let content_type = content_type.to_string();
                                        let content_type = HeaderValue::from_str(&content_type);
                                        response
                                            .headers_mut()
                                            .insert("content-type", content_type.unwrap());
                                        response
                                    }
                                    _ => hyper_staticfile::ResponseBuilder::new()
                                        .build(&req, result)
                                        .unwrap(),
                                }
                            }));
                        }
                        let body = format!("route not found: {} {} {}\n", method, origin, route);
                        let resp = Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(body.into())
                            .unwrap();
                        return Box::new(future::ok(resp));
                    }
                };

                let sender = worker.sender.clone();

                // Create a one-shot, reverse channel so that the worker thread can send its response
                let (tx, rx) = oneshot::channel();

                // Send the request to the service worker thread, await the response, and send that to
                // the client
                Box::new(sender.send((req, tx)).then(|_| {
                    rx.then(move |res: Result<ResponseResult, oneshot::Canceled>| {
                        if let Ok(res) = res {
                            if let Ok(res) = res {
                                future::ok(res)
                            } else {
                                future::ok(
                                    Response::builder()
                                        .status(StatusCode::SERVICE_UNAVAILABLE)
                                        .body(format!("route not available: {}\n", route).into())
                                        .unwrap(),
                                )
                            }
                        } else {
                            future::ok(
                                Response::builder()
                                    .status(StatusCode::SERVICE_UNAVAILABLE)
                                    .body(format!("route not available: {}\n", route).into())
                                    .unwrap(),
                            )
                        }
                    })
                }))
            })
        };

        log_osgood_message!(
            "{}",
            format!("Application has started, listening on {}", origin)
        );

        Server::bind(&addr)
            .serve(service)
            .map_err(|e| log_osgood_error!("Error: {}", e))
    }));
    //tokio::run(server);

    osgood_v8::wrapper::platform_dispose();
}

fn make_workers(config: Config) -> Result<Vec<Worker>, std::io::Error> {
    let mut workers = Vec::new();
    let origin = config.origin;
    for route in origin.routes {
        if let Ok(handler) = std::fs::read_to_string(&route.worker_file) {
            workers.push(Worker::new(
                &origin.origin,
                &route.method,
                route.pattern,
                &handler,
                &route.worker_file,
                route.policies,
                &route.raw,
            ));
        } else {
            log_osgood_error!("Could not find worker file: {}", &route.worker_file);
            std::process::exit(1);
        }
    }
    Ok(workers)
}

fn parse_args<'a>() -> (string::String, clap::ArgMatches<'a>) {
    let (v8_flags_vec, mut args): (Vec<string::String>, Vec<string::String>) =
        std::env::args().partition(|arg| arg.starts_with("--v8-"));
    let v8_flags_vec: Vec<string::String> = v8_flags_vec
        .iter()
        .map(|f| f.replace("--v8-", "--"))
        .collect();
    let v8_flags = v8_flags_vec.join(" ");
    if v8_flags.contains("--help") {
        // this suppresses error asking for a filename if just trying to get --v8-help, while stil
        // requiring it in all other cases
        args.push("IGNORED_FILENAME_FOR_HELP_PURPOSES".to_owned());
    }
    let options = clap::App::new("osgood")
        .version(crate_version!())
        .arg(
            clap::Arg::with_name("APPFILE")
                .required(true)
                .help("An Osgood Application JavaScript file")
                .index(1),
        )
        .after_help(
            "In addition, you can pass V8 flags prefixing them with \
             '--v8-' instead of just '--'. List them with '--v8-help'.",
        )
        .get_matches_from(args);

    (v8_flags, options)
}
