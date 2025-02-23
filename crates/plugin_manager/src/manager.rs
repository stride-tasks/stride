use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use stride_core::event::{HostEvent, PluginEvent};
use wasmi::{Config, Engine};

use crate::{
    manifest::{PluginAction, PluginManifest, PluginState},
    Error, EventQueue, Plugin, Result,
};

/// flutter_rust_bridge:unignore
#[derive(Debug, Default)]
pub struct PluginManager {
    pub(crate) plugins_path: PathBuf,
    pub(crate) plugins: Vec<Plugin>,

    pub(crate) engine: Engine,

    pub(crate) plugin_events: VecDeque<EventQueue>,
}

impl PluginManager {
    pub fn new(plugins_path: String) -> Result<Self> {
        let plugins_path = Path::new(&plugins_path);
        if !plugins_path.exists() {
            std::fs::create_dir_all(plugins_path)?;
        }

        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);

        Ok(Self {
            plugins_path: plugins_path.to_path_buf(),
            plugins: Vec::new(),

            engine,
            plugin_events: VecDeque::new(),
        })
    }

    pub fn load(&mut self) -> Result<()> {
        let entries = self.plugins_path.read_dir()?;

        let mut plugins = Vec::new();
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

            if !manifest.state.is_enabled() {
                continue;
            }

            plugins.push(Plugin { manifest });
        }

        self.plugins = plugins;
        Ok(())
    }

    /// flutter_rust_bridge:ignore
    pub fn list(&self) -> Result<&[Plugin]> {
        Ok(&self.plugins)
    }

    pub fn disable(&mut self, plugin_name: &str, reason: Option<String>) -> Result<bool> {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.manifest.name == plugin_name)
        else {
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

    pub fn emit(&mut self, event: HostEvent) -> Result<()> {
        self.emit_event(&event)
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn process_event(&mut self) -> Option<PluginAction> {
        let mut events = self.plugin_events.pop_front()?;
        let event = events.events.pop_front();

        let manifest = self.plugin(&events.plugin_name).unwrap().manifest.clone();
        let permissions = manifest.permissions;
        if !events.events.is_empty() {
            self.plugin_events.push_back(events);
        }

        let event = event?;

        let action = match event {
            PluginEvent::TaskCreate { .. } if !permissions.task.create => PluginAction::Disable {
                plugin_name: manifest.name,
                reason: "missing 'task.create' permission".to_string(),
            },
            PluginEvent::TaskModify { .. } if !permissions.task.modify => PluginAction::Disable {
                plugin_name: manifest.name,
                reason: "missing 'task.modify' permission".to_string(),
            },
            PluginEvent::TaskSync if !permissions.task.sync => PluginAction::Disable {
                plugin_name: manifest.name,
                reason: "missing 'task.sync' permission".to_string(),
            },
            _ => PluginAction::Event {
                plugin_name: manifest.name,
                event,
            },
        };

        Some(action)
    }
}
