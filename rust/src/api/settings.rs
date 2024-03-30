use std::sync::Mutex;

use base64::Engine;
use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use openssl::{pkey::PKey, rsa::Rsa};
use pem::{encode, Pem};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::git::known_hosts::{Host, KnownHosts};

use super::paths::application_support_path;

/*

#[derive(Debug, Serialize, Deserialize)]
pub struct SshKey {
    pub uuid: Uuid,
    pub note: String,

    pub public: String,
    pub private: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capsule {
    pub name: String,

    pub origin: String,
    pub branch: String,
    pub author: String,
    pub email: String,

    pub ssh_id: Uuid,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub keys: Vec<SshKey>,
    pub known_hosts: Vec<Host>,
    pub capsules: Vec<Capsule>,
}

impl ApplicationSettings {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

*/

lazy_static! {
    pub(crate) static ref SETTINGS_INSTANCE: Mutex<Settings> = Settings::default().into();
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKey {
    pub uuid: Uuid,
    pub public: String,
    pub private: String,
}

impl SshKey {
    const SSH_ED25519: &'static str = "ssh-ed25519";

    pub fn generate() -> SshKey {
        let rsa = PKey::generate_ed25519().unwrap();

        let public_key = rsa.raw_public_key().unwrap();
        let private_key = rsa.private_key_to_der().unwrap();

        let private_pem = Pem::new("PRIVATE KEY", private_key);
        let private = encode(&private_pem);

        let mut data = Vec::new();
        data.extend((Self::SSH_ED25519.len() as u32).to_be_bytes());
        data.extend(Self::SSH_ED25519.as_bytes());

        data.extend((public_key.len() as u32).to_be_bytes());
        data.extend(public_key);

        // https://coolaj86.com/articles/the-openssh-private-key-format/

        let public = base64::engine::general_purpose::STANDARD.encode(data);

        SshKey {
            uuid: Uuid::new_v4(),
            private,
            public: format!("{} {public}", Self::SSH_ED25519),
        }
    }
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Repository {
    // pub name: String,
    // pub branch: String,
    pub origin: String,
    pub author: String,
    pub email: String,

    pub ssh_key_uuid: Option<Uuid>,
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub keys: Vec<SshKey>,
    pub known_hosts: KnownHosts,
    pub repository: Repository,
}

impl Settings {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Settings {
    pub(crate) fn get() -> Self {
        SETTINGS_INSTANCE.lock().unwrap().clone()
    }
    pub fn load() -> anyhow::Result<Settings> {
        let filepath = application_support_path().join("settings.json");

        if !filepath.exists() {
            return anyhow::Ok(Settings::default());
        }

        let contents = std::fs::read_to_string(filepath)?;
        let settings: Self = serde_json::from_str(&contents)?;

        {
            *SETTINGS_INSTANCE.lock().unwrap() = settings.clone();
        }
        anyhow::Ok(settings)
    }
    pub fn save(settings: Settings) -> anyhow::Result<()> {
        let filepath = application_support_path().join("settings.json");

        let contents = serde_json::to_string_pretty(&settings)?;
        std::fs::write(filepath, contents)?;

        {
            *SETTINGS_INSTANCE.lock().unwrap() = settings;
        }
        Ok(())
    }

    pub(crate) fn ssh_key(&self, uuid: &Uuid) -> Option<&SshKey> {
        self.keys.iter().find(|key| &key.uuid == uuid)
    }
}