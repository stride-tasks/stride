use std::path::Path;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

mod cli;
mod dsl;
mod generator;

fn copy_directory(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_directory(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    generator::generate_all_html(&cli.api_dir, &cli.output_dir)?;

    copy_directory(&cli.api_dir, &cli.output_dir.join("api"))?;
    copy_directory(&cli.assets_dir, &cli.output_dir.join("assets"))?;
    Ok(())
}
