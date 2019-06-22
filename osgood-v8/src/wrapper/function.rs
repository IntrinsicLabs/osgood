use super::{Context, IntoValue, Local, V8};

use V8::Function;

impl Local<Function> {
    pub fn call(
        &mut self,
        context: Local<Context>,
        recv: &IntoValue,
        argv: Vec<&IntoValue>,
    ) -> Local<V8::Value> {
        let argc = argv.len() as i32;
        let mut argv: Vec<V8::Local<V8::Value>> =
            argv.iter().map(|&arg| arg.into_value().into()).collect();
        unsafe {
            self.inner_mut()
                .Call(
                    context.into(),
                    recv.into_value().into(),
                    argc,
                    argv.as_mut_ptr(),
                )
                .to_local_checked()
                .unwrap()
        }
    }
}
