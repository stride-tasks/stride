use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};
use uuid::Uuid;

use crate::{RustError, frb_generated::StreamSink};

use super::error::SettingsError;

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
