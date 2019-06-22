use super::*;

pub use V8::String;

macro_rules! c_str {
    ($str:expr) => {
        format!("{}\0", $str).as_ptr() as *const ::std::os::raw::c_char
    };
}

impl V8::String {
    pub fn new_from_slice(val: &str) -> Local<V8::String> {
        unsafe { V8::String_NewFromUtf8(Isolate::raw(), c_str!(val), 0, -1).into() }
    }
}

impl IntoValue for std::string::String {
    fn into_value(&self) -> Local<V8::Value> {
        V8::String::new_from_slice(&self).into()
    }
}

impl IntoValue for &str {
    fn into_value(&self) -> Local<V8::Value> {
        V8::String::new_from_slice(*self).into()
    }
}
