extern crate phf_codegen;

// use phf_codegen;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let target_dir = env::var("OUT_DIR").unwrap();
    if is_lock_newer_than_binary(target_dir) {
        println!("cargo:rerun-if-changed={}", "js/package-lock.json");
        let child = Command::new("npm")
            .arg("install")
            .current_dir("js")
            .status()
            .unwrap();
        assert!(child.success());
    }

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("bootstrap.rs");
    let mut file = BufWriter::new(fs::File::create(&path).unwrap());

    write!(
        &mut file,
        "#[allow(clippy::all)]\nstatic BOOTSTRAP_MODULES: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();

    let mut map = &mut phf_codegen::Map::<String>::new();

    let bootstrap_dir = Path::new("js/bootstrap");
    assert!(bootstrap_dir.is_dir());

    for entry in fs::read_dir(bootstrap_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // Ignore files without `.js` extension
        match path.extension().and_then(OsStr::to_str) {
            Some("js") => (),
            _ => continue,
        };

        // Ignore files that start with `.`
        if path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap()
            .starts_with('.')
        {
            continue;
        }

        // TODO: Allow subdirs in bootstrap folder
        assert!(!path.is_dir());
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());

        let key = String::from(
            path.strip_prefix(bootstrap_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .clone(),
        );
        map = map.entry(
            key,
            &format!("r#\"{}\"#", &(fs::read_to_string(&path).unwrap())),
        );
    }

    map.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}

// This prevents `cargo build` from always running `npm install`
fn is_lock_newer_than_binary(target_dir: String) -> bool {
    let lock_file = fs::metadata("js/package-lock.json");
    if let Err(_err) = lock_file {
        return true;
    }
    let lock_file = lock_file.unwrap();

    let binary_file = fs::metadata(format!("{}/osgood", target_dir));
    // let binary_file = fs::metadata("target/debug/osgood");
    if let Err(_err) = binary_file {
        return true;
    }
    let binary_file = binary_file.unwrap();

    if let Ok(lock_time) = lock_file.modified() {
        if let Ok(binary_time) = binary_file.modified() {
            return lock_time > binary_time;
        } else {
            return true;
        }
    } else {
        return true;
    }
}
