fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/ffi/xcv_wrapper.cpp")
        .file("src/ffi/network_bypass/dpdk_joyride.cpp")
        .include("src/ffi")
        .include("src/ffi/network_bypass")
        .flag_if_supported("-std=c++17")
        .compile("xcv_core");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ffi/xcv_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/ffi/network_bypass/dpdk_joyride.cpp");
}
