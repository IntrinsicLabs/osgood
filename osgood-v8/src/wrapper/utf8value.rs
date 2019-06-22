use super::{V8, Isolate, Valuable};
use std::ffi::CString;
use std::fmt;

pub struct Utf8Value {
    val_: V8::String_Utf8Value
}

impl Utf8Value {
    pub fn new(from: &Valuable) -> Utf8Value {
        Utf8Value {
            val_: unsafe { V8::String_Utf8Value::new(Isolate::raw(), from.as_value().into()) }
        }
    }

    pub fn into_string(&self) -> String {
        let raw = unsafe { CString::from_raw(self.val_.str_) };
        raw.into_string().unwrap()
    }
}

impl Drop for Utf8Value {
    fn drop(&mut self) {
        unsafe { self.val_.destruct(); }
    }
}

impl fmt::Display for Utf8Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.into_string())
    }
}
