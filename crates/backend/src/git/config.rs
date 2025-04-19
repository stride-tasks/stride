use std::path::PathBuf;

use super::{encryption_key::EncryptionKey, ssh_key::SshKey};

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

    pub fn repository_path(&self) -> PathBuf {
        self.root_path.join("source")
    }

    pub fn tasks_path(&self) -> PathBuf {
        self.repository_path().join(Self::TASK_DIR)
    }

    pub fn keys_filepath(&self) -> PathBuf {
        self.tasks_path().join("keys")
    }
}
