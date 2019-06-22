#[macro_use]
extern crate osgood_v8;
use osgood_v8::wrapper::*;

#[osgood_v8_macros::v8_fn]
fn wrapped_function(context: FunctionCallbackInfo) {}

#[osgood_v8_macros::v8_fn]
pub fn pub_wrapped_function(context: FunctionCallbackInfo) {}
