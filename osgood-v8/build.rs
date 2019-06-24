use git2::{ErrorCode, Repository};

use bindgen;
use cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let v8_dir = match env::var("CUSTOM_V8") {
        Ok(custom_v8_dir) => {
            let custom_v8_dir = PathBuf::from(custom_v8_dir);
            assert!(custom_v8_dir.exists());
            custom_v8_dir
        }
        Err(_) => build_v8(),
    };

    compile_wrappers(v8_dir.clone());
    generate_bindings(v8_dir);
}

fn build_v8() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let v8_repo_path = out_dir.clone().join("third_party/v8");

    if v8_repo_path.exists() {
        return v8_repo_path;
    }

    // Fetch the v8 source
    fetch_v8(out_dir.clone());

    // Checkout the correct v8 version
    let status = Command::new("git")
        .args(&["checkout", include_str!("v8-version.txt").trim()])
        .current_dir(v8_repo_path.clone())
        .status()
        .expect("Failed to checkout correct v8 version");
    assert!(status.success());

    // Update third-party repos and run pre-compile hooks
    let status = Command::new("gclient")
        .arg("sync")
        .current_dir(v8_repo_path.clone())
        .status()
        .expect("Failed to synchronize gclient deps");
    assert!(status.success());

    // Build v8
    generate_config(out_dir.clone());
    run_ninja(out_dir.clone());

    v8_repo_path
}

fn compile_wrappers(v8_dir: PathBuf) {
    let include_dir = v8_dir.join("include");

    println!("cargo:rerun-if-changed=src/wrapper.cpp");

    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("--std=c++14")
        .include(include_dir)
        .file("src/wrapper.cpp")
        .compile("libwrapper.a");
}

fn generate_bindings(v8_dir: PathBuf) {
    println!("cargo:rustc-link-lib=v8_libbase");
    println!("cargo:rustc-link-lib=v8_libplatform");
    println!("cargo:rustc-link-lib=v8_monolith");
    println!("cargo:rustc-link-lib=c++");
    println!(
        "cargo:rustc-link-search={}/out.gn/x64.release/obj",
        v8_dir.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search={}/out.gn/x64.release/obj/third_party/icu",
        v8_dir.to_str().unwrap()
    );

    let bindings = bindgen::Builder::default()
        .generate_comments(true)
        .header("src/wrapper.cpp")
        .rust_target(bindgen::RustTarget::Nightly)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("--std=c++14")
        .clang_arg(format!("-I{}", v8_dir.join("include").to_str().unwrap()))
        // Because there are some layout problems with these
        .opaque_type("std::.*")
        .whitelist_type("std::unique_ptr\\<v8::Platform\\>")
        .whitelist_type("v8::.*")
        .blacklist_type("std::basic_string.*")
        .whitelist_function("v8::.*")
        .whitelist_function("osgood::.*")
        .whitelist_var("v8::.*")
        // Re-structure the modules a bit and hide the "root" module
        .raw_line("#[doc(hidden)]")
        // .generate_inline_functions(true)
        .enable_cxx_namespaces()
        .derive_debug(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .rustfmt_bindings(true) // comment this for a slightly faster build
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn fetch_v8(out_dir: PathBuf) {
    let v8_repo_path = out_dir.join("third_party/v8");

    if !v8_repo_path.exists() {
        let res = Command::new("fetch")
            .arg("v8")
            .current_dir(out_dir.join("third_party"))
            .status();
        let status = match res {
            Ok(val) => val,
            Err(_) => {
                download_depot_tools(out_dir.clone());
                Command::new("fetch")
                    .arg("v8")
                    .current_dir(out_dir.join("third_party"))
                    .status()
                    .unwrap()
            }
        };
        assert!(status.success());
    }
}

fn download_depot_tools(out_dir: PathBuf) {
    let depot_tools_repo_url = "https://chromium.googlesource.com/chromium/tools/depot_tools.git";
    let depot_tools_repo_path = out_dir.join("third_party/depot_tools");

    // Clone the depot_tools repo
    match Repository::clone(depot_tools_repo_url, depot_tools_repo_path.clone()) {
        Ok(_) => (),
        Err(ref e) if e.code() == ErrorCode::Exists => (),
        Err(e) => panic!("Failed to clone depot tools: {}", e),
    };

    // Set the path
    if let Some(path) = env::var_os("PATH") {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(depot_tools_repo_path.clone());
        let new_path = env::join_paths(paths).unwrap();
        env::set_var("PATH", &new_path);
    }
}

fn generate_config(out_dir: PathBuf) {
    let v8_repo_path = out_dir.join("third_party/v8");

    let status = Command::new("tools/dev/v8gen.py")
        .args(&[
            "x64.release",
            "--",
            "v8_monolithic=true",
            "v8_use_external_startup_data=false",
            "use_custom_libcxx=false",
        ])
        .current_dir(v8_repo_path)
        .status()
        .expect("Failed to generate v8 build configuration");
    assert!(status.success());
}

fn run_ninja(out_dir: PathBuf) {
    let ninja_config_path = out_dir.join("third_party/v8/out.gn/x64.release");

    let status = Command::new("ninja")
        .current_dir(ninja_config_path)
        .status()
        .expect("Failed to compile v8");

    assert!(status.success());
}
