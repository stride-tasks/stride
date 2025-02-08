//! Plugin Manager used in `stride`.

#![allow(clippy::missing_errors_doc)]

use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use logging::PluginLogger;
use serde::{Deserialize, Serialize};
use wasmi::{core::ValType, Caller, Config, Engine, Func, Linker, Module, Store};
use wasmi_wasi::{WasiCtx, WasiCtxBuilder};
use zip::ZipArchive;

mod error;
mod logging;

pub use error::{Error, Result};

/// Creates the [`WasiCtx`] for this session.
fn wasi_context(plugin_name: &str) -> WasiCtx {
    let mut wasi_builder = WasiCtxBuilder::new();
    // wasi_builder.preopened_dir(Dir::, guest_path);
    // wasi_builder.args(&self.argv())?;
    // Add pre-opened TCP sockets.
    //
    // Note that `num_fd` starts at 3 because the inherited `stdin`, `stdout` and `stderr`
    // are already mapped to `0, 1, 2` respectively.

    // wasi_builder.inherit_stdout();
    wasi_builder.stdout(Box::new(PluginLogger::new(plugin_name.to_string(), false)));
    wasi_builder.stderr(Box::new(PluginLogger::new(plugin_name.to_string(), true)));

    // for (socket, num_fd) in self.preopen_sockets()?.into_iter().zip(3..) {
    //     wasi_builder.preopened_socket(num_fd, socket)?;
    // }
    // Add pre-opened directories.
    // for (dir_name, dir) in self.preopen_dirs()? {
    //     wasi_builder.preopened_dir(dir, dir_name)?;
    // }
    wasi_builder.build()
}

struct HostState {
    wasi: WasiCtx,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventType {
    pub plugin: String,
    pub name: String,
}

impl EventType {
    fn plugin_name(&self) -> &str {
        &self.plugin
    }
    fn event_name(&self) -> &str {
        &self.name
    }
    fn new(plugin_name: &str, event_name: &str) -> Self {
        Self {
            plugin: plugin_name.to_string(),
            name: event_name.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventHandlerType {
    plugin: String,
    ty: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub ty: EventType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginApi {
    V1,
}

const EVENT_HANDLER_PREFIX: &str = "stride__handler__";

pub type PluginName = String;
pub type EventName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginEvent {
    types: Vec<EventName>,
}

pub trait ManifestState: Sized {
    fn skip_serializing(&self) -> bool {
        true
    }
}

impl ManifestState for () {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginState {
    pub enabled: bool,
}

impl Default for PluginState {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl ManifestState for PluginState {
    fn skip_serializing(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginManifest<T: ManifestState = ()> {
    pub api: PluginApi,
    pub name: PluginName,
    pub events: HashMap<PluginName, PluginEvent>,

    #[serde(default)]
    #[serde(skip_serializing_if = "ManifestState::skip_serializing")]
    pub state: T,
}

#[derive(Debug)]
pub struct Plugin {
    pub manifest: PluginManifest<PluginState>,
}

pub trait Hook: Debug {
    fn hook(&mut self, plugin_manager: &mut PluginManager, event: &Event) -> Result<bool>;
}

#[derive(Debug)]
pub struct PluginManager {
    plugins_path: PathBuf,
    plugins: Vec<Plugin>,

    engine: Engine,

    hooks: HashMap<EventType, Box<dyn Hook + 'static>>,
    events: VecDeque<Event>,
}

impl PluginManager {
    pub fn new(plugins_path: &Path) -> Result<Self> {
        // let plugins_path = application_support_path().join("plugins");
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

            hooks: HashMap::new(),
            events: VecDeque::new(),
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

            if !manifest.state.enabled {
                continue;
            }

            plugins.push(Plugin { manifest });
        }

        self.plugins = plugins;
        Ok(())
    }

    pub fn list(&self) -> Result<&[Plugin]> {
        Ok(&self.plugins)
    }

    fn validate_wasm_code(&self, manifest: &PluginManifest, wasm: &[u8]) -> Result<()> {
        let module = Module::new(&self.engine, wasm).map_err(Error::InvalidCode)?;

        let mut event_handler_exports = HashMap::new();
        for export in module.exports() {
            let Some(func) = export.ty().func() else {
                continue;
            };

            let name = export.name();
            if !name.starts_with(EVENT_HANDLER_PREFIX) {
                // Logger::warn(&format!(
                //     "Plugin({}): Skipping exported function: {name}",
                //     manifest.name
                // ));
                continue;
            }
            let name_without_prefix = &name[EVENT_HANDLER_PREFIX.len()..];
            let Some((plugin_name, event_type)) = name_without_prefix.split_once("__") else {
                return Err(Error::InvalidEventHandlerName(name.to_string()));
            };

            let event_type = event_type.replace('_', "-");
            let Some(plugin_event) = manifest.events.get(plugin_name) else {
                // Logger::warn(&format!(
                //     "Plugin({}): manifest missing event handler: {plugin_name}/{event_type}",
                //     manifest.name
                // ));
                continue;
            };
            let event_handler_export = event_handler_exports
                .entry(plugin_name)
                .or_insert_with(Vec::new);

            if !plugin_event.types.contains(&event_type) {
                return Err(Error::MissingEventHandler(format!(
                    "{plugin_name}/{event_type}"
                )));
            }
            event_handler_export.push(event_type);

            let params = func.params();
            if params != [ValType::I32, ValType::I32] {
                return Err(Error::EventHandlerSignature(name.to_string()));
            }

            let results = func.results();
            if results != [ValType::I32] {
                return Err(Error::EventHandlerSignature(name.to_string()));
            }
        }
        // for hook in
        Ok(())
    }

    pub fn import(&mut self, plugin_archive_path: &str) -> Result<()> {
        let file = File::open(plugin_archive_path)?;
        let mut archive = ZipArchive::new(file).map_err(Error::from)?;

        let mut manifest: Option<PluginManifest> = None;
        let mut code_content = None;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(Error::from)?;
            let filename = file.name();
            if filename == "manifest.toml" {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                manifest = Some(toml::from_str(&contents).map_err(Error::Deserialize)?);
            } else if filename == "code.wasm" {
                let mut contents = Vec::<u8>::new();
                file.read_to_end(&mut contents)?;
                code_content = Some(contents);
            } else {
                return Err(Error::UnknownFile {
                    filename: filename.to_string(),
                });
            }
        }

        let Some(manifest) = manifest else {
            return Err(Error::MissingManifest);
        };

        if manifest.name.is_empty() || manifest.name.len() > 255 || !manifest.name.is_ascii() {
            return Err(Error::InvalidName {
                name: manifest.name.to_string(),
            });
        }

        let Some(code_content) = code_content else {
            return Err(Error::MissingCode);
        };

        self.validate_wasm_code(&manifest, &code_content)?;

        let plugin = Plugin {
            manifest: PluginManifest::<PluginState> {
                api: manifest.api,
                name: manifest.name,
                events: manifest.events,
                state: PluginState::default(),
            },
        };

        self.install(plugin, code_content.as_slice())?;
        Ok(())
    }

    pub fn toggle(&mut self, plugin_name: &str) -> Result<bool> {
        let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|plugin| plugin.manifest.name == plugin_name)
        else {
            return Ok(false);
        };

        plugin.manifest.state.enabled = !plugin.manifest.state.enabled;

        let plugin_path = self.plugins_path.join(&plugin.manifest.name);
        let source_path = plugin_path.join("source");
        std::fs::create_dir_all(&source_path)?;

        let manifest_path = source_path.join("manifest.toml");
        let manifest_content =
            toml::to_string_pretty(&plugin.manifest).map_err(Error::Serialize)?;
        std::fs::write(&manifest_path, manifest_content)?;

        Ok(true)
    }

    fn install(&mut self, plugin: Plugin, code: &[u8]) -> Result<()> {
        let plugin_path = self.plugins_path.join(&plugin.manifest.name);
        let source_path = plugin_path.join("source");
        std::fs::create_dir_all(&source_path)?;

        let manifest_path = source_path.join("manifest.toml");
        let manifest_content =
            toml::to_string_pretty(&plugin.manifest).map_err(Error::Serialize)?;
        std::fs::write(&manifest_path, manifest_content)?;

        let code_path = source_path.join("code.wasm");
        std::fs::write(&code_path, code)?;

        self.plugins.push(plugin);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    pub fn emit_event(&mut self, event: &Event) -> Result<()> {
        for plugin in &self.plugins {
            if !plugin
                .manifest
                .events
                .get(event.ty.plugin_name())
                .is_some_and(|plugin_event| {
                    plugin_event
                        .types
                        .contains(&event.ty.event_name().to_string())
                })
            {
                continue;
            }

            let plugin_path = self.plugins_path.join(&plugin.manifest.name);
            let source_path = plugin_path.join("source");
            let code_path = source_path.join("code.wasm");
            let wasm = std::fs::read(code_path)?;
            let module = Module::new(&self.engine, &wasm).expect("already validated");

            let mut event_handler_exports = HashMap::new();
            for export in module.exports() {
                let Some(_func) = export.ty().func() else {
                    continue;
                };

                let name = export.name();
                if !name.starts_with(EVENT_HANDLER_PREFIX) {
                    continue;
                }
                let name_without_prefix = &name[EVENT_HANDLER_PREFIX.len()..];
                let Some((plugin_name, event_type)) = name_without_prefix.split_once("__") else {
                    continue;
                };

                let event_type = event_type.replace('_', "-");

                event_handler_exports
                    .insert(EventType::new(plugin_name, event_type.as_str()), name);
            }

            let wasi_ctx = wasi_context(&plugin.manifest.name);
            let host_state = HostState { wasi: wasi_ctx };
            let mut store = Store::new(&self.engine, host_state);
            store.set_fuel(500).unwrap();

            // In order to create Wasm module instances and link their imports
            // and exports we require a `Linker`.
            let mut linker = <Linker<HostState>>::new(&self.engine);

            wasmi_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi).unwrap();

            let host_hello = Func::wrap(
                &mut store,
                |_caller: Caller<'_, HostState>, param1: i32, param2: i32, param3: i32| {
                    println!("Got {param1},{param2},{param3} from WebAssembly");
                    0
                },
            );
            linker
                .define(
                    "env",
                    "stride__resource_get__stirde__task_title",
                    host_hello,
                )
                .unwrap();

            let instance = linker
                .instantiate(&mut store, &module)
                .unwrap()
                .start(&mut store)
                .unwrap();

            let mut event_handlers = HashMap::new();
            for (event_type, export_name) in event_handler_exports {
                let event_handler = instance
                    .get_typed_func::<(i32, i32), i32>(&store, export_name)
                    .unwrap();
                event_handlers.insert(event_type, event_handler);
            }

            let stride_allocate = instance
                .get_typed_func::<i32, i32>(&store, "stride__allocate")
                .unwrap();

            let stride_deallocate = instance
                .get_typed_func::<(i32, i32), ()>(&store, "stride__deallocate")
                .unwrap();

            store.set_fuel(50_000).unwrap();
            let ret = stride_allocate
                .call(&mut store, event.data.len() as i32)
                .unwrap() as usize;

            let memory = instance.get_memory(&mut store, "memory").unwrap();
            memory.data_mut(&mut store)[ret..ret + event.data.len()]
                .copy_from_slice(event.data.as_slice());

            let Some(event_handler) = event_handlers.get(&event.ty) else {
                println!("Spec is incorrect/mismatch: {:?}", event.ty);
                continue;
            };

            // And finally we can call the wasm!
            event_handler
                .call(&mut store, (ret as i32, event.data.len() as i32))
                .unwrap();

            stride_deallocate
                .call(&mut store, (ret as i32, event.data.len() as i32))
                .unwrap();
        }
        Ok(())
    }

    pub fn insert_hook<T: Hook + 'static>(
        &mut self,
        event_type: EventType,
        hook: T,
    ) -> Option<Box<dyn Hook>> {
        self.hooks.insert(event_type, Box::new(hook))
    }
}
