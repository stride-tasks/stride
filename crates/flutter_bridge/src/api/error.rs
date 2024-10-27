use crate::{
    git::known_hosts::{Host, KnownHostsError},
    task::TaskStatus,
};
use flutter_rust_bridge::frb;
use stride_crypto::crypter::Error as EncryptionError;

// pub type Result<T> = std::result::Result<T, Error>;

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
#[derive(Debug)]
pub enum KeyStoreError {
    InvalidTaskStatus {
        identifier: u8,
    },
    InvalidCipherLength {
        expected_length: usize,
        actual_length: usize,
    },
    DuplicateEntry {
        status: TaskStatus,
    },
    LockError,
    Verification,
    MissingEncryptionKeys,
}

impl std::error::Error for KeyStoreError {}
impl std::fmt::Display for KeyStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTaskStatus { identifier } => {
                write!(f, "invalid task status in key store: {identifier}")
            }
            Self::InvalidCipherLength {
                expected_length,
                actual_length,
            } => writeln!(
                f,
                "incorrect cipher length, expected {expected_length}, actual: {actual_length}"
            ),
            Self::DuplicateEntry { status } => {
                write!(f, "already defined key of type {status:?}")
            }
            Self::LockError => write!(f, "cannot lock key store"),
            Self::Verification => f.write_str("encryption key verification"),
            Self::MissingEncryptionKeys => f.write_str("missing encryption keys"),
        }
    }
}

#[frb(ignore)]
#[derive(Debug)]
pub enum ImportError {
    Deserialize(serde_json::Error),
}

impl std::error::Error for ImportError {}
impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deserialize(error) => {
                write!(f, "deserialization error: {error}")
            }
        }
    }
}

#[frb(ignore)]
#[derive(Debug)]
pub enum ExportError {
    Serialize(serde_json::Error),
}

impl std::error::Error for ExportError {}
impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialize(error) => {
                write!(f, "serialization error: {error}")
            }
        }
    }
}

#[frb(ignore)]
#[derive(Debug)]
pub enum ErrorKind {
    NoSshKeysProvided,
    Io(std::io::Error),
    VarEnv(std::env::VarError),
    Network {
        message: Box<str>,
    },
    Authentication {
        message: Box<str>,
    },
    KnownHosts(KnownHostsError),
    UnknownHost {
        host: Host,
    },
    MissingHostKey {
        hostname: Box<str>,
    },
    UnknownKeyType,
    MissmatchRemoteKey {
        expected: Box<str>,
        actual: Box<str>,
    },
    TaskChampion(taskchampion::Error),
    CorruptTask,
    Import(ImportError),
    Export(ExportError),
    Other {
        message: Box<str>,
    },
    Git(git2::Error),
    KeyStore(KeyStoreError),
    Encryption(EncryptionError),
    Settings(SettingsError),
    Base64Decode(base64::DecodeError),
}

impl std::error::Error for ErrorKind {}
impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSshKeysProvided => write!(f, "no ssh keys are provided"),
            Self::Io(error) => write!(f, "IO error: {error}"),
            Self::Network { message } => write!(f, "network error: {message}"),
            Self::Authentication { message } => write!(f, "ssh authentication error: {message}"),
            Self::KnownHosts(error) => write!(f, "known hosts error: {error}"),
            Self::UnknownHost { host } => write!(f, "unknown host error: {host}"),
            Self::MissingHostKey { hostname } => {
                write!(f, "{hostname} remote host key is not available")
            }
            Self::UnknownKeyType => write!(f, "unknown remote key type"),
            Self::MissmatchRemoteKey { expected, actual } => writeln!(
                f,
                "mismatched host key: expected {expected}, actual {actual}"
            ),
            Self::CorruptTask => write!(f, "task encoding is corrupted."),
            Self::Import(error) => write!(f, "import error: {error}"),
            Self::Export(error) => write!(f, "export error: {error}"),
            Self::Other { message } => write!(f, "other error: {message}"),
            Self::Git(error) => write!(f, "libgit2 error: {error}"),
            Self::KeyStore(error) => write!(f, "key store error: {error}"),
            Self::Encryption(error) => write!(f, "encryption error: {error}"),
            Self::Settings(error) => write!(f, "settings error: {error}"),
            Self::Base64Decode(error) => write!(f, "base64 decode error: {error}"),
            Self::VarEnv(error) => write!(f, "var env error: {error}"),
            Self::TaskChampion(error) => write!(f, "taskchampion error: {error}"),
        }
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
impl From<taskchampion::Error> for ErrorKind {
    fn from(value: taskchampion::Error) -> Self {
        Self::TaskChampion(value)
    }
}
impl From<std::env::VarError> for ErrorKind {
    fn from(error: std::env::VarError) -> Self {
        Self::VarEnv(error)
    }
}
impl From<ImportError> for ErrorKind {
    fn from(error: ImportError) -> Self {
        Self::Import(error)
    }
}
impl From<ExportError> for ErrorKind {
    fn from(error: ExportError) -> Self {
        Self::Export(error)
    }
}
impl From<KeyStoreError> for ErrorKind {
    fn from(error: KeyStoreError) -> Self {
        Self::KeyStore(error)
    }
}
impl From<EncryptionError> for ErrorKind {
    fn from(error: EncryptionError) -> Self {
        Self::Encryption(error)
    }
}
impl From<SettingsError> for ErrorKind {
    fn from(error: SettingsError) -> Self {
        Self::Settings(error)
    }
}
impl From<base64::DecodeError> for ErrorKind {
    fn from(error: base64::DecodeError) -> Self {
        Self::Base64Decode(error)
    }
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
    pub fn as_unknown_host(&self) -> Option<Host> {
        let ErrorKind::UnknownHost { host } = self.repr.as_ref() else {
            return None;
        };

        Some(host.clone())
    }

    #[frb(sync)]
    pub fn is_key_store_verification(&self) -> bool {
        matches!(
            self.repr.as_ref(),
            ErrorKind::KeyStore(KeyStoreError::Verification)
        )
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
impl From<std::env::VarError> for RustError {
    fn from(error: std::env::VarError) -> Self {
        ErrorKind::from(error).into()
    }
}
impl From<taskchampion::Error> for RustError {
    fn from(value: taskchampion::Error) -> Self {
        ErrorKind::from(value).into()
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
