use std::{env, io};
use std::path::PathBuf;

fn main() -> Result<(), io::Error> { // TODO: use jni-sys for jni.h
    let sys_root_path = PathBuf::from("/opt/android/ndk/26.0.10792818/toolchains/llvm/prebuilt/linux-x86_64/sysroot");
    let include_dir_path = sys_root_path.join("usr").join("include")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()?;

    let shared_lib_path = sys_root_path.join("usr").join("lib").join("arm-linux-androideabi")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()?;

    let headers_path_str = "wrapper.h";

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", shared_lib_path.to_str().expect("Invalid path"));

    // Tell cargo to tell rustc to link our library.
    // println!("cargo:rustc-link-lib=log");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        .clang_arg(format!("-I{}", include_dir_path.to_str().expect("Invalid path")))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
    bindings
        .write_to_file(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src").join("bindings.rs"))
        .expect("Couldn't write bindings!");
    Ok(())
}