fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/ffi/xcv_wrapper.cpp")
        .include("src/ffi")
        .flag_if_supported("-std=c++17")
        .compile("xcv_core");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ffi/xcv_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/ffi/xcv_wrapper.h");
}
