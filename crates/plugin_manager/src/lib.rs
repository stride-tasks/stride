//! Plugin Manager used in `stride`.

#![allow(clippy::missing_errors_doc)]

use std::{
    collections::VecDeque,
    fmt::Debug,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use logging::PluginLogger;
use manifest::{PluginManifest, PluginState};
use wasmi::{core::ValType, Caller, Config, Engine, Extern, Func, Linker, Module, Store};
use wasmi_wasi::{WasiCtx, WasiCtxBuilder};
use zip::ZipArchive;

mod error;
mod logging;
pub mod manifest;

pub use error::{Error, Result};

const EVENT_HANDLER_NAME: &str = "stride__event_handler";

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
    events: Vec<Box<[u8]>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventType {
    TaskCreate,
    TaskModify,
    TaskSync,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub ty: EventType,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct EventQueue {
    plugin: String,
    events: Vec<Box<[u8]>>,
}

#[derive(Debug)]
pub struct Plugin {
    pub manifest: PluginManifest<PluginState>,
}

#[derive(Debug, Clone)]
pub enum PluginAction {
    Ok,
    Disable { reason: String },
}

pub trait Hook<E>: Debug {
    fn hook(
        &mut self,
        plugin_manager: &mut PluginManager,
        plugin: &str,
        event_data: &[u8],
    ) -> std::result::Result<PluginAction, E>;
}

#[derive(Debug)]
pub struct PluginManager {
    plugins_path: PathBuf,
    plugins: Vec<Plugin>,

    engine: Engine,

    events: VecDeque<EventQueue>,
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

            if !manifest.state.is_enabled() {
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

    #[must_use]
    pub fn plugin(&self, plugin_name: &str) -> Option<&Plugin> {
        self.plugins
            .iter()
            .find(|plugin| plugin.manifest.name == plugin_name)
    }
    #[must_use]
    pub fn plugin_mut(&mut self, plugin_name: &str) -> Option<&mut Plugin> {
        self.plugins
            .iter_mut()
            .find(|plugin| plugin.manifest.name == plugin_name)
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
                permissions: manifest.permissions,
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

        plugin.manifest.state = match plugin.manifest.state {
            PluginState::Disable { .. } => PluginState::Enable,
            PluginState::Enable => PluginState::Disable { reason: None },
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

    pub fn disable(&mut self, plugin_name: &str, reason: Option<Box<str>>) -> Result<bool> {
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

        plugin.manifest.state = PluginState::Disable { reason };

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
    pub fn emit_event(&mut self, event: &Event) -> Result<()> {
        for plugin in &self.plugins {
            match event.ty {
                EventType::TaskCreate if !plugin.manifest.events.task.create => continue,
                EventType::TaskModify if !plugin.manifest.events.task.modify => continue,
                EventType::TaskSync if !plugin.manifest.events.task.sync => continue,
                EventType::TaskCreate | EventType::TaskModify | EventType::TaskSync => {}
            }

            let plugin_path = self.plugins_path.join(&plugin.manifest.name);
            let source_path = plugin_path.join("source");
            let code_path = source_path.join("code.wasm");
            let wasm = std::fs::read(code_path)?;
            let module = Module::new(&self.engine, &wasm).expect("already validated");

            let wasi_ctx = wasi_context(&plugin.manifest.name);
            let host_state = HostState {
                wasi: wasi_ctx,
                events: Vec::default(),
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

                    caller.data_mut().events.push(json.into_boxed_slice());
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

            self.events.push_back(EventQueue {
                plugin: plugin.manifest.name.clone(),
                events: std::mem::take(&mut store.data_mut().events),
            });
        }

        Ok(())
    }

    pub fn process_events<R>(
        &mut self,
        hook: &mut dyn Hook<R>,
    ) -> Result<std::result::Result<(), R>> {
        let events = std::mem::take(&mut self.events);
        for EventQueue { plugin, events } in events {
            for event in events {
                let result = hook.hook(self, &plugin, &event);
                if result.is_err() {
                    return Ok(result.map(|_| ()));
                }
                match result {
                    Err(err) => return Ok(Err(err)),
                    Ok(action) => match action {
                        PluginAction::Ok => {}
                        PluginAction::Disable { reason } => {
                            self.disable(&plugin, Some(reason.into_boxed_str()))?;
                        }
                    },
                }
            }
        }
        Ok(Ok(()))
    }
}
