use flutter_rust_bridge::frb;
use thiserror::Error;

use crate::{git::known_hosts::Host, task::TaskStatus};

use stride_crypto::crypter::Error as EncryptionError;

// pub type Result<T> = std::result::Result<T, Error>;

#[frb(ignore)]
#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("invalid encoding: {0}")]
    InvalidEncoding(serde_json::Error),
}

#[frb(ignore)]
#[derive(Debug, Error)]
pub enum KeyStoreError {
    #[error("invalid task status in key store: {identifier}")]
    InvalidTaskStatus { identifier: u8 },

    #[error("incorrect cipher length, expected {expected_length}, actual: {actual_length}")]
    InvalidCipherLength {
        expected_length: usize,
        actual_length: usize,
    },

    #[error("already defined key of type {status:?}")]
    DuplicateEntry { status: TaskStatus },

    #[error("cannot lock key store")]
    LockError,
}

#[frb(ignore)]
#[derive(Debug, Error)]
pub enum ImportError {
    #[error("deserialization error: {0}")]
    Deserialize(serde_json::Error),
}

#[frb(ignore)]
#[derive(Debug, Error)]
pub enum ExportError {
    #[error("serialization error: {0}")]
    Serialize(serde_json::Error),
}

#[frb(ignore)]
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("no ssh keys are provided")]
    NoSshKeysProvided,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("network error: {message}")]
    Network { message: Box<str> },

    #[error("ssh authentication error: {message}")]
    Authentication { message: Box<str> },

    #[error("unknown host error: {host}")]
    UnknownHost { host: Host },

    #[error("{hostname} remote host key is not available")]
    MissingHostKey { hostname: Box<str> },

    #[error("unknown remote key type")]
    UnknownKeyType,

    #[error("mismatched host key: expected {expected}, actual {actual}")]
    MissmatchRemoteKey {
        expected: Box<str>,
        actual: Box<str>,
    },

    #[error("task encoding is corrupted.")]
    CorruptTask,

    #[error("import error: {0}")]
    Import(#[from] ImportError),

    #[error("export error: {0}")]
    Export(#[from] ExportError),

    #[error("other error: {message}")]
    Other { message: Box<str> },

    #[error("libgit2 error: {0}")]
    Git(git2::Error),

    #[error("key store error: {0}")]
    KeyStore(#[from] KeyStoreError),

    #[error("encryption error: {0}")]
    Encryption(#[from] EncryptionError),

    #[error("settings error: {0}")]
    Settings(#[from] SettingsError),

    #[error("base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
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

    #[frb(sync)]
    #[must_use]
    pub fn to_error_string(&self) -> String {
        self.to_string()
    }
}

impl From<git2::Error> for RustError {
    fn from(error: git2::Error) -> Self {
        let kind = match error.class() {
            git2::ErrorClass::Net => ErrorKind::Network {
                message: error.message().into(),
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

impl From<ImportError> for RustError {
    fn from(error: ImportError) -> Self {
        Self {
            repr: Box::new(error.into()),
        }
    }
}

impl From<ExportError> for RustError {
    fn from(error: ExportError) -> Self {
        Self {
            repr: Box::new(error.into()),
        }
    }
}
impl From<EncryptionError> for RustError {
    fn from(error: EncryptionError) -> Self {
        Self {
            repr: Box::from(ErrorKind::Encryption(error)),
        }
    }
}

impl From<KeyStoreError> for RustError {
    fn from(error: KeyStoreError) -> Self {
        Self {
            repr: Box::from(ErrorKind::KeyStore(error)),
        }
    }
}

impl From<SettingsError> for RustError {
    fn from(error: SettingsError) -> Self {
        Self {
            repr: Box::from(ErrorKind::Settings(error)),
        }
    }
}

impl From<base64::DecodeError> for RustError {
    fn from(error: base64::DecodeError) -> Self {
        Self {
            repr: Box::from(ErrorKind::Base64Decode(error)),
        }
    }
}
