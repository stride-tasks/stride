use std::{fs::File, io::Read, path::PathBuf};

use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasmi::{Config, Engine, Module};
use zip::ZipArchive;

use crate::{ErrorKind, RustError};

use super::{error::PluginError, settings::application_support_path};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginVersion {
    V1,
}

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub enum EventHook {
//     TaskCreate,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginManifest {
    pub version: PluginVersion,
    pub name: String,
    // author: String,
    // hooks: Vec<EventHook>,
}

#[derive(Debug)]
pub struct Plugin {
    pub manifest: PluginManifest,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct PluginManager {
    plugins_path: PathBuf,
    plugins: Vec<Plugin>,

    engine: Engine,
}

impl PluginManager {
    pub fn new() -> Result<Self, RustError> {
        let plugins_path = application_support_path().join("plugins");
        if !plugins_path.exists() {
            std::fs::create_dir_all(&plugins_path)?;
        }

        let config = Config::default();
        let engine = Engine::new(&config);

        Ok(Self {
            plugins_path,
            plugins: Vec::new(),

            engine,
        })
    }

    pub fn load(&mut self) -> Result<(), RustError> {
        let entries = self.plugins_path.read_dir()?;

        let mut plugins = Vec::new();
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

            let plugin_path = self.plugins_path.join(uuid.to_string());
            let source_path = plugin_path.join("source");
            let manifest_path = source_path.join("manifest.toml");

            let manifest_content = std::fs::read_to_string(&manifest_path)?;

            let manifest = toml::from_str(&manifest_content)
                .map_err(|error| ErrorKind::from(PluginError::Deserialize(error)))?;

            plugins.push(Plugin { manifest });
        }

        self.plugins = plugins;
        Ok(())
    }

    pub fn list(&self) -> Result<&[Plugin], RustError> {
        Ok(&self.plugins)
    }

    fn validate_wasm_code(&self, wasm: &[u8]) -> Result<(), RustError> {
        let Err(error) = Module::new(&self.engine, wasm) else {
            return Ok(());
        };
        Err(PluginError::InvalidCode(error).into())
    }

    pub fn import(&mut self, plugin_archive_path: String) -> Result<(), RustError> {
        let file = File::open(&plugin_archive_path)?;
        let mut archive = ZipArchive::new(file).map_err(PluginError::from)?;

        let mut manifest = None;
        let mut code_content = None;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(PluginError::from)?;
            let filename = file.name();
            if filename == "manifest.toml" {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                manifest = Some(toml::from_str(&contents).map_err(PluginError::Deserialize)?);
            } else if filename == "code.wasm" {
                let mut contents = Vec::<u8>::new();
                file.read_to_end(&mut contents)?;
                self.validate_wasm_code(&contents)?;
                code_content = Some(contents);
            } else {
                return Err(PluginError::UnknownFile {
                    filename: filename.to_string(),
                }
                .into());
            }
        }

        let Some(manifest) = manifest else {
            return Err(PluginError::MissingManifest.into());
        };

        let Some(_code_content) = code_content else {
            return Err(PluginError::MissingCode.into());
        };

        // TODO: Verify code.wasm

        let plugin = Plugin { manifest };
        self.install(plugin)?;
        Ok(())
    }

    fn install(&mut self, plugin: Plugin) -> Result<(), RustError> {
        let uuid = Uuid::now_v7();
        let plugin_path = self.plugins_path.join(uuid.to_string());
        let source_path = plugin_path.join("source");
        std::fs::create_dir_all(&source_path)?;

        let manifest_path = source_path.join("manifest.toml");
        let manifest_content =
            toml::to_string_pretty(&plugin.manifest).map_err(PluginError::Serialize)?;
        std::fs::write(&manifest_path, manifest_content)?;

        let code_path = source_path.join("code.wasm");
        // TODO: Write plugin contents.
        std::fs::write(&code_path, "")?;

        self.plugins.push(plugin);
        Ok(())
    }
}
