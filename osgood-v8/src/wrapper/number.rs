use super::*;

pub use V8::Number;

impl V8::Number {
    pub fn new(i: f64) -> Local<Number> {
        unsafe { V8::Number_New(Isolate::raw(), i).into() }
    }
}

impl Local<V8::Number> {
    pub fn value(&mut self) -> f64 {
        unsafe { self.inner_mut().Value() }
    }
}

impl IntoValue for f64 {
    fn into_value(&self) -> Local<V8::Value> {
        V8::Number::new(*self).into()
    }
}

impl IntoValue for i32 {
    fn into_value(&self) -> Local<V8::Value> {
        V8::Number::new(f64::from(*self)).into()
    }
}

impl IntoValue for u16 {
    fn into_value(&self) -> Local<V8::Value> {
        V8::Number::new(f64::from(*self)).into()
    }
}
