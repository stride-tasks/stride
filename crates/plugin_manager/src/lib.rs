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
use wasmi::{core::ValType, Caller, Config, Engine, Extern, Func, Linker, Module, Store};
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
    events: VecDeque<Vec<u8>>,
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

const EVENT_HANDLER_NAME: &str = "stride__event_handler";

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

pub trait Hook<E>: Debug {
    fn hook(&mut self, plugin: &str, event_data: &[u8]) -> std::result::Result<bool, E>;
}

#[derive(Debug)]
pub struct PluginManager<R> {
    plugins_path: PathBuf,
    plugins: Vec<Plugin>,

    engine: Engine,

    hook: Option<Box<dyn Hook<R>>>,
}

impl<R> PluginManager<R> {
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

            hook: None,
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

    fn validate_wasm_code(&self, _manifest: &PluginManifest, wasm: &[u8]) -> Result<()> {
        let module = Module::new(&self.engine, wasm).map_err(Error::InvalidCode)?;

        for export in module.exports() {
            let Some(func) = export.ty().func() else {
                continue;
            };

            let name = export.name();
            if name != EVENT_HANDLER_NAME {
                continue;
            }

            let params = func.params();
            if params != [ValType::I32, ValType::I32] {
                return Err(Error::EventHandlerSignature(name.to_string()));
            }

            let results = func.results();
            if results != [ValType::I32] {
                return Err(Error::EventHandlerSignature(name.to_string()));
            }
        }
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
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::too_many_lines)]
    pub fn emit_event(&mut self, event: &Event) -> Result<std::result::Result<(), R>> {
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

            let wasi_ctx = wasi_context(&plugin.manifest.name);
            let host_state = HostState {
                wasi: wasi_ctx,
                events: VecDeque::default(),
            };
            let mut store = Store::new(&self.engine, host_state);

            let mut linker = <Linker<HostState>>::new(&self.engine);

            wasmi_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi).unwrap();

            let stride_emit = Func::wrap(
                &mut store,
                |mut caller: Caller<'_, HostState>, event_data: i32, event_len: i32| {
                    let Some(memory_export) =
                        caller.get_export("memory").and_then(Extern::into_memory)
                    else {
                        return;
                    };

                    let data = memory_export.data(&mut caller);

                    let event_data = event_data as usize;
                    let event_len = event_len as usize;
                    let json = data
                        .get(event_data..event_data + event_len)
                        .unwrap()
                        .to_vec();

                    caller.data_mut().events.push_back(json);
                },
            );
            linker.define("env", "stride__emit", stride_emit).unwrap();

            let instance = linker
                .instantiate(&mut store, &module)
                .unwrap()
                .ensure_no_start(&mut store)
                .unwrap();

            let stride_allocate = instance
                .get_typed_func::<i32, i32>(&store, "stride__allocate")
                .unwrap();

            let stride_deallocate = instance
                .get_typed_func::<(i32, i32), ()>(&store, "stride__deallocate")
                .unwrap();

            let stride_init = instance
                .get_typed_func::<(), ()>(&store, "stride__init")
                .unwrap();

            let event_handler = instance
                .get_typed_func::<(i32, i32), i32>(&store, EVENT_HANDLER_NAME)
                .unwrap();

            store.set_fuel(100_000).unwrap();
            stride_init.call(&mut store, ()).unwrap();

            store.set_fuel(100_000).unwrap();
            let ret = stride_allocate
                .call(&mut store, event.data.len() as i32)
                .unwrap() as usize;

            let memory = instance.get_memory(&mut store, "memory").unwrap();
            memory.data_mut(&mut store)[ret..ret + event.data.len()]
                .copy_from_slice(event.data.as_slice());

            // And finally we can call the wasm!
            store.set_fuel(1_000_000).unwrap();
            event_handler
                .call(&mut store, (ret as i32, event.data.len() as i32))
                .unwrap();

            stride_deallocate
                .call(&mut store, (ret as i32, event.data.len() as i32))
                .unwrap();

            let Some(hook) = &mut self.hook else {
                return Ok(Ok(()));
            };
            for event in store.into_data().events {
                let result = hook.hook(&plugin.manifest.name, &event);
                if result.is_err() {
                    return Ok(result.map(|_| ()));
                }
            }
        }
        Ok(Ok(()))
    }

    pub fn insert_hook<T: Hook<R> + 'static>(&mut self, hook: T) -> Option<Box<dyn Hook<R>>> {
        self.hook.replace(Box::new(hook))
    }
}
