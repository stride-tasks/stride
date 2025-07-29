use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::Result;

#[derive(thiserror::Error, Debug)]
pub enum SshError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("SSH key was not found with ID: {id}")]
    NotFound { id: Uuid },
}

#[derive(Debug, Clone)]
pub struct SshKey {
    pub id: Uuid,
    pub public_key: String,
    pub public_path: PathBuf,
    pub private_path: PathBuf,
}

impl SshKey {
    pub fn load_keys(keys_path: &Path) -> Result<Vec<SshKey>, SshError> {
        let Ok(entries) = keys_path.read_dir() else {
            return Ok(Vec::new());
        };

        let mut result = Vec::new();
        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };

            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };

            let Ok(id) = Uuid::try_parse(&name) else {
                continue;
            };

            let key_path = keys_path.join(id.to_string());
            let public_path = key_path.join("key.pub");
            let private_path = key_path.join("key");

            let public_key = std::fs::read_to_string(&public_path)?;

            result.push(SshKey {
                id,
                public_key,
                public_path,
                private_path,
            });
        }

        Ok(result)
    }

    pub fn load_key(id: Uuid, keys_path: &Path) -> Result<SshKey, SshError> {
        let key_path = keys_path.join(id.to_string());
        if !key_path.exists() {
            return Err(SshError::NotFound { id });
        }

        let public_path = key_path.join("key.pub");
        let private_path = key_path.join("key");

        let public_key = std::fs::read_to_string(&public_path)?;

        Ok(SshKey {
            id,
            public_key,
            public_path,
            private_path,
        })
    }

    pub fn generate(keys_path: &Path) -> Result<Self, SshError> {
        let keys = stride_crypto::ed25519::Ed25519::generate();

        Self::save(keys_path, &keys.public, &keys.private)
    }

    pub fn save(keys_path: &Path, public_key: &str, private_key: &str) -> Result<Self, SshError> {
        Self::update(keys_path, Uuid::now_v7(), public_key, private_key)
    }

    pub fn update(
        keys_path: &Path,
        id: Uuid,
        public_key: &str,
        private_key: &str,
    ) -> Result<Self, SshError> {
        let key_path = keys_path.join(id.to_string());
        if !key_path.exists() {
            std::fs::create_dir_all(&key_path)?;
        }

        let public_path = key_path.join("key.pub");
        let private_path = key_path.join("key");

        std::fs::write(&public_path, public_key)?;
        std::fs::write(&private_path, private_key)?;

        Ok(Self {
            id,
            public_key: public_key.to_string(),
            public_path,
            private_path,
        })
    }

    pub fn remove_key(keys_path: &Path, id: Uuid) -> Result<(), SshError> {
        let key_path = keys_path.join(id.to_string());
        if key_path.exists() {
            std::fs::remove_dir_all(key_path)?;
        }
        Ok(())
    }

    #[must_use]
    pub fn id(&self) -> Uuid {
        self.id
    }

    #[must_use]
    pub fn public_key(&self) -> &str {
        &self.public_key
    }
}
