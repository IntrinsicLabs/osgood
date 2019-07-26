use super::*;
use std::ffi::CStr;

#[derive(Debug, Copy, Clone)]
pub struct Local<T> {
    local_: V8::Local<T>,
}

impl<T> Local<T> {
    pub fn from(local_: V8::Local<T>) -> Local<T> {
        Local { local_ }
    }

    pub unsafe fn inner_mut(&mut self) -> &mut T {
        self.local_.val_.as_mut().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Persistent<T> {
    persistent_: *mut V8::Persistent<T>,
}

/// This trait is for any types that can be converted to Local<Value>
pub trait IntoValue {
    fn into_value(&self) -> Local<V8::Value>;
}

/// This trait is for V8 types that can be "upcasted" to Value
pub trait Valuable {
    fn as_value(&self) -> Local<V8::Value>;
    fn as_rust_string(&self) -> std::string::String {
        unsafe {
            let mut utf8 = V8::String_Utf8Value::new(Isolate::raw(), self.as_value().into());
            let to_return = CStr::from_ptr(utf8.str_).to_string_lossy().into_owned();
            utf8.destruct();
            to_return
        }
    }
}

impl<T> convert::From<V8::Local<T>> for Local<T> {
    fn from(orig: V8::Local<T>) -> Local<T> {
        Local::from(orig)
    }
}

impl<T> convert::From<Local<T>> for V8::Local<T> {
    fn from(orig: Local<T>) -> V8::Local<T> {
        orig.local_
    }
}

impl<T> V8::MaybeLocal<T> {
    pub fn to_local_checked(&self) -> Option<Local<T>> {
        let val = unsafe { self.val_.as_mut() };
        if let Some(val) = val {
            Some(Local {
                local_: V8::Local {
                    val_: val,
                    _phantom_0: std::marker::PhantomData,
                },
            })
        } else {
            None
        }
    }
}

macro_rules! persistent {
    ($type:ty, $to_persistent:ident, $from_persistent: ident, $reset: ident) => {
        impl convert::From<Local<$type>> for Persistent<$type> {
            fn from(local: Local<$type>) -> Persistent<$type> {
                let inner = local.local_;
                unsafe {
                    Persistent {
                        persistent_: osgood::$to_persistent(Isolate::raw(), inner),
                    }
                }
            }
        }

        impl convert::From<Persistent<$type>> for Local<$type> {
            fn from(persistent: Persistent<$type>) -> Local<$type> {
                let persistent_ptr = persistent.persistent_;
                unsafe { osgood::$from_persistent(Isolate::raw(), persistent_ptr).into() }
            }
        }

        impl convert::From<&Persistent<$type>> for Local<$type> {
            fn from(persistent: &Persistent<$type>) -> Local<$type> {
                let persistent_ptr = persistent.persistent_;
                unsafe { osgood::$from_persistent(Isolate::raw(), persistent_ptr).into() }
            }
        }

        impl Persistent<$type> {
            pub fn reset(&self) {
                let persistent_ptr = self.persistent_;
                unsafe { osgood::$reset(persistent_ptr) }
            }

            pub fn into_local(&self) -> Local<$type> {
                self.into()
            }
        }
    };
}

macro_rules! each_valuable_type {
    ($type:ty) => {
        impl Valuable for Local<$type> {
            fn as_value(&self) -> Local<V8::Value> {
                unsafe { std::mem::transmute::<_, Local<V8::Value>>(*self) }
            }
        }

        impl convert::From<Local<$type>> for Local<V8::Value> {
            fn from(orig: Local<$type>) -> Local<V8::Value> {
                orig.as_value()
            }
        }
    };
}

persistent!(
    V8::Value,
    persistent_from_value,
    persistent_to_value,
    persistent_reset_value
);
persistent!(
    V8::Script,
    persistent_from_script,
    persistent_to_script,
    persistent_reset_script
);
persistent!(
    V8::Object,
    persistent_from_object,
    persistent_to_object,
    persistent_reset_object
);
persistent!(
    V8::Array,
    persistent_from_array,
    persistent_to_array,
    persistent_reset_array
);
persistent!(
    V8::String,
    persistent_from_string,
    persistent_to_string,
    persistent_reset_string
);
persistent!(
    V8::Number,
    persistent_from_number,
    persistent_to_number,
    persistent_reset_number
);
persistent!(
    V8::Integer,
    persistent_from_integer,
    persistent_to_integer,
    persistent_reset_integer
);
persistent!(
    V8::Function,
    persistent_from_function,
    persistent_to_function,
    persistent_reset_function
);
persistent!(
    V8::ArrayBuffer,
    persistent_from_array_buffer,
    persistent_to_array_buffer,
    persistent_reset_array_buffer
);
persistent!(
    V8::Module,
    persistent_from_module,
    persistent_to_module,
    persistent_reset_module
);
persistent!(
    V8::Message,
    persistent_from_message,
    persistent_to_message,
    persistent_reset_message
);
each_valuable_type!(V8::Object);
each_valuable_type!(V8::Map);
each_valuable_type!(V8::Array);
each_valuable_type!(V8::String);
each_valuable_type!(V8::Primitive);
each_valuable_type!(V8::Function);
each_valuable_type!(V8::Number);
each_valuable_type!(V8::Integer);
each_valuable_type!(V8::ArrayBuffer);
each_valuable_type!(V8::Private);

// For V8::Value, this is not done with each_valuable_type! because the From is already implemented
// for identical types
impl Valuable for Local<V8::Value> {
    fn as_value(&self) -> Local<V8::Value> {
        *self
    }
}

macro_rules! downcast {
    ($type:ty, $rust_fn:ident) => {
        pub fn $rust_fn(&self) -> Local<$type> {
            unsafe {
                std::mem::transmute::<Local<V8::Value>, _>(*self)
            }
        }
    }
}

pub enum TypeofTypes {
    String,
    Object,
    Number,
    Symbol,
    Undefined,
    BigInt,
    Function,
    Boolean,
}

impl Local<V8::Value> {
    downcast!(V8::String, to_string);
    downcast!(V8::Function, to_function);
    downcast!(V8::Object, to_object);
    downcast!(V8::Array, to_array);
    downcast!(V8::Number, to_number);
    downcast!(V8::ArrayBuffer, to_array_buffer);

    pub fn as_rust_bool(&mut self, context: Local<V8::Context>) -> bool {
        unsafe {
            let maybe_bool = self.inner_mut().ToBoolean(context.into());
            maybe_bool.to_local_checked().unwrap().inner_mut().Value()
        }
    }

    // TODO: Refactor so that `mut` isn't required here
    #[allow(clippy::wrong_self_convention)]
    pub fn is_boolean(&mut self) -> bool {
        unsafe { self.inner_mut().IsBoolean() }
    }

    pub fn type_of(&mut self) -> TypeofTypes {
        let v8_type_string = unsafe { self.inner_mut().TypeOf(Isolate::raw()) };
        let v8_type_string = Local::from(v8_type_string);
        let type_string = v8_type_string.as_value().as_rust_string();
        let type_str = type_string.as_ref();
        match type_str {
            "string" => TypeofTypes::String,
            "object" => TypeofTypes::Object,
            "number" => TypeofTypes::Number,
            "symbol" => TypeofTypes::Symbol,
            "undefined" => TypeofTypes::Undefined,
            "bigint" => TypeofTypes::BigInt,
            "function" => TypeofTypes::Function,
            "boolean" => TypeofTypes::Boolean,
            x => panic!("Unkown typeof string: {}", x),
        }
    }

    pub fn is_function(mut self) -> bool {
        unsafe { self.inner_mut().IsFunction() }
    }
}

impl<T> IntoValue for Option<T>
where
    T: IntoValue,
{
    fn into_value(&self) -> Local<V8::Value> {
        match self {
            Some(thing) => thing.into_value(),
            None => Isolate::null().as_value(),
        }
    }
}

impl<T> IntoValue for Local<T>
where
    Local<T>: Valuable,
{
    fn into_value(&self) -> Local<V8::Value> {
        self.as_value()
    }
}
