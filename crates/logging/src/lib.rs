//! Logging crate used in the `stride`.

use std::{borrow::Cow, fmt::Write as _, fs::File, io::Write, panic::Location, path::Path};

use chrono::Local;

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

/// Init logger.
///
/// # Panics
///
/// - If can't create directory part of the path.
/// - If can't open file for read and write.
pub fn init(path: &Path) {
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
            let mut result = String::new();
            writeln!(
                &mut result,
                "{} {}: {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level().as_str(),
                record.args()
            )
            .map_err(std::io::Error::other)?;
            let result = result.replace('\n', "\\n");
            writeln!(buf, "{result}")
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
