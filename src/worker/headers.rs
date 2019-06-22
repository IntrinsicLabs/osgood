use super::*;
use osgood_v8::wrapper::{Local, Valuable};
use osgood_v8::V8;

pub fn v8_headers(header_map: &HeaderMap) -> Local<V8::Object> {
    let mut v8_headers = V8::Object::new();
    for (h_name, h_value) in header_map {
        v8_headers.set(h_name.as_str(), h_value.to_str().unwrap());
    }
    v8_headers
}

pub fn rust_headers(v8_headers: Local<V8::Value>, context: Local<V8::Context>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    for (h_name, h_value) in v8_headers.to_object().iter(context) {
        header_map.insert(
            HeaderName::from_bytes((&h_name.as_rust_string()).as_bytes()).unwrap(),
            (&h_value.as_rust_string()).parse().unwrap(),
        );
    }
    header_map
}
