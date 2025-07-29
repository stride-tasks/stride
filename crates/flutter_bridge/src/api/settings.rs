use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};
use uuid::Uuid;

use crate::{RustError, frb_generated::StreamSink};

use super::{
    error::SettingsError,
    filter::{Filter, FilterSelection},
};

use stride_backend_git::ssh_key::SshKey as InnerSshKey;

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
    Ok(InnerSshKey::load_keys(&ssh_key_path())?
        .into_iter()
        .map(|key| SshKey { inner: key })
        .collect())
}

#[frb(opaque)]
#[derive(Debug)]
pub struct SshKey {
    pub(crate) inner: InnerSshKey,
}

impl SshKey {
    pub fn generate() -> Result<Self, RustError> {
        Ok(Self {
            inner: InnerSshKey::generate(&ssh_key_path())?,
        })
    }

    pub fn save(public_key: &str, private_key: &str) -> Result<Self, RustError> {
        Ok(Self {
            inner: InnerSshKey::save(&ssh_key_path(), public_key, private_key)?,
        })
    }

    pub fn update(uuid: Uuid, public_key: &str, private_key: &str) -> Result<Self, RustError> {
        Ok(Self {
            inner: InnerSshKey::update(&ssh_key_path(), uuid, public_key, private_key)?,
        })
    }

    pub fn remove_key(uuid: Uuid) -> Result<(), RustError> {
        InnerSshKey::remove_key(&ssh_key_path(), uuid)?;
        Ok(())
    }

    #[frb(sync, getter)]
    pub fn uuid(&self) -> Uuid {
        self.inner.id
    }

    #[frb(sync, getter)]
    pub fn public_key(&self) -> String {
        self.inner.public_key.to_string()
    }
}

const fn default_theme_mode() -> bool {
    true
}

fn default_repository_name() -> String {
    String::from("unnamed")
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositorySpecification {
    #[serde(default = "Uuid::now_v7")]
    pub uuid: Uuid,

    #[serde(default = "default_repository_name")]
    pub name: String,
}

impl Default for RepositorySpecification {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: default_repository_name(),
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
    pub repositories: Vec<RepositorySpecification>,
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
        // HACK: We need an ~/.ssh directory for libssh2. On android and iOS
        // that means redefining the HOME dir, so that we can control
        // it. <2025-02-25>
        //
        // SAFETY: We do this initially when the applicattion is initialized,
        //         so this should be thread-safe.
        #[cfg(any(target_os = "android", target_os = "ios"))]
        unsafe {
            std::env::set_var("HOME", &paths.support_path);
        }

        let ssh_path = Path::new(&paths.support_path).join(".ssh");

        std::fs::create_dir_all(&ssh_path)?;

        stride_logging::init(Path::new(&paths.log_path));

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
}
