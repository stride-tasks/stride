use std::{borrow::Cow, fs::File, io::Write, panic::Location, path::Path};

use chrono::Local;

use super::settings::application_log_path;

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

    std::panic::set_hook(Box::new(|p| {
        let location = p.location().unwrap_or_else(|| Location::caller());
        let mut message = p
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .map(Cow::Borrowed);
        if message.is_none() {
            message = p
                .payload()
                .downcast_ref::<String>()
                .map(String::as_str)
                .map(Cow::Borrowed);
        }
        if message.is_none() {
            message = p
                .payload()
                .downcast_ref::<Box<dyn std::fmt::Display>>()
                .map(ToString::to_string)
                .map(Cow::Owned);
        }

        log::error!(
            "PANIC at {}:{}:{}: {}",
            location.file(),
            location.line(),
            location.column(),
            message.map_or(Cow::Borrowed("<non-formattable>"), |m| Cow::Owned(
                m.replace('\n', "\\n")
            )),
        );
    }));
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
        let message = message.replace('\n', "\\n");
        log::debug!("{message}");
    }

    pub fn trace(message: &str) {
        let message = message.replace('\n', "\\n");
        log::trace!("{message}");
    }

    pub fn info(message: &str) {
        let message = message.replace('\n', "\\n");
        log::info!("{message}");
    }

    pub fn warn(message: &str) {
        let message = message.replace('\n', "\\n");
        log::warn!("{message}");
    }

    pub fn error(message: &str) {
        let message = message.replace('\n', "\\n");
        log::error!("{message}");
    }
}
