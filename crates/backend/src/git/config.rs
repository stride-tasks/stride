use std::path::{Path, PathBuf};

use base64::Engine;
use stride_core::{
    backend::{Config, EncryptionMode, Schema, Value},
    state::KnownPaths,
};

use crate::{Backend, BackendHandler, git::GitBackend};

use super::{encryption_key::EncryptionKey, ssh_key::SshKey};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Handler;

impl BackendHandler for Handler {
    fn name(&self) -> Box<str> {
        "git".into()
    }

    fn config_schema(&self) -> Schema {
        Schema::builder(self.name())
            .field("origin", "Origin", Value::String(None))
            .field("author", "Author", Value::string("Stride"))
            .field(
                "email",
                "E-mail",
                Value::string("noreply.stride.tasks@gmail.com"),
            )
            .field("branch", "Git Branch", Value::String(Some("main".into())))
            .field(
                "encryption_key",
                "Encryption Key",
                Value::Encryption {
                    mode: EncryptionMode::AesOcb256,
                    value: None,
                },
            )
            .field("ssh_key", "SSH Key", Value::SshKey(None))
            .build()
    }

    fn create(
        &self,
        config: &Config,
        path: &Path,
        known_paths: &KnownPaths,
    ) -> crate::Result<Box<dyn Backend>> {
        let config = GitConfig {
            root_path: path.to_path_buf(),
            author: config.string_value("author")?.into(),
            email: config.string_value("email")?.into(),
            branch: config.string_value("branch")?.into(),
            origin: config.string_value("origin")?.into(),
            encryption_key: EncryptionKey {
                key: base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .encode(config.encryption_aes_ocb_256("encryption_key")?),
            },
            ssh_key: {
                let id = config.ssh_key_value("ssh_key")?;
                SshKey::load_key(id, known_paths)?
            },
        };

        Ok(Box::new(GitBackend::new(config)?))
    }
}

#[derive(Debug, Clone)]
pub struct GitConfig {
    pub root_path: PathBuf,

    pub author: Box<str>,
    pub email: Box<str>,
    pub branch: Box<str>,
    pub origin: Box<str>,
    pub encryption_key: EncryptionKey,
    pub ssh_key: SshKey,
}

impl GitConfig {
    const TASK_DIR: &str = "tasks";

    #[must_use]
    pub fn repository_path(&self) -> PathBuf {
        self.root_path.join("source")
    }

    #[must_use]
    pub fn tasks_path(&self) -> PathBuf {
        self.repository_path().join(Self::TASK_DIR)
    }

    #[must_use]
    pub fn keys_filepath(&self) -> PathBuf {
        self.tasks_path().join("keys")
    }
}
