use std::path::{Path, PathBuf};

mod dsl;
mod generator;

fn main() {
    let api_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| default_api_dir().display().to_string());
    let output_dir = std::env::args()
        .nth(2)
        .unwrap_or_else(|| default_output_dir().display().to_string());

    if let Err(err) = generator::generate_all_html(Path::new(&api_dir), Path::new(&output_dir)) {
        eprintln!("api page generator failed: {err}");
        std::process::exit(1);
    }
}

fn default_api_dir() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    root.join("../../api")
}

fn default_output_dir() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    root.join("output")
}
