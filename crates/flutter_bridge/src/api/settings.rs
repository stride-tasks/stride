use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};
use uuid::Uuid;

use crate::{
    api::error::KeyStoreError, base64_decode, frb_generated::StreamSink,
    git::known_hosts::KnownHosts, ErrorKind, RustError,
};

use super::{
    error::SettingsError,
    filter::{Filter, FilterSelection},
};

use super::logging::init_logger;

pub(crate) static APPLICATION_STATE_INSTANCE: LazyLock<Mutex<State>> =
    LazyLock::new(Mutex::default);

pub(crate) static SETTINGS_STREAM_SINK: LazyLock<Mutex<Option<StreamSink<Settings>>>> =
    LazyLock::new(Mutex::default);

#[frb(ignore)]
#[derive(Default)]
pub(crate) struct State {
    paths: ApplicationPaths,
    settings: Settings,
}

#[derive(Debug, Default, Clone)]
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

pub(crate) fn ssh_key_path() -> PathBuf {
    application_support_path().join(".ssh").join("keys")
}

pub fn ssh_keys() -> Result<Vec<SshKey>, RustError> {
    let ssh_key_path = ssh_key_path();
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

        let public_key = std::fs::read_to_string(&public_path)
            .unwrap_or_else(|_| panic!("missing public key in {uuid} SSH key"));

        result.push(SshKey {
            uuid,
            public_key,
            public_path: key_path.join("key.pub"),
            private_path: key_path.join("key"),
        });
    }

    Ok(result)
}

impl KnownHosts {
    pub fn load() -> Result<Self, RustError> {
        KnownHosts::read_standard_file()
    }

    pub fn save(this: &Self) -> Result<(), RustError> {
        KnownHosts::write_standard_file(this)
    }
}

#[frb(opaque)]
#[derive(Debug)]
pub struct SshKey {
    pub(crate) uuid: Uuid,
    pub(crate) public_key: String,
    pub(crate) public_path: PathBuf,
    pub(crate) private_path: PathBuf,
}

impl SshKey {
    pub fn generate() -> Result<Self, RustError> {
        let keys = stride_crypto::ed25519::Ed25519::generate();

        Self::save(&keys.public, &keys.private)
    }

    pub fn save(public_key: &str, private_key: &str) -> Result<Self, RustError> {
        Self::update(Uuid::now_v7(), public_key, private_key)
    }

    pub fn update(uuid: Uuid, public_key: &str, private_key: &str) -> Result<Self, RustError> {
        let key_path = ssh_key_path().join(uuid.to_string());
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

    pub fn remove_key(uuid: Uuid) -> Result<(), RustError> {
        let mut settings = Settings::get();

        for repository in &mut settings.repositories {
            if repository.ssh_key_uuid != Some(uuid) {
                continue;
            }
            repository.ssh_key_uuid = None;
        }

        let key_path = ssh_key_path().join(uuid.to_string());
        if key_path.exists() {
            std::fs::remove_dir_all(key_path)?;
        }
        Settings::save(settings)
    }

    #[frb(sync, getter)]
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    #[frb(sync, getter)]
    pub fn public_key(&self) -> String {
        self.public_key.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionKey {
    pub key: String,
}

impl EncryptionKey {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn generate() -> Self {
        let key = stride_crypto::crypter::Crypter::generate();
        Self {
            key: key.to_base64(),
        }
    }

    pub fn save(repository_uuid: Uuid, key: &str) -> Result<Self, RustError> {
        let decoded = base64_decode(key)?;

        if decoded.len() != 32 {
            return Err(KeyStoreError::InvalidCipherLength {
                expected_length: 32,
                actual_length: decoded.len(),
            }
            .into());
        }

        let mut settings = Settings::get();

        let repository = settings.repository_mut(repository_uuid)?;
        if let Some(encryption_key) = &mut repository.encryption {
            encryption_key.key = key.to_string();
            let result = encryption_key.to_owned();
            Settings::save(settings)?;
            return Ok(result);
        }

        let this = Self {
            key: key.to_string(),
        };

        repository.encryption = Some(this.clone());
        Settings::save(settings)?;
        Ok(this)
    }

    // Store keys on disk instead of memory like ssh keys.
    pub fn remove_key(repository_uuid: Uuid) -> Result<bool, RustError> {
        let mut settings = Settings::get();
        settings.repository_mut(repository_uuid)?.encryption = None;
        Settings::save(settings)?;
        Ok(true)
    }

    #[frb(sync)]
    #[must_use]
    pub fn validate(key: &str) -> Option<String> {
        let Ok(decoded) = base64_decode(key) else {
            return Some("invalid base64".to_owned());
        };

        if decoded.len() != 32 {
            return Some("encryption key must be ${32 * 8} bits".to_owned());
        }
        None
    }
}

const fn default_theme_mode() -> bool {
    true
}

fn default_email() -> String {
    String::from("noreply.stride.tasks@gmail.com")
}

fn default_author() -> String {
    String::from("stride")
}

fn default_branch_name() -> String {
    String::from("main")
}

fn default_repository_name() -> String {
    String::from("unnamed")
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    #[serde(default = "Uuid::now_v7")]
    pub uuid: Uuid,

    #[serde(default = "default_repository_name")]
    pub name: String,

    pub origin: String,

    #[serde(default = "default_author")]
    pub author: String,

    #[serde(default = "default_email")]
    pub email: String,

    #[serde(default = "default_branch_name")]
    pub branch: String,

    pub ssh_key_uuid: Option<Uuid>,
    #[serde(default)]
    pub encryption: Option<EncryptionKey>,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: default_repository_name(),
            origin: String::new(),
            author: default_author(),
            email: default_email(),
            branch: default_branch_name(),
            ssh_key_uuid: None,
            encryption: None,
        }
    }
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_theme_mode")]
    pub dark_mode: bool,

    #[serde(default)]
    pub periodic_sync: bool,

    #[serde(default)]
    pub filters: Vec<Filter>,
    #[serde(default)]
    pub selected_filter: Option<FilterSelection>,

    /// The current selected repository.
    #[serde(default)]
    pub current_repository: Option<Uuid>,

    #[serde(default)]
    pub repositories: Vec<Repository>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dark_mode: default_theme_mode(),
            periodic_sync: false,
            filters: Vec::default(),
            selected_filter: None,
            current_repository: None,
            repositories: Vec::default(),
        }
    }
}

impl Settings {
    #[frb(sync)]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get() -> Self {
        APPLICATION_STATE_INSTANCE.lock().unwrap().settings.clone()
    }
    pub fn load(paths: ApplicationPaths) -> Result<Settings, RustError> {
        std::env::set_var("HOME", &paths.support_path);

        let ssh_path = Path::new(&paths.support_path).join(".ssh");

        std::fs::create_dir_all(&ssh_path)?;

        init_logger(Path::new(&paths.log_path));

        let filepath = Path::new(&paths.support_path).join("settings.json");

        {
            *APPLICATION_STATE_INSTANCE.lock().unwrap() = State {
                paths,
                settings: Settings::default(),
            };
        }

        let settings = if filepath.exists() {
            let contents = std::fs::read_to_string(filepath)?;
            serde_json::from_str(&contents).map_err(SettingsError::InvalidEncoding)?
        } else {
            Settings::default()
        };

        APPLICATION_STATE_INSTANCE.lock().unwrap().settings = settings.clone();
        Ok(settings)
    }
    pub fn save(settings: Settings) -> Result<(), RustError> {
        let filepath = application_support_path().join("settings.json");

        let contents =
            serde_json::to_string_pretty(&settings).map_err(SettingsError::InvalidEncoding)?;
        std::fs::write(filepath, contents)?;

        {
            APPLICATION_STATE_INSTANCE.lock().unwrap().settings = settings.clone();
        }

        let mut stream = SETTINGS_STREAM_SINK.lock().expect("should not fail");
        let Some(stream) = stream.as_mut() else {
            return Ok(());
        };
        stream.add(settings).unwrap();
        Ok(())
    }
    pub fn create_stream(stream_sink: StreamSink<Settings>) {
        let mut stream = SETTINGS_STREAM_SINK.lock().unwrap();
        *stream = Some(stream_sink);
    }

    pub(crate) fn repository(&self, uuid: Uuid) -> Result<&Repository, RustError> {
        Ok(self
            .repositories
            .iter()
            .find(|repository| repository.uuid == uuid)
            .ok_or(ErrorKind::RepositoryNotFound)?)
    }
    pub(crate) fn repository_mut(&mut self, uuid: Uuid) -> Result<&mut Repository, RustError> {
        Ok(self
            .repositories
            .iter_mut()
            .find(|repository| repository.uuid == uuid)
            .ok_or(ErrorKind::RepositoryNotFound)?)
    }
}

pub(crate) fn ssh_key(uuid: &Uuid) -> Option<(PathBuf, PathBuf)> {
    let key_path = ssh_key_path().join(uuid.to_string());
    if !key_path.exists() {
        return None;
    }

    Some((key_path.join("key.pub"), key_path.join("key")))
}
