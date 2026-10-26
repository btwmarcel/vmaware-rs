use bindgen_helpers::{Renamer, rename_enum};

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .include("deps")
        .include("src")
        .file("src/wrapper.cpp")
        .std("c++20");

    build.compile("vmaware-bridge");

    let mut renamer = Renamer::new(false);

    rename_enum!(
        renamer,
        "enum_flags" => "Technique",
        case: Pascal,
    );

    renamer.rename_item("VM_enum_flags", "Technique");

    let builder = bindgen_helpers::Builder::default()
        .header("deps/vmaware.hpp")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
        .allowlist_type("VM_enum_flags")
        .allowlist_var("VM_technique_count")
        .rustified_enum("VM_enum_flags")
        .parse_callbacks(Box::new(renamer));

    let bindings = builder
        .generate()
        .expect("bindgen failed to generate bindings");

    let out_dir =
        std::env::var("OUT_DIR").expect("OUT_DIR environment variable is not set by Cargo");
    let out_path = std::path::PathBuf::from(out_dir);

    bindings
        .write_to_file(out_path.join("flags_bindgen.rs"))
        .expect("Failed to write flags_bindgen.rs");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=deps/vmaware.hpp");

    if target_os == "windows" {
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");
    }
}
