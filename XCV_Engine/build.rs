fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/xcv_wrapper.cpp")
        .file("src/writer_wrapper.cpp")
        .file("src/impress_wrapper.cpp")
        .include("src")
        .include("src/libo_core")
        .flag_if_supported("-std=c++17")
        .compile("xcv_core");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/xcv_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/xcv_wrapper.h");
    println!("cargo:rerun-if-changed=src/writer_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/writer_wrapper.h");
    println!("cargo:rerun-if-changed=src/impress_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/impress_wrapper.h");
}
