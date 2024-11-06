fn main() {
    let target_path = "../../Assets/RustLib";
    csbindgen::Builder::default()
        .input_extern_file("../src/lib.rs")
        .input_extern_file("../src/core.rs")
        .input_extern_file("../src/minecraft.rs")
        .csharp_dll_name("rust_unity")
        .csharp_namespace("RustLib")
        .csharp_class_name("Lib")
        .csharp_class_accessibility("public")
        .csharp_use_function_pointer(false)
        .generate_csharp_file(format!("{target_path}/RustLib.g.cs"))
        .unwrap();
    
    #[cfg(not(debug_assertions))]
    {
        let target_path = format!("{target_path}/dlls/rust_unity");
        std::fs::create_dir_all(&target_path).unwrap();
        std::fs::copy("../target/release/rust_unity.dll", format!("{}.dll", &target_path)).expect("Failed to copy rust_unity.dll");
        std::fs::copy("../target/release/rust_unity.pdb", format!("{}.pdb", &target_path)).expect("Failed to copy rust_unity.pdb");
    }
}