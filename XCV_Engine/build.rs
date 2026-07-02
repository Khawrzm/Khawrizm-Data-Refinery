fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/ffi/xcv_wrapper.cpp")
        .file("src/ffi/network_bypass/dpdk_joyride.cpp")
        .file("src/ffi/isac_radar/taco_spatial.cpp")
        .file("src/ffi/taco_core/tabular_locality.cpp")
        .include("src/ffi")
        .include("src/ffi/network_bypass")
        .include("src/ffi/isac_radar")
        .include("src/ffi/taco_core")
        .flag_if_supported("-std=c++17")
        .compile("xcv_core");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ffi/xcv_wrapper.cpp");
}
