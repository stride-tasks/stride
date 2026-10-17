use flutter_rust_bridge::frb;

#[frb(ignore)]
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
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
