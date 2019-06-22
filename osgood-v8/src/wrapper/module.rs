use super::{osgood, Exception, Isolate, Local, Valuable, V8};

pub use V8::Module;

enum Status {
    Uninstantiated,
    Instantiating,
    Instantiated,
    Evaluating,
    Evaluated,
    Errored,
}

impl Module {
    pub fn compile(
        src: Local<V8::String>,
        name: Local<V8::String>,
    ) -> Result<Local<V8::Module>, String> {
        let origin = unsafe { osgood::create_module_origin(Isolate::raw(), name.into()) };
        let result = unsafe { osgood::compile_module(Isolate::raw(), origin, src.into()) };
        if result.is_exception {
            let mut exception: Exception = result.exception.into();
            Err(exception.syntax_error_stack())
        } else {
            Ok(result.ret_val.into())
        }
    }

    pub fn empty_and_throw(message: &str) -> V8::MaybeLocal<V8::Module> {
        Isolate::throw_error(message);
        unsafe { osgood::empty_module() }
    }
}

impl Local<Module> {
    pub fn instantiate(
        &mut self,
        ctx: Local<V8::Context>,
        callback: V8::Module_ResolveCallback,
    ) -> Result<(), String> {
        let result = unsafe { osgood::instantiate_module(ctx.into(), (*self).into(), callback) };
        if result {
            Ok(())
        } else {
            Err("Failed to instantiate module".to_string())
        }
    }

    pub fn evaluate(&mut self, ctx: Local<V8::Context>) -> Result<Local<V8::Value>, String> {
        let result = unsafe { osgood::evaluate_module(Isolate::raw(), ctx.into(), (*self).into()) };
        if result.is_exception {
            Err(Local::from(result.ret_val)
                .to_object()
                .get(ctx, "stack")
                .as_rust_string())
        } else {
            Ok(Local::from(result.ret_val))
        }
    }

    pub fn get_hash(&mut self) -> i32 {
        unsafe { self.inner_mut().GetIdentityHash() }
    }

    pub fn get_exports(mut self, context: Local<V8::Context>) -> Result<Local<V8::Object>, String> {
        unsafe {
            let module = self.inner_mut();
            let mut exports: Local<V8::Value> = module.GetModuleNamespace().into();
            let exports = exports.inner_mut();

            if !exports.IsObject() {
                return Err(String::from("Module namespace was not an object"));
            }

            Ok(exports.ToObject(context.into()).to_local_checked().unwrap())
        }
    }
}

impl From<Local<Module>> for V8::MaybeLocal<V8::Module> {
    fn from(wrapped: Local<Module>) -> V8::MaybeLocal<V8::Module> {
        unsafe { osgood::from_local_module(wrapped.into()) }
    }
}
