use flutter_rust_bridge::frb;
use thiserror::Error;

use crate::git::known_hosts::Host;

// pub type Result<T> = std::result::Result<T, Error>;

#[frb(ignore)]
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("no ssh keys are provided")]
    NoSshKeysProvided,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("network error: {message}")]
    Network { message: String },

    #[error("ssh authentication error: {message}")]
    Authentication { message: String },

    #[error("unknown host error: {host}")]
    UnknownHost { host: Host },

    #[error("{hostname} remote host key is not available")]
    MissingHostKey { hostname: String },

    #[error("unknown remote key type")]
    UnknownKeyType,

    #[error("mismatched host key: expected {expected}, actual {actual}")]
    MissmatchRemoteKey { expected: String, actual: String },

    #[error("task encoding is corrupted.")]
    CorruptTask,

    #[error("other error: {message}")]
    Other { message: String },

    #[error("libgit2 error: {0}")]
    Git(git2::Error),
}

#[frb(opaque)]
#[derive(Debug, Error)]
#[error("{repr}")]
// #[frb(opaque)]
pub struct RustError {
    /// Reduce the size of Error because it's passed around in a lot of functions.
    #[from]
    repr: Box<ErrorKind>,
}

impl RustError {
    #[frb(sync)]
    #[must_use]
    pub fn as_unknown_host(&self) -> Option<Host> {
        let ErrorKind::UnknownHost { host } = self.repr.as_ref() else {
            return None;
        };

        Some(host.clone())
    }
}

impl From<git2::Error> for RustError {
    fn from(error: git2::Error) -> Self {
        let kind = match error.class() {
            git2::ErrorClass::Net => ErrorKind::Network {
                message: error.message().to_string(),
            },
            _ => ErrorKind::Git(error),
        };

        Self {
            repr: Box::from(kind),
        }
    }
}

impl From<std::io::Error> for RustError {
    fn from(error: std::io::Error) -> Self {
        Self {
            repr: Box::from(ErrorKind::Io(error)),
        }
    }
}

impl From<ErrorKind> for RustError {
    fn from(error: ErrorKind) -> Self {
        Self {
            repr: Box::from(error),
        }
    }
}
