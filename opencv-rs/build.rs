use cmake::Config;

fn main() {
    let cmake_config = Config::new("../opencv")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_PERF_TESTS", "OFF")
        .define("BUILD_DOCS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_opencv_apps", "OFF")
        .define("BUILD_LIST", "core,imgcodecs,imgproc")
        .define("CMAKE_C_COMPILIER", "clang")
        .define("CMAKE_CXX_COMPILIER", "clang++");
    let dst = cmake_config.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}
