use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::Result;

#[derive(Debug, Clone)]
pub struct SshKey {
    pub uuid: Uuid,
    pub public_key: String,
    pub public_path: PathBuf,
    pub private_path: PathBuf,
}

impl SshKey {
    pub fn generate(keys_path: &Path) -> Result<Self> {
        let keys = stride_crypto::ed25519::Ed25519::generate();

        Self::save(keys_path, &keys.public, &keys.private)
    }

    pub fn save(keys_path: &Path, public_key: &str, private_key: &str) -> Result<Self> {
        Self::update(keys_path, Uuid::now_v7(), public_key, private_key)
    }

    pub fn update(
        keys_path: &Path,
        uuid: Uuid,
        public_key: &str,
        private_key: &str,
    ) -> Result<Self> {
        let key_path = keys_path.join(uuid.to_string());
        if !key_path.exists() {
            std::fs::create_dir_all(&key_path)?;
        }

        let public_path = key_path.join("key.pub");
        let private_path = key_path.join("key");

        std::fs::write(&public_path, public_key)?;
        std::fs::write(&private_path, private_key)?;

        Ok(Self {
            uuid,
            public_key: public_key.to_string(),
            public_path,
            private_path,
        })
    }

    pub fn remove_key(keys_path: &Path, uuid: Uuid) -> Result<()> {
        let key_path = keys_path.join(uuid.to_string());
        if key_path.exists() {
            std::fs::remove_dir_all(key_path)?;
        }
        Ok(())
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }
}
