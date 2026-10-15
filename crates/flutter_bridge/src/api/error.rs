use flutter_rust_bridge::frb;
use stride_backend_git::ssh_key::SshError;

#[frb(ignore)]
#[derive(Debug)]
pub enum SettingsError {
    InvalidEncoding(serde_json::Error),
}

impl std::error::Error for SettingsError {}
impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEncoding(error) => write!(f, "invalid encoding: {error}"),
        }
    }
}

#[frb(ignore)]
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("settings error: {0}")]
    Settings(#[from] SettingsError),
    #[error("ssh error: {0}")]
    Ssh(#[from] SshError),
    #[error("other error: {message}")]
    Other { message: Box<str> },
}

#[frb(opaque)]
#[derive(Debug)]
pub struct RustError {
    /// Reduce the size of Error because it's passed around in a lot of functions.
    repr: Box<ErrorKind>,
}

impl std::error::Error for RustError {}
impl std::fmt::Display for RustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.repr.fmt(f)
    }
}

impl RustError {
    #[frb(sync)]
    #[must_use]
    pub fn to_error_string(&self) -> String {
        self.to_string()
    }
}

impl<T: Into<ErrorKind>> From<T> for RustError {
    fn from(value: T) -> Self {
        Self {
            repr: Box::new(value.into()),
        }
    }
}
