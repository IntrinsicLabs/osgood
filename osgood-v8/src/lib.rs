#![deny(clippy::all)]
#![allow(dead_code)] // comment this out to see what's defined but not used yet

mod binding;
pub use binding::osgood;
pub use binding::V8;

pub mod wrapper;

/// Creates a Local<String> from any `format!`-able variable
#[macro_export]
macro_rules! v8_str {
    ( $val:expr ) => {
        // TODO should this really be a reference??
        &$crate::wrapper::String::new_from_slice($val)
    };
}

#[macro_export]
macro_rules! isolate_scope {
    ( $isolate:expr, $code:block ) => {
        $isolate.enter();
        $code
        $isolate.exit();
        $isolate.dispose();
    }
}

#[macro_export]
macro_rules! handle_scope {
    ( $code:block ) => {
        let mut scope = $crate::wrapper::HandleScope::new();
        $code
        drop(scope);
    }
}

#[macro_export]
macro_rules! context_scope {
    ( $context:expr, $code:block ) => {
        $context.enter();
        $code
        $context.exit();
    }
}

#[macro_export]
macro_rules! isolate_and_handle_scope {
    ( $isolate:expr, $code:block ) => {
        isolate_scope!($isolate, {
            handle_scope!($code);
        });
    };
}

#[macro_export]
macro_rules! v8fn {
    ($name:ident, $code:expr) => {
        extern "C" fn $name(args: *const $crate::V8::FunctionCallbackInfo) {
            let args = $crate::wrapper::FunctionCallbackInfo::new(args);
            handle_scope!({
                $code(args);
            });
        }
    };
    (pub $name:ident, $code:expr) => {
        pub extern "C" fn $name(args: *const $crate::V8::FunctionCallbackInfo) {
            let args = $crate::wrapper::FunctionCallbackInfo::new(args);
            handle_scope!({
                $code(args);
            });
        }
    };
}

#[macro_export]
macro_rules! v8_simple_init {
    ($code:expr) => {
        let isolate = $crate::wrapper::Isolate::new();
        isolate_and_handle_scope!(isolate, {
            let mut context = $crate::wrapper::Context::new();
            context_scope!(context, {
                $code(context);
            });
        });
    };
}

#[macro_export]
macro_rules! v8_spawn {
    ($code:expr) => {
        std::thread::spawn(move || {
            let isolate = $crate::wrapper::Isolate::new();
            isolate.enter();
            let scope = $crate::wrapper::HandleScope::new();
            let mut context = $crate::wrapper::Context::new();
            context.enter();
            $code(context);
            context.exit();
            drop(scope);
            isolate.exit();
            isolate.dispose();
        });
    };
}

#[macro_export]
macro_rules! v8_args {
    ($($item:expr),+) => { {
        let args: Vec<&$crate::wrapper::IntoValue> = vec![$($item),+];
        args
    } }
}
