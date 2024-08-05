use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::git::known_hosts::KnownHosts;

use super::filter::{Filter, FilterSelection};

use super::logging::init_logger;

lazy_static! {
    pub(crate) static ref APPLICATION_STATE_INSTANCE: Mutex<State> = State::default().into();
}

#[frb(ignore)]
#[derive(Default)]
pub(crate) struct State {
    paths: ApplicationPaths,
    settings: Settings,
}

#[derive(Debug, Default)]
pub struct ApplicationPaths {
    pub support_path: String,
    pub document_path: String,
    pub cache_path: String,

    pub log_path: String,
}

#[allow(dead_code)]
pub(crate) fn application_support_path() -> PathBuf {
    PathBuf::from(
        APPLICATION_STATE_INSTANCE
            .lock()
            .unwrap()
            .paths
            .support_path
            .clone(),
    )
}

#[allow(dead_code)]
pub(crate) fn application_document_path() -> PathBuf {
    PathBuf::from(
        APPLICATION_STATE_INSTANCE
            .lock()
            .unwrap()
            .paths
            .document_path
            .clone(),
    )
}

#[allow(dead_code)]
pub(crate) fn application_cache_path() -> PathBuf {
    PathBuf::from(
        APPLICATION_STATE_INSTANCE
            .lock()
            .unwrap()
            .paths
            .cache_path
            .clone(),
    )
}

pub(crate) fn application_log_path() -> PathBuf {
    PathBuf::from(
        APPLICATION_STATE_INSTANCE
            .lock()
            .unwrap()
            .paths
            .log_path
            .clone(),
    )
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
        APPLICATION_STATE_INSTANCE.lock().unwrap().settings.clone()
    }
    pub fn load(paths: ApplicationPaths) -> anyhow::Result<Settings> {
        std::env::set_var("HOME", &paths.support_path);

        let ssh_path = Path::new(&paths.support_path).join(".ssh");

        std::fs::create_dir_all(&ssh_path).unwrap();
        std::fs::write(ssh_path.join("known_hosts"), "\n").unwrap();

        init_logger(Path::new(&paths.log_path));

        let filepath = Path::new(&paths.support_path).join("settings.json");

        if !filepath.exists() {
            return anyhow::Ok(Settings::default());
        }

        let contents = std::fs::read_to_string(filepath)?;
        let settings: Self = serde_json::from_str(&contents)?;

        {
            *APPLICATION_STATE_INSTANCE.lock().unwrap() = State {
                paths,
                settings: settings.clone(),
            };
        }
        anyhow::Ok(settings)
    }
    pub fn save(settings: Settings) -> anyhow::Result<()> {
        let filepath = application_support_path().join("settings.json");

        let contents = serde_json::to_string_pretty(&settings)?;
        std::fs::write(filepath, contents)?;

        {
            APPLICATION_STATE_INSTANCE.lock().unwrap().settings = settings;
        }
        Ok(())
    }

    pub(crate) fn ssh_key(&self, uuid: &Uuid) -> Option<&SshKey> {
        self.keys.iter().find(|key| &key.uuid == uuid)
    }
}
