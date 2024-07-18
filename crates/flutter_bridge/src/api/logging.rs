use std::{fs::File, io::Write, path::Path};

use chrono::Local;

use super::paths::application_log_path;

struct LogOutput {
    file: File,
}

impl Write for LogOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::stdout().write_all(buf)?;
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()?;
        self.file.flush()
    }
}

pub(crate) fn init_logger(path: &Path) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("could not create logs directory");
    }

    let file = File::options()
        .append(true)
        .create(true)
        .read(true)
        .open(path)
        .expect("Can't open log file");

    let target = Box::new(LogOutput { file });

    let _ = env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}: {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level().as_str(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Trace)
        .target(env_logger::Target::Pipe(target))
        .try_init();
}

/// # Panics
///
/// If the logger file cannot be read.
#[must_use]
pub fn get_logs() -> String {
    let path = application_log_path();
    if !path.exists() {
        return String::new();
    }
    std::fs::read_to_string(path).expect("could not read logs")
}

#[derive(Debug, Clone, Copy)]
pub struct Logger {}

impl Logger {
    pub fn debug(message: &str) {
        log::debug!("{message}");
    }

    pub fn trace(message: &str) {
        log::trace!("{message}");
    }

    pub fn info(message: &str) {
        log::info!("{message}");
    }

    pub fn warn(message: &str) {
        log::warn!("{message}");
    }

    pub fn error(message: &str) {
        log::error!("{message}");
    }
}
