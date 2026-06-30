fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/xcv_wrapper.cpp")
        // مسار أكواد LibreOffice اللي سحبناها
        .include("/home/linux/Downloads/KHAWRIZM_OMNI_ARCHIVE/Extracted_Gold/XCV_Calc_Engine")
        .flag_if_supported("-std=c++17")
        .compile("xcv_core");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/xcv_wrapper.cpp");
}
