use futures::stream::Stream;
use futures::sync::oneshot;
use futures::Future;

use hyper::header::USER_AGENT;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;

use futures::sink::Sink;
use tokio::runtime::current_thread;

use super::NULL;
use super::*;

thread_local! {
    static FETCH_ID_TO_TX: RefCell<HashMap<i32, body::Sender>> = RefCell::new(HashMap::new());
}
lazy_thread_local!(FETCH_CB, set_fetch_cb, Persistent<V8::Function>);

macro_rules! fetch_outbound {
    ($fn_name:ident, $client:expr) => {
        pub fn $fn_name(
            req: Request<Body>,
            tx: ResponseResultSender,
        ) -> impl Future<Item = (), Error = ()> {
            $client
                .request(req)
                .map_err(|e| {
                    // TODO There should be an error sent to the caller.
                    log_osgood_error!("Outbound fetch: {}", e);
                    format!("{}", e)
                })
                .then(move |result| {
                    let _ = tx.send(result);
                    future::ok(())
                })
        }
    };
}
fetch_outbound!(fetch_http_outbound, Client::new());
fetch_outbound!(
    fetch_https_outbound,
    Client::builder().build(HttpsConnector::new(4).unwrap())
);

#[derive(Debug, PartialEq)]
enum FetchBodyType {
    Stream,
    String,
    None,
}

#[v8_fn]
pub fn set_fetch_handler(args: FunctionCallbackInfo) {
    let func = args.get(0).unwrap().to_function();
    set_fetch_cb(func.into());
}

pub fn call_fetch_handler(context: Local<V8::Context>, args: Vec<&IntoValue>) {
    let null = Isolate::null();
    FETCH_CB.with(|cb| {
        let mut cb: Local<V8::Function> = cb.borrow().unwrap().into();
        cb.call(context, &null, args);
    });
}

#[v8_fn]
pub fn start_fetch(args: FunctionCallbackInfo) {
    let context = get_context();
    let fetch_id = args.get(4).unwrap().to_number().value() as i32;
    let type_string = args.get(5).unwrap().as_rust_string();
    let body_type = match type_string.as_str() {
        "string" => FetchBodyType::String,
        "stream" => FetchBodyType::Stream,
        _ => FetchBodyType::None,
    };
    if body_type == FetchBodyType::Stream {
        let has_stream = FETCH_ID_TO_TX.with(|cell| cell.borrow().contains_key(&fetch_id));
        if has_stream {
            let mut v8_chunk = args.get(3).unwrap();
            if v8_chunk.is_boolean() {
                FETCH_ID_TO_TX.with(|cell| {
                    cell.borrow_mut().remove(&fetch_id);
                });
                return;
            }
            FETCH_ID_TO_TX.with(|cell| {
                let mut m = cell.borrow_mut();
                let sender = m.get_mut(&fetch_id).unwrap();
                let _ = sender.send_data(v8_chunk.as_rust_string().into());
            });
            return;
        }
    }
    let v8_url_string = args.get(0).unwrap().to_string();
    let v8_headers = args.get(1).unwrap().to_object().get(context, "_headers");
    let v8_method = args.get(2).unwrap().as_rust_string();
    let v8_body_string = args.get(3).unwrap();

    let mut header_map = headers::rust_headers(v8_headers, context);
    if !header_map.contains_key(USER_AGENT) {
        header_map.insert(
            USER_AGENT,
            format!("osgood/{}", crate_version!()).parse().unwrap(),
        );
    }

    let body = match body_type {
        FetchBodyType::String => Body::from(v8_body_string.as_rust_string()),
        FetchBodyType::Stream => {
            let (sender, body) = Body::channel();
            FETCH_ID_TO_TX.with(|cell| {
                cell.borrow_mut().insert(fetch_id, sender);
            });
            body
        }
        FetchBodyType::None => Body::empty(),
    };

    let mut request = Request::new(body);
    *request.headers_mut() = header_map.clone();
    *request.method_mut() = Method::from_bytes(v8_method.as_ref()).unwrap();
    let outbound_url = v8_url_string.as_rust_string();
    *request.uri_mut() = outbound_url.parse().unwrap();

    if !policies::policy_check(&v8_method, &outbound_url, &header_map) {
        // TODO: Replace violation URL host with guilty host header
        let outbound_url = outbound_url.as_str();
        let outbound_url = match outbound_url.find('#') {
            Some(index) => outbound_url.split_at(index).0,
            None => outbound_url,
        };
        let outbound_url = match outbound_url.find('?') {
            Some(index) => outbound_url.split_at(index).0,
            None => outbound_url,
        };
        let error = format!("POLICY_VIOLATION [ {} {} ]", v8_method, outbound_url);
        call_fetch_handler(context, vec![&error, &NULL, &NULL, &fetch_id]);
        return;
    }

    current_thread::spawn(future::lazy(move || {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        FETCH_TX.with(move |tx| {
            tx.borrow_mut()
                .as_mut()
                .unwrap()
                .clone()
                .send((request, oneshot_tx))
                .then(move |_| {
                    oneshot_rx
                        .then(move |res| handle_outbound_response(res.expect("response"), fetch_id))
                })
        })
    }));
}

fn handle_outbound_response(
    res: ResponseResult,
    fetch_id: i32,
) -> Box<Future<Item = (), Error = ()>> {
    let context = get_context();
    if res.is_err() {
        let err = res.unwrap_err();
        handle_scope!({
            call_fetch_handler(context, vec![&err, &NULL, &NULL, &fetch_id]);
        });
        return Box::new(future::ok(()));
    }
    let res = res.unwrap();
    handle_scope!({
        let mut meta = V8::Object::new();
        meta.set("status", res.status().as_u16());
        let status_string = res.status().canonical_reason().unwrap().to_string();
        meta.set("statusText", status_string);
        meta.set("headers", headers::v8_headers(res.headers()));

        call_fetch_handler(context, vec![&NULL, &NULL, &meta, &fetch_id]);
    });
    Box::new(
        res.into_body()
            .for_each(move |chunk| {
                handle_scope!({
                    let chunk = ArrayBuffer::new_from_u8_array(chunk.as_ref(), chunk.len());
                    call_fetch_handler(context, vec![&NULL, &chunk, &NULL, &fetch_id]);
                });
                future::ok(())
            })
            .and_then(move |_| {
                handle_scope!({
                    call_fetch_handler(context, vec![&NULL, &NULL, &NULL, &fetch_id]);
                });
                future::ok(())
            })
            .map_err(|e| {
                // TODO There should be an error sent to the caller.
                log_osgood_error!("Outbound fetch: {}", e);
            }),
    )
}
