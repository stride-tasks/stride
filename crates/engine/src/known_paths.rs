use std::path::PathBuf;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct KnownPaths {
    pub support: PathBuf,
    pub cache: PathBuf,

    pub logs: PathBuf,
    pub ssh_keys: PathBuf,
}

impl KnownPaths {
    const DATABASE_FILE_NAME: &str = "db.sqlite";
    const DESCRIPTOR_FILE_NAME: &str = "descriptor.toml";

    #[must_use]
    pub fn new(support: PathBuf, cache: PathBuf) -> Self {
        Self {
            ssh_keys: support.join(".ssh").join("keys"),
            support,
            logs: cache.join("logs").join("log.txt"),
            cache,
        }
    }

    #[must_use]
    pub fn repository_path(&self, id: Uuid) -> PathBuf {
        self.repositories_path().join(id.to_string())
    }
    #[must_use]
    pub fn repositories_path(&self) -> PathBuf {
        self.support.join("repository")
    }
    #[must_use]
    pub fn backend_path(&self, repository_id: Uuid) -> PathBuf {
        self.repository_path(repository_id).join("backend")
    }
    #[must_use]
    pub fn database_filepath(&self, repository_id: Uuid) -> PathBuf {
        self.repository_path(repository_id)
            .join(Self::DATABASE_FILE_NAME)
    }
    #[must_use]
    pub fn descriptor_filepath(&self, repository_id: Uuid) -> PathBuf {
        self.repository_path(repository_id)
            .join(Self::DESCRIPTOR_FILE_NAME)
    }
}
