use super::{osgood, Local, Valuable, V8};

pub struct FunctionCallbackInfo {
    info_: *const V8::FunctionCallbackInfo,
}

impl FunctionCallbackInfo {
    pub fn new(info_: *const V8::FunctionCallbackInfo) -> FunctionCallbackInfo {
        FunctionCallbackInfo { info_ }
    }

    pub fn length(&self) -> i32 {
        unsafe { self.info_.as_ref().unwrap().length_ }
    }

    pub fn get(&self, i: i32) -> Result<Local<V8::Value>, String> {
        if self.length() == 0 || i < 0 || i > self.length() {
            Err(String::from("OOB"))
        } else {
            Ok(unsafe { osgood::info_get_arg(self.info_, i).into() })
        }
    }

    pub fn set_return_value(&self, ret_val: &impl Valuable) {
        unsafe {
            osgood::info_set_return_value(self.info_, ret_val.as_value().into());
        }
    }
}
