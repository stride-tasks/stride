use super::settings::application_log_path;

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
