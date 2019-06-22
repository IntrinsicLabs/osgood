use super::*;
use std::convert;

#[derive(Debug, Copy, Clone)]
pub struct Exception {
    exception_: Local<V8::Object>,
}

impl Exception {
    pub fn as_rust_string(&mut self) -> std::string::String {
        let context = Isolate::get_current_context();
        let mut to_string = self.exception_.get(context, "toString").to_function();
        let result = to_string.call(context, self, vec![]);
        result.as_rust_string()
    }

    pub fn syntax_error_stack(&mut self) -> std::string::String {
        unsafe {
            let message =
                V8::Exception_CreateMessage(Isolate::raw(), self.exception_.as_value().into());
            let message = message.val_.as_ref().unwrap();
            let origin = message.GetScriptOrigin();
            let name: Local<V8::Value> = origin.resource_name_.into();
            let name = name.as_rust_string();
            let context: V8::Local<V8::Context> = Isolate::get_current_context().into();
            let line = message.GetLineNumber(context).value_;
            let col = message.GetStartColumn1(context).value_;
            format!(
                "{}\n    at {}:{}:{}",
                self.as_rust_string(),
                name,
                line,
                col
            )
        }
    }
}

impl convert::From<Local<V8::Object>> for Exception {
    fn from(val: Local<V8::Object>) -> Exception {
        Exception { exception_: val }
    }
}

impl convert::From<Local<V8::Value>> for Exception {
    fn from(val: Local<V8::Value>) -> Exception {
        val.to_object().into()
    }
}

impl convert::From<V8::Local<V8::Value>> for Exception {
    fn from(val: V8::Local<V8::Value>) -> Exception {
        let val: Local<V8::Value> = val.into();
        val.to_object().into()
    }
}

impl IntoValue for Exception {
    fn into_value(&self) -> Local<V8::Value> {
        self.exception_.as_value()
    }
}
