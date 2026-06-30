fn main() {
    cc::Build::new()
        .cpp(true)
        .file("ring0_ai_governor.cpp")
        .flag("-ffreestanding")
        .compile("ring0_ai_governor");

    println!("cargo:rerun-if-changed=ring0_ai_governor.cpp");
}
