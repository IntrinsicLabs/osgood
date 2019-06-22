use super::*;

pub use V8::Array;

impl Local<V8::Array> {
    pub fn length(&mut self) -> i32 {
        unsafe { self.inner_mut().Length() as i32 }
    }
}
