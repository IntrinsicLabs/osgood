use super::{osgood, Isolate, Local, Object, V8};

pub use V8::Context;

impl Context {
    // TODO this should be on a wrapper struct rather than the native class
    pub fn new() -> Local<V8::Context> {
        unsafe { osgood::new_context(Isolate::raw()).into() }
    }
}

impl Local<Context> {
    pub fn global(&mut self) -> Local<Object> {
        unsafe { self.inner_mut().Global().into() }
    }

    pub fn enter(&mut self) {
        unsafe {
            self.inner_mut().Enter();
        }
    }

    pub fn exit(&mut self) {
        unsafe {
            self.inner_mut().Exit();
        }
    }
}
