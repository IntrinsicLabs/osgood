use super::{osgood, Context, IntoValue, Isolate, Local, Valuable, V8};

pub use V8::Object;
use V8::Value;

type FunctionCallback = unsafe extern "C" fn(info: *const V8::FunctionCallbackInfo);

impl Object {
    pub fn new() -> Local<V8::Object> {
        unsafe { V8::Object_New(Isolate::raw()).into() }
    }
}

impl Local<Object> {
    pub fn set_extern_method(
        &mut self,
        context: Local<Context>,
        name: &str,
        func: FunctionCallback,
    ) {
        unsafe {
            let v8_name = V8::String::new_from_slice(name).as_value();
            let tmpl = osgood::new_function_template(Isolate::raw(), Some(func));
            let v8_fn = tmpl
                .val_
                .as_mut()
                .unwrap()
                .GetFunction(context.into())
                .to_local_checked()
                .unwrap()
                .into();
            let v8_fn_val =
                std::mem::transmute::<V8::Local<V8::Function>, V8::Local<V8::Value>>(v8_fn);
            self.inner_mut().Set(v8_name.into(), v8_fn_val);
        }
    }

    pub fn set(&mut self, name: &str, val: impl IntoValue) {
        unsafe {
            let key = V8::String::new_from_slice(name);
            self.inner_mut()
                .Set(key.as_value().into(), val.into_value().into());
        }
    }

    // TODO optimize so that they key can be a value
    pub fn get(&mut self, context: Local<Context>, name: &str) -> Local<Value> {
        unsafe {
            let key = V8::String::new_from_slice(name);
            self.inner_mut()
                .Get1(context.into(), key.as_value().into())
                .to_local_checked()
                .unwrap()
        }
    }

    pub fn set_private(&mut self, context: Local<Context>, key_name: &str, val: impl IntoValue) {
        unsafe {
            let priv_key = private(key_name);
            self.inner_mut()
                .SetPrivate(context.into(), priv_key, val.into_value().into());
        }
    }

    pub fn get_private(&mut self, context: Local<Context>, key_name: &str) -> Local<Value> {
        unsafe {
            let priv_key = private(key_name);
            self.inner_mut()
                .GetPrivate(context.into(), priv_key)
                .to_local_checked()
                .unwrap()
        }
    }

    pub fn iter(self, context: Local<V8::Context>) -> ObjectIterator {
        ObjectIterator::new(self, context)
    }
}

pub struct ObjectIterator {
    obj: Local<V8::Object>,
    len: usize,
    cursor: usize,
    names: Local<V8::Object>,
    context: Local<Context>,
}

impl ObjectIterator {
    fn new(mut obj: Local<V8::Object>, context: Local<V8::Context>) -> ObjectIterator {
        let mut names: Local<V8::Array> = unsafe {
            obj.inner_mut()
                .GetOwnPropertyNames(context.into())
                .to_local_checked()
                .unwrap()
        };
        let length = names.length() as usize;

        ObjectIterator {
            obj,
            len: length,
            cursor: 0,
            names: names.as_value().to_object(),
            context,
        }
    }
}

impl std::iter::Iterator for ObjectIterator {
    type Item = (Local<V8::Value>, Local<V8::Value>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.len {
            None
        } else {
            let name = self.names.get(self.context, &self.cursor.to_string());
            let val = unsafe { self.obj.inner_mut().Get(name.into()) };
            self.cursor += 1;
            Some((name, val.into()))
        }
    }
}

unsafe fn private(key_name: &str) -> V8::Local<V8::Private> {
    let isolate = Isolate::raw();
    V8::Private::ForApi(isolate, V8::String::new_from_slice(key_name).into())
}
