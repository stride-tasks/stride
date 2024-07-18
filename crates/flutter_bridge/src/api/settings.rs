use std::sync::Mutex;

use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::git::known_hosts::KnownHosts;

use super::{
    filter::{Filter, FilterSelection},
    paths::application_support_path,
};

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
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn generate() -> SshKey {
        let keys = stride_crypto::ed25519::Ed25519::generate();

        SshKey {
            uuid: Uuid::now_v7(),
            private: keys.private,
            public: keys.public,
        }
    }
}

fn default_branch_name() -> String {
    String::from("main")
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    // pub name: String,
    pub origin: String,
    pub author: String,
    pub email: String,
    #[serde(default = "default_branch_name")]
    pub branch: String,

    pub ssh_key_uuid: Option<Uuid>,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            origin: String::new(),
            author: String::new(),
            email: String::new(),
            branch: default_branch_name(),
            ssh_key_uuid: None,
        }
    }
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub dark_mode: bool,
    pub keys: Vec<SshKey>,
    pub known_hosts: KnownHosts,
    pub repository: Repository,

    #[serde(default)]
    pub periodic_sync: bool,

    #[serde(default)]
    pub filters: Vec<Filter>,
    #[serde(default)]
    pub selected_filter: Option<FilterSelection>,
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
