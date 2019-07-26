use super::*;

pub use V8::Private;

impl Private {
    pub fn for_api(name: &str) -> Local<Private> {
        unsafe {
            V8::Private_ForApi(Isolate::raw(), V8::String::new_from_slice(name).into()).into()
        }
    }
}
