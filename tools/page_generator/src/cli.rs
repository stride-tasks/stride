use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(short = 'a', long, default_value = "./api")]
    pub api_dir: PathBuf,

    #[clap(short = 's', long, default_value = "./assets")]
    pub assets_dir: PathBuf,

    #[clap(short = 'o', long, default_value = "./output")]
    pub output_dir: PathBuf,
}
