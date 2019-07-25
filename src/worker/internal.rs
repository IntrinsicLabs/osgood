use phf;

use std::cell::RefCell;
use std::collections::HashMap;
use std::{error, fmt};

use crate::osgood_v8::wrapper::*;
use crate::osgood_v8::V8;

thread_local!(static BOOTSTRAP_MAP: RefCell<HashMap<std::string::String, Persistent<V8::Module>>> = RefCell::new(HashMap::new()));

include!(concat!(env!("OUT_DIR"), "/bootstrap.rs"));

#[derive(Debug)]
pub enum InternalModuleErrorKind<'a> {
    InvalidModuleName(&'a str),
    ModuleNotFound(&'a str),
    JSError(std::string::String),
}

#[derive(Debug)]
pub struct InternalModuleError<'a> {
    kind: InternalModuleErrorKind<'a>,
}

impl<'a> InternalModuleError<'a> {
    pub fn invalid_module_name(name: &'a str) -> InternalModuleError {
        InternalModuleError {
            kind: InternalModuleErrorKind::InvalidModuleName(name),
        }
    }

    pub fn module_not_found(name: &'a str) -> InternalModuleError {
        InternalModuleError {
            kind: InternalModuleErrorKind::ModuleNotFound(name),
        }
    }
}

impl<'a> error::Error for InternalModuleError<'a> {}

impl<'a> fmt::Display for InternalModuleError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.kind {
                InternalModuleErrorKind::InvalidModuleName(name) => fmt_worker_error!(
                    "Internal module name must start with 'internal:' (received '{}')",
                    name
                ),
                InternalModuleErrorKind::ModuleNotFound(name) => {
                    fmt_worker_error!("Module '{}' was not found in the bootstrap map", name)
                }
                InternalModuleErrorKind::JSError(err) => fmt_worker_error!("{}", err),
            }
        )
    }
}

impl<'a> From<std::string::String> for InternalModuleError<'a> {
    fn from(str: std::string::String) -> Self {
        InternalModuleError {
            kind: InternalModuleErrorKind::JSError(str),
        }
    }
}

pub fn run_internal_module(
    context: Local<Context>,
    name: &str,
) -> Result<Local<Module>, InternalModuleError> {
    if !name.starts_with("internal:") {
        return Err(InternalModuleError::invalid_module_name(name));
    }
    match BOOTSTRAP_MODULES.get(name.get("internal:".len()..).unwrap()) {
        None => Err(InternalModuleError::module_not_found(name)),
        Some(src) => {
            let src = v8_str!(src);
            let v8_name = v8_str!(name);
            let mut module = Module::compile(*src, *v8_name)?;
            BOOTSTRAP_MAP.with(|map| {
                map.borrow_mut()
                    .insert(std::string::String::from(name), module.into());
            });
            module.instantiate(context, Some(resolve_internal_module))?;
            module.evaluate(context)?;
            Ok(module)
        }
    }
}

extern "C" fn resolve_internal_module(
    context: V8::Local<V8::Context>,
    specifier: V8::Local<V8::String>,
    _referrer: V8::Local<V8::Module>,
) -> V8::MaybeLocal<V8::Module> {
    let specifier = Local::from(specifier).as_rust_string();
    let module = BOOTSTRAP_MAP.with(|map| {
        map.borrow().get(&specifier).and_then(|persistent| {
            let local: Local<V8::Module> = persistent.into();
            Some(local.into())
        })
    });
    match module {
        Some(module) => module,
        None => match run_internal_module(Local::from(context), &specifier) {
            Ok(module) => module.into(),
            Err(err) => {
                println!("{}", err);
                Module::empty_and_throw(&err.to_string())
            }
        },
    }
}
