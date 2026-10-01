//! This is the main entry point for the `stride_api_codegen_cli` crate,
//! which provides a command-line interface for generating code from API definitions.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
    time::SystemTime,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    let force = extract_flag(&mut args, "--force");

    let mut args = args.into_iter();
    let command = args
        .next()
        .unwrap_or_else(|| panic!("usage: stride_api_codegen <generate-rust|generate-dart> --api-dir <dir> [--output <path>] [--force]"));

    match command.as_str() {
        "generate-rust" => {
            let api_dir = next_arg(&mut args, "--api-dir");
            let output = next_arg(&mut args, "--output");
            let stale_files = stale_api_files(&output, &api_dir)?;
            if !force && stale_files.is_empty() {
                return Ok(());
            }
            if force {
                println!("Regenerating: forced");
            } else {
                for file in &stale_files {
                    println!("Regenerating: {}", file.display());
                }
            }
            let generated = stride_api_codegen::generate_rust(&api_dir)?;
            write_output(output, &generated)?;
        }
        "generate-dart" => {
            let api_dir = next_arg(&mut args, "--api-dir");
            let output = next_arg(&mut args, "--output");
            let stale_files = stale_api_files(&output, &api_dir)?;
            if !force && stale_files.is_empty() {
                return Ok(());
            }
            if force {
                println!("Regenerating: forced");
            } else {
                for file in &stale_files {
                    println!("Regenerating: {}", file.display());
                }
            }
            let generated = stride_api_codegen::generate_dart(&api_dir)?;
            write_output(output, &generated)?;
        }
        other => {
            return Err(format!("unknown command: {other}").into());
        }
    }

    Ok(())
}

fn extract_flag(args: &mut Vec<String>, flag: &str) -> bool {
    if let Some(pos) = args.iter().position(|arg| arg == flag) {
        args.remove(pos);
        true
    } else {
        false
    }
}

#[allow(dead_code)]
fn latest_mtime(dirs: &[PathBuf]) -> Result<Option<SystemTime>, Box<dyn std::error::Error>> {
    let mut latest: Option<SystemTime> = None;
    for dir in dirs {
        visit_mtime(dir, &mut latest)?;
    }
    Ok(latest)
}

fn stale_api_files(
    output: &Path,
    api_dir: &Path,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let cutoff = match output.metadata() {
        Ok(metadata) => metadata.modified()?,
        Err(_) => return all_api_files(api_dir),
    };

    // Get the path of the running binary
    let exe_path = env::current_exe()?;

    // Read the file metadata
    let metadata = fs::metadata(exe_path)?;

    // Get the modification time (mtime) as a SystemTime
    let generator_mtime: SystemTime = metadata.modified()?;

    if cutoff < generator_mtime {
        return all_api_files(api_dir);
    }

    let mut stale = Vec::new();
    visit_stale_files(api_dir, cutoff, &mut stale)?;
    stale.sort();
    Ok(stale)
}

fn all_api_files(api_dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    collect_files(api_dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_name() == "target" {
                continue;
            }
            collect_files(&entry.path(), files)?;
        }
        return Ok(());
    }
    files.push(path.to_path_buf());
    Ok(())
}

fn visit_stale_files(
    path: &Path,
    cutoff: SystemTime,
    stale: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_name() == "target" {
                continue;
            }
            visit_stale_files(&entry.path(), cutoff, stale)?;
        }
        return Ok(());
    }

    let mtime = metadata.modified()?;
    if mtime > cutoff {
        stale.push(path.to_path_buf());
    }
    Ok(())
}

#[allow(dead_code)]
fn visit_mtime(
    path: &Path,
    latest: &mut Option<SystemTime>,
) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_name() == "target" {
                continue;
            }
            visit_mtime(&entry.path(), latest)?;
        }
        return Ok(());
    }

    let mtime = metadata.modified()?;
    if latest.is_none_or(|current| mtime > current) {
        *latest = Some(mtime);
    }
    Ok(())
}

fn next_arg(args: &mut impl Iterator<Item = String>, name: &str) -> PathBuf {
    let value = match args.next() {
        Some(value) if value == name => args.next(),
        Some(value) => Some(value),
        None => None,
    };

    let value = value.unwrap_or_else(|| {
        panic!("missing required argument: {name}");
    });

    PathBuf::from(value)
}

fn write_output(output: PathBuf, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, contents)?;
    Ok(())
}
