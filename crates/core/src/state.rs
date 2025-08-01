use std::path::PathBuf;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct KnownPaths {
    pub support: PathBuf,
    pub cache: PathBuf,
    pub download: PathBuf,

    pub logs: PathBuf,
    pub ssh_keys: PathBuf,
}

impl KnownPaths {
    #[must_use]
    pub fn repository_path(&self, id: Uuid) -> PathBuf {
        self.support.join("repository").join(id.to_string())
    }
    #[must_use]
    pub fn backend_path(&self, repository_id: Uuid) -> PathBuf {
        self.repository_path(repository_id).join("backend")
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub known_paths: KnownPaths,
}
