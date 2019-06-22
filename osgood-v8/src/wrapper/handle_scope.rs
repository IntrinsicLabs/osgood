use super::*;

pub struct HandleScope {
    scope_: V8::HandleScope,
}

impl HandleScope {
    pub fn new() -> HandleScope {
        HandleScope {
            scope_: unsafe { V8::HandleScope::new(Isolate::raw()) },
        }
    }
}

impl Drop for HandleScope {
    fn drop(&mut self) {
        unsafe {
            self.scope_.destruct();
        }
    }
}

impl Default for HandleScope {
    fn default() -> Self {
        Self::new()
    }
}
