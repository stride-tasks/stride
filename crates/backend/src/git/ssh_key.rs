use std::path::{Path, PathBuf};

use stride_core::state::KnownPaths;
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
    pub fn load_keys(known_path: &KnownPaths) -> Result<Vec<SshKey>> {
        let ssh_key_path = &known_path.ssh_keys;
        let Ok(entries) = ssh_key_path.read_dir() else {
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

            let Ok(uuid) = Uuid::try_parse(&name) else {
                continue;
            };

            let key_path = ssh_key_path.join(uuid.to_string());
            let public_path = key_path.join("key.pub");
            let private_path = key_path.join("key");

            let public_key = std::fs::read_to_string(&public_path)
                .unwrap_or_else(|_| panic!("missing public key in {uuid} SSH key"));

            result.push(SshKey {
                uuid,
                public_key,
                public_path,
                private_path,
            });
        }

        Ok(result)
    }

    pub fn load_key(uuid: Uuid, known_path: &KnownPaths) -> Result<SshKey, std::io::Error> {
        let ssh_key_path = &known_path.ssh_keys;

        let key_path = ssh_key_path.join(uuid.to_string());
        let public_path = key_path.join("key.pub");
        let private_path = key_path.join("key");

        let public_key = std::fs::read_to_string(&public_path)?;

        Ok(SshKey {
            uuid,
            public_key,
            public_path,
            private_path,
        })
    }

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

    #[must_use]
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    #[must_use]
    pub fn public_key(&self) -> &str {
        &self.public_key
    }
}
