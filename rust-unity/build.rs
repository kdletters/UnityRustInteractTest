const TARGET_PATH: &'static str = r#"..\RustWrapper\RustWrapper"#;

fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("rust_unity")
        .csharp_namespace("RustLib")
        .csharp_class_name("Lib")
        .csharp_class_accessibility("public")
        .csharp_use_function_pointer(false)
        .generate_csharp_file(format!("{TARGET_PATH}/RustLib.g.cs"))
        .unwrap();

    // #[cfg(debug_assertions)]
    // {
    //     std::fs::copy("target/debug/rust_unity.dll", format!("{TARGET_PATH}/rust_unity.dll")).expect("Failed to copy rust_unity.dll");
    //     std::fs::copy("target/debug/rust_unity.pdb", format!("{TARGET_PATH}/rust_unity.pdb")).expect("Failed to copy rust_unity.pdb");
    // }
    // #[cfg(not(debug_assertions))]
    // {
    //     std::fs::copy("target/release/rust_unity.dll", format!("{TARGET_PATH}/rust_unity.dll")).expect("Failed to copy rust_unity.dll");
    //     std::fs::copy("target/release/rust_unity.pdb", format!("{TARGET_PATH}/rust_unity.pdb")).expect("Failed to copy rust_unity.pdb");
    // }
}