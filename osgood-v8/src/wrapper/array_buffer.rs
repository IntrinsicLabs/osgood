use super::*;

pub use V8::ArrayBuffer;

use std::ffi::c_void;
use std::ptr::copy_nonoverlapping;

impl ArrayBuffer {
    pub fn new_from_u8_array(arr: &[u8], len: usize) -> Local<V8::ArrayBuffer> {
        unsafe {
            let mut array_buffer = Local::from(ArrayBuffer::New(Isolate::raw(), len));
            let src = arr.as_ptr() as *mut c_void;
            copy_nonoverlapping(src, array_buffer.inner_mut().GetContents().data_, len);
            array_buffer
        }
    }
}

impl Local<ArrayBuffer> {
    pub fn as_vec_u8(&mut self) -> Vec<u8> {
        unsafe {
            let contents = self.inner_mut().GetContents();
            let arr_ptr = contents.data_ as *mut u8;
            let len = contents.byte_length_;
            let mut dst = Vec::with_capacity(len);
            copy_nonoverlapping(arr_ptr, dst.as_mut_ptr(), len);
            dst.set_len(len);
            dst
        }
    }
}
