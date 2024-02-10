use std::{env, io};
use std::path::PathBuf;

fn main() -> Result<(), io::Error> { // TODO: use jni-sys for jni.h
    if cfg!(target_family = "windows") {
        eprintln!("=== WARNING: Building for Windows is not supported, clang will likely fail.");
    }
    // TODO: Better android path detection
    let android_home = env::var("ANDROID_HOME");

    let android_path = PathBuf::from(android_home.unwrap_or_else(|_| "/opt/android/".to_string()));

    if !android_path.exists() {
        panic!("Android SDK not found at {:?}. Please configure the ANDROID_HOME env variable", android_path);
    }

    println!("=== Android SDK found at {:?}", android_path);

    let ndk_path = PathBuf::from(android_path.join("ndk"));

    if !ndk_path.exists() {
        panic!("Android NDK not found at {:?}. Please install it", ndk_path);
    }

    println!("=== Android NDK found at {:?}", ndk_path);

    // Find the latest installed NDK version
    let ndk_dir = ndk_path.read_dir()?;

    let mut ndk_version_option: Option<String> = None;
    for entry in ndk_dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.file_name().unwrap().to_str().unwrap().starts_with("26.0.") {
            let version = path.file_name().unwrap().to_str().unwrap().to_string();
            if let Some(ndk_version) = ndk_version_option.clone() {
                if version > ndk_version {
                    ndk_version_option = Some(version);
                }
            } else {
                ndk_version_option = Some(version);
            }
        }
    }
    let ndk_version_path = ndk_path.join(ndk_version_option.expect("No installed NDK version found in /ndk/"));
    println!("=== Android NDK version found at {:?}", ndk_version_path);
    let prebuilt_path = ndk_version_path.join("toolchains/llvm/prebuilt/");
    let android_sysroot_host = env::var("ANDROID_SYSROOT_HOST").ok();
    let host_triple_path = prebuilt_path.join(android_sysroot_host.unwrap_or("linux-x86_64".to_string())); // TODO: Detect host triple
    let sys_root_path = host_triple_path.join("sysroot");
    if !sys_root_path.exists() {
        panic!("Android NDK sysroot not found at {:?}.", sys_root_path);
    }
    println!("=== Android NDK sysroot found at {:?}", sys_root_path);
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