use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let protocol_dir = manifest_dir.join("../../protocol").canonicalize().unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let output_path = out_dir.join("stride_schema.rs");

    println!("cargo:rerun-if-changed={}", protocol_dir.display());

    let generated = stride_api_codegen::generate_rust(&protocol_dir)
        .expect("stride_api_codegen should be able to generate protocol bindings");
    fs::write(&output_path, generated).expect("failed to write generated schema bindings");
}
