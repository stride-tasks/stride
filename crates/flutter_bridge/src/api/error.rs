use flutter_rust_bridge::frb;
use stride_backend::Error as BackendError;
use stride_backend_git::{
    Error as GitBackendError,
    error::KeyStoreError,
    known_hosts::{Host, KnownHostsError},
    ssh_key::SshError,
};
use stride_database::Error as DatabaseError;
use stride_plugin_manager::Error as PluginError;

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
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("environment variable error: {0}")]
    VarEnv(#[from] std::env::VarError),
    #[error("known hosts error: {0}")]
    KnownHosts(#[from] KnownHostsError),
    #[error("import error: {0}")]
    Import(#[from] ImportError),
    #[error("export error: {0}")]
    Export(#[from] ExportError),
    #[error("settings error: {0}")]
    Settings(#[from] SettingsError),
    #[error("base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("plugin error: {0}")]
    Plugin(#[from] PluginError),
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),
    #[error("backend error: {0}")]
    Backend(#[from] BackendError),
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
    pub fn as_unknown_host(&self) -> Option<Host> {
        let ErrorKind::Backend(backend) = self.repr.as_ref() else {
            return None;
        };

        let GitBackendError::UnknownHost { host } = &backend.downcast_ref::<GitBackendError>()?
        else {
            return None;
        };

        Some(host.clone())
    }

    #[frb(sync)]
    pub fn is_key_store_verification(&self) -> bool {
        let ErrorKind::Backend(backend) = self.repr.as_ref() else {
            return false;
        };
        let Some(GitBackendError::KeyStore(error)) = &backend.downcast_ref::<GitBackendError>()
        else {
            return false;
        };

        matches!(error, KeyStoreError::Verification)
    }

    #[frb(sync)]
    #[must_use]
    pub fn to_error_string(&self) -> String {
        self.to_string()
    }

    #[frb(sync)]
    #[must_use]
    pub fn is_out_of_fuel_trap_code(&self) -> bool {
        let ErrorKind::Plugin(error) = self.repr.as_ref() else {
            return false;
        };

        error.is_out_of_fuel_trap_code()
    }

    #[frb(sync)]
    #[must_use]
    pub fn plugin_name(&self) -> Option<String> {
        let ErrorKind::Plugin(error) = self.repr.as_ref() else {
            return None;
        };

        error.plugin_name().map(Into::into)
    }
}

impl<T: Into<ErrorKind>> From<T> for RustError {
    fn from(value: T) -> Self {
        Self {
            repr: Box::new(value.into()),
        }
    }
}
