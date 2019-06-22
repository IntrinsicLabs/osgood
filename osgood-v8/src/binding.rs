#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//pub use self::root::__BindgenBitfieldUnit;
//pub use self::root::std as cppstd;
pub use self::root::v8 as V8;
//pub use self::root::FILE;
pub use self::root::osgood;
