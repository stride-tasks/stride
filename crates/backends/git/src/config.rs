use std::path::{Path, PathBuf};

use stride_backend::{Backend, BackendHandler};
use stride_core::{
    backend::{Config, Schema, SchemaValue},
    state::KnownPaths,
};

use crate::{Error, GitBackend};

use super::ssh_key::SshKey;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Handler;

impl BackendHandler for Handler {
    fn name(&self) -> Box<str> {
        "git".into()
    }

    fn config_schema(&self) -> Schema {
        Schema::builder(self.name())
            .field(
                "origin",
                "Origin",
                SchemaValue::String { default: None },
                false,
            )
            .field(
                "author",
                "Author",
                SchemaValue::String {
                    default: Some("Stride".into()),
                },
                false,
            )
            .field(
                "email",
                "E-mail",
                SchemaValue::String {
                    default: Some("noreply.stride.tasks@gmail.com".into()),
                },
                false,
            )
            .field(
                "branch",
                "Git Branch",
                SchemaValue::String {
                    default: Some("main".into()),
                },
                false,
            )
            .field(
                "encryption_key",
                "Encryption Key",
                SchemaValue::Bytes {
                    default: None,
                    min: Some(32),
                    max: Some(32),
                    category: Some(stride_core::backend::BytesCategory::Password),
                    generator: Some(stride_core::backend::BytesGenerator::CryptoRandom),
                },
                true,
            )
            .field(
                "ssh_key",
                "SSH Key",
                SchemaValue::SshKey { default: None },
                false,
            )
            .build()
    }

    fn create(
        &self,
        config: &Config,
        path: &Path,
        known_paths: &KnownPaths,
    ) -> stride_backend::Result<Box<dyn Backend>> {
        let schema = self.config_schema();
        let config = config.align(&schema)?.fill(&schema)?;

        let config = GitConfig {
            root_path: path.to_path_buf(),
            author: config.string_value("author")?.into(),
            email: config.string_value("email")?.into(),
            branch: config.string_value("branch")?.into(),
            origin: config.string_value("origin")?.into(),
            encryption_key: config.bytes_value("encryption_key")?.into(),
            ssh_key: {
                let id = config.uuid_value("ssh_key")?;
                SshKey::load_key(id, &known_paths.ssh_keys).map_err(Error::from)?
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
    pub encryption_key: Box<[u8]>,
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
