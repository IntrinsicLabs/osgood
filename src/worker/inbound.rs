use super::*;
use hyper::header::HeaderValue;

enum ResponseHolder {
    Resp(body::Sender),
    Tx(ResponseResultSender),
}

thread_local! {
    static REQ_ID_TO_TX: RefCell<HashMap<i32, ResponseHolder>> = RefCell::new(HashMap::new());
}
thread_local! {
    static NEXT_REQ_ID: RefCell<i32> = RefCell::new(0);
}

pub fn handle_inbound((req, tx): Message, origin: &str) -> impl Future<Item = (), Error = ()> {
    let req_id = get_next_req_id();
    REQ_ID_TO_TX.with(|cell| {
        cell.borrow_mut().insert(req_id, ResponseHolder::Tx(tx));
    });
    let mut context = get_context();
    handle_scope!({
        let worker_handler = context.global().get_private(context, "worker_handler");
        let method = req.method().to_string();
        let mut uri = std::string::String::new();
        uri.push_str(&origin);
        uri.push_str(&req.uri().to_string());
        let v8_headers = headers::v8_headers(req.headers());
        call_inbound_req_head_handler(
            context,
            vec![&req_id, &worker_handler, &method, &uri, &v8_headers],
        );
    });

    req.into_body()
        .for_each(move |chunk| {
            handle_scope!({
                let chunk = std::string::String::from_utf8(chunk.to_vec()).unwrap();
                call_inbound_req_body_handler(context, vec![&req_id, &chunk]);
            });
            future::ok(())
        })
        .and_then(move |_| {
            handle_scope!({
                call_inbound_req_body_handler(context, vec![&req_id]);
            });
            future::ok(())
        })
        .map_err(|e| {
            // TODO send a "Bad Request" response.
            log_osgood_error!("Inbound request: {}", e);
        })
}

macro_rules! send_response {
    ($req_id:expr, $response:expr) => {
        REQ_ID_TO_TX.with(move |cell| {
            let mut m = cell.borrow_mut();
            let _ = match (*m).remove($req_id).unwrap() {
                ResponseHolder::Tx(tx) => tx.send($response),
                _ => panic!("bad state"),
            };
        });
    };
}

#[v8_fn]
pub fn set_inbound_req_head_handler(args: FunctionCallbackInfo) {
    let mut context = get_context();
    let mut global = context.global();
    let func = args.get(0).unwrap().to_function();
    global.set_private(context, "inbound_req_head_handler", func);
}

#[v8_fn]
pub fn set_inbound_req_body_handler(args: FunctionCallbackInfo) {
    let mut context = get_context();
    let mut global = context.global();
    let func = args.get(0).unwrap().to_function();
    global.set_private(context, "inbound_req_body_handler", func);
}

pub fn call_inbound_req_head_handler(mut context: Local<V8::Context>, args: Vec<&IntoValue>) {
    let null = Isolate::null();
    context
        .global()
        .get_private(context, "inbound_req_head_handler")
        .to_function()
        .call(context, &null, args);
}

pub fn call_inbound_req_body_handler(mut context: Local<V8::Context>, args: Vec<&IntoValue>) {
    let null = Isolate::null();
    context
        .global()
        .get_private(context, "inbound_req_body_handler")
        .to_function()
        .call(context, &null, args);
}

#[v8_fn]
pub fn string_response(args: FunctionCallbackInfo) {
    let req_id = args.get(1).unwrap().to_number().value() as i32;
    let mut response = Response::new(args.get(0).unwrap().as_rust_string().into());
    (*response.headers_mut()).insert("Content-Type", HeaderValue::from_str("text/plain").unwrap());
    send_response!(&req_id, Ok(response));
}

#[v8_fn]
pub fn start_response(args: FunctionCallbackInfo) {
    let context = get_context();
    let mut v8_response = args.get(0).unwrap().to_object();
    let req_id = args.get(1).unwrap().to_number().value() as i32;
    let status_code = v8_response.get(context, "status").to_number().value() as u16;
    let v8_headers = v8_response
        .get(context, "headers")
        .to_object()
        .get(context, "_headers");
    let has_string_body = args.length() == 3;
    let context = Isolate::get_current_context();
    let header_map = headers::rust_headers(v8_headers, context);
    if has_string_body {
        let body = args.get(2).unwrap();
        let mut response = Response::new(body.as_rust_string().into());
        *response.status_mut() = StatusCode::from_u16(status_code).unwrap();
        *response.headers_mut() = header_map;
        send_response!(&req_id, Ok(response));
    } else {
        let (sender, body) = Body::channel();
        let mut response = Response::new(body);
        *response.status_mut() = StatusCode::from_u16(status_code).unwrap();
        *response.headers_mut() = header_map;
        send_response!(&req_id, Ok(response));
        REQ_ID_TO_TX.with(move |cell| {
            let mut m = cell.borrow_mut();
            m.insert(req_id, ResponseHolder::Resp(sender));
        });
    }
}

#[v8_fn]
pub fn write_response(args: FunctionCallbackInfo) {
    let mut v8_chunk = args.get(0).unwrap();
    let req_id = args.get(1).unwrap().to_number().value() as i32;
    let context = Isolate::get_current_context();
    if v8_chunk.as_rust_bool(context) {
        let mut chunk = v8_chunk.to_array_buffer();
        let chunk = chunk.as_vec_u8();
        REQ_ID_TO_TX.with(|cell| {
            let mut m = cell.borrow_mut();
            let response_body_sender = match (*m).get_mut(&req_id).unwrap() {
                ResponseHolder::Resp(x) => x,
                _ => panic!("bad state"),
            };
            let _ = response_body_sender.send_data(chunk.into());
        });
    } else {
        REQ_ID_TO_TX.with(|cell| {
            (*cell.borrow_mut()).remove(&req_id).unwrap();
        });
    }
}

fn get_next_req_id() -> i32 {
    NEXT_REQ_ID.with(|id| {
        let new_id = *id.borrow();
        *id.borrow_mut() = new_id + 1;
        new_id
    })
}
