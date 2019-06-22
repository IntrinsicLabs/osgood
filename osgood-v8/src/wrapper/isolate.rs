use super::*;
use std::ptr;

#[derive(Debug, Copy, Clone)]
pub struct Isolate {
    isolate_: *mut V8::Isolate,
}

impl convert::From<Isolate> for *mut V8::Isolate {
    fn from(orig: Isolate) -> *mut V8::Isolate {
        orig.isolate_
    }
}

impl Isolate {
    pub fn new() -> Isolate {
        unsafe {
            let params = V8::Isolate_CreateParams {
                code_event_handler: None,
                constraints: V8::ResourceConstraints {
                    max_semi_space_size_in_kb_: 0,
                    max_old_space_size_: 0,
                    stack_limit_: ptr::null_mut(),
                    code_range_size_: 0,
                    max_zone_pool_size_: 0,
                },
                snapshot_blob: ptr::null_mut(),
                counter_lookup_callback: None,
                create_histogram_callback: None,
                add_histogram_sample_callback: None,
                external_references: ptr::null_mut(),
                only_terminate_in_safe_scope: false,
                allow_atomics_wait: true,
                array_buffer_allocator: V8::ArrayBuffer_Allocator_NewDefaultAllocator(),
            };
            let isolate = V8::Isolate::New(&params);
            V8::Isolate_SetMicrotasksPolicy(isolate, V8::MicrotasksPolicy_kAuto);
            Isolate::from(isolate)
        }
    }

    pub fn raw() -> *mut V8::Isolate {
        unsafe { V8::Isolate_GetCurrent() }
    }

    pub fn from(isolate_: *mut V8::Isolate) -> Isolate {
        Isolate { isolate_ }
    }

    pub fn enter(self) {
        unsafe {
            self.isolate_.as_mut().unwrap().Enter();
        }
    }

    pub fn exit(self) {
        unsafe {
            self.isolate_.as_mut().unwrap().Exit();
        }
    }

    pub fn dispose(self) {
        unsafe {
            self.isolate_.as_mut().unwrap().Dispose();
        }
    }
    pub fn throw_error(error_string: &str) {
        unsafe {
            let error_string = V8::String::new_from_slice(error_string);
            let exception = V8::Exception_Error(error_string.into());
            Isolate::raw().as_mut().unwrap().ThrowException(exception);
        }
    }

    pub fn throw_type_error(error_string: &str) {
        unsafe {
            let error_string = V8::String::new_from_slice(error_string);
            let exception = V8::Exception_TypeError(error_string.into());
            Isolate::raw().as_mut().unwrap().ThrowException(exception);
        }
    }

    pub fn throw_range_error(error_string: &str) {
        unsafe {
            let error_string = V8::String::new_from_slice(error_string);
            let exception = V8::Exception_RangeError(error_string.into());
            Isolate::raw().as_mut().unwrap().ThrowException(exception);
        }
    }

    pub fn null() -> Local<V8::Primitive> {
        unsafe { osgood::null(Isolate::raw()).into() }
    }

    pub fn get_current_context() -> Local<V8::Context> {
        unsafe { Isolate::raw().as_mut().unwrap().GetCurrentContext().into() }
    }
}

impl Default for Isolate {
    fn default() -> Self {
        Self::new()
    }
}
