use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
};

use stride_core::event::{HostEvent, PluginEvent};
use wasmi::{Config, Engine};

use crate::{
    Error, EventQueue, Plugin, Result,
    manifest::{PluginAction, PluginManifest, PluginState},
};

/// flutter_rust_bridge:unignore
#[derive(Debug, Default)]
pub struct PluginManager {
    pub(crate) plugins_path: PathBuf,
    pub(crate) plugins: HashMap<String, Plugin>,

    pub(crate) engine: Engine,

    pub(crate) host_events: VecDeque<(String, HostEvent)>,
    pub(crate) plugin_events: VecDeque<EventQueue>,
}

impl PluginManager {
    pub fn new(plugins_path: &Path) -> Result<Self> {
        if !plugins_path.exists() {
            std::fs::create_dir_all(plugins_path)?;
        }

        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);

        Ok(Self {
            plugins_path: plugins_path.to_path_buf(),
            plugins: HashMap::new(),

            engine,
            host_events: VecDeque::new(),
            plugin_events: VecDeque::new(),
        })
    }

    pub fn load(&mut self) -> Result<()> {
        let entries = self.plugins_path.read_dir()?;

        let mut plugins = HashMap::new();
        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };

            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };

            let plugin_path = self.plugins_path.join(name);
            let source_path = plugin_path.join("source");
            let manifest_path = source_path.join("manifest.toml");

            let manifest_content = std::fs::read_to_string(&manifest_path)?;

            let manifest: PluginManifest<PluginState> =
                toml::from_str(&manifest_content).map_err(Error::Deserialize)?;

            plugins.insert(manifest.name.to_string(), Plugin { manifest });
        }

        self.plugins = plugins;
        Ok(())
    }

    /// flutter_rust_bridge:ignore
    pub fn list(&self) -> impl Iterator<Item = &Plugin> {
        self.plugins.values()
    }

    pub fn disable(&mut self, plugin_name: &str, reason: Option<String>) -> Result<bool> {
        let Some(plugin) = self.plugins.get_mut(plugin_name) else {
            return Ok(false);
        };

        match plugin.manifest.state {
            PluginState::Disable { .. } => return Ok(false),
            PluginState::Enable => {}
        }

        plugin.manifest.state = PluginState::Disable {
            reason: reason.map(String::into_boxed_str),
        };

        let plugin_path = self.plugins_path.join(&plugin.manifest.name);
        let source_path = plugin_path.join("source");
        std::fs::create_dir_all(&source_path)?;

        let manifest_path = source_path.join("manifest.toml");
        let manifest_content =
            toml::to_string_pretty(&plugin.manifest).map_err(Error::Serialize)?;
        std::fs::write(&manifest_path, manifest_content)?;

        Ok(true)
    }

    pub fn remove(&mut self, plugin_name: &str) -> Result<bool> {
        let Some(plugin) = self.plugins.remove(plugin_name) else {
            return Ok(false);
        };

        let plugin_path = self.plugins_path.join(&plugin.manifest.name);
        std::fs::remove_dir_all(plugin_path)?;

        Ok(true)
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn process_plugin_event(&mut self) -> Option<PluginAction> {
        let mut events = self.plugin_events.pop_front()?;
        let event = events.events.pop_front();

        let manifest = self.plugin(&events.plugin_name).unwrap().manifest.clone();
        let permissions = manifest.permission;
        if !events.events.is_empty() {
            self.plugin_events.push_back(events);
        }

        let event = event?;

        match &event {
            PluginEvent::TaskCreate { .. } if !permissions.task.is_some_and(|task| task.create) => {
                return Some(PluginAction::Disable {
                    plugin_name: manifest.name,
                    reason: "missing 'task.create' permission".to_string(),
                });
            }
            PluginEvent::TaskModify { .. } if !permissions.task.is_some_and(|task| task.modify) => {
                return Some(PluginAction::Disable {
                    plugin_name: manifest.name,
                    reason: "missing 'task.modify' permission".to_string(),
                });
            }
            PluginEvent::TaskSync if !permissions.task.is_some_and(|task| task.sync) => {
                return Some(PluginAction::Disable {
                    plugin_name: manifest.name,
                    reason: "missing 'task.sync' permission".to_string(),
                });
            }
            PluginEvent::TaskQuery { .. } if !permissions.task.is_some_and(|task| task.query) => {
                return Some(PluginAction::Disable {
                    plugin_name: manifest.name,
                    reason: "missing 'task.query' permission".to_string(),
                });
            }
            PluginEvent::NetworkRequest { host, .. } => {
                let Some(network) = permissions.network else {
                    return Some(PluginAction::Disable {
                        plugin_name: manifest.name,
                        reason: "missing 'network' permission".to_string(),
                    });
                };

                if !network.urls.contains(host) {
                    return Some(PluginAction::Disable {
                        plugin_name: manifest.name,
                        reason: format!("missing requested url in 'network.url': {host}"),
                    });
                }
            }
            _ => {}
        };

        Some(PluginAction::Event {
            plugin_name: manifest.name,
            event,
        })
    }
}
