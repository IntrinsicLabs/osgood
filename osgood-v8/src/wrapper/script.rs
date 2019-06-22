use super::{osgood, Exception, Isolate, Local, Valuable, V8};

pub use V8::Script;

impl Script {
    pub fn compile(
        ctx: Local<V8::Context>,
        src: Local<V8::String>,
    ) -> Result<Local<V8::Script>, std::string::String> {
        unsafe {
            let result = osgood::compile_script(Isolate::raw(), ctx.into(), src.into());
            if result.is_exception {
                let mut exception: Exception = result.exception.into();
                Err(exception.syntax_error_stack())
            } else {
                Ok(Local::from(result.ret_val))
            }
        }
    }
}

impl Local<Script> {
    pub fn run(
        &mut self,
        context: Local<V8::Context>,
    ) -> Result<Local<V8::Value>, std::string::String> {
        unsafe {
            let result = osgood::run_script(Isolate::raw(), context.into(), (*self).into());
            if result.is_exception {
                Err(Local::from(result.ret_val)
                    .to_object()
                    .get(context, "stack")
                    .as_rust_string())
            } else {
                Ok(Local::from(result.ret_val))
            }
        }
    }
}
