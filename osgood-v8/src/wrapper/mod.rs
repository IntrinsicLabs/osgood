use super::osgood;
use super::V8;
use std::convert;
use std::env;
use std::os::raw::c_char;

mod local;
pub use local::*;

mod isolate;
pub use isolate::*;

mod handle_scope;
pub use handle_scope::*;

mod functioncallbackinfo;
pub use functioncallbackinfo::*;

mod script;
pub use script::*;

mod module;
pub use module::*;

mod context;
pub use context::*;

mod array;
pub use array::*;

mod object;
pub use object::*;

mod function;
pub use function::*;

mod string;
pub use string::*;

mod number;
pub use number::*;

mod array_buffer;
pub use array_buffer::*;

mod exception;
pub use exception::*;

/// This is a convenience `None`, which can be used by reference as a "null" in arguments to v8
/// functions.
pub const NULL: Option<u16> = None;

pub fn platform_init() {
    let args: Vec<std::string::String> = env::args().collect();
    let name = format!("{}\0", args[0]).as_ptr() as *const c_char;
    unsafe {
        osgood::platform_init(name);
    }
}

pub fn platform_dispose() {
    unsafe {
        osgood::platform_dispose();
    }
}

pub fn process_messages() {
    unsafe {
        osgood::process_messages(Isolate::raw());
    }
}
