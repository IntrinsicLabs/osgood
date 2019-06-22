extern crate phf_codegen;

// use phf_codegen;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
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
