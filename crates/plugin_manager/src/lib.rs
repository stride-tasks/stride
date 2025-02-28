//! Plugin Manager used in `stride`.

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]

use std::{collections::VecDeque, fmt::Debug, fs::File, io::Read, path::Path};

use logging::PluginLogger;
use manifest::{PluginManifest, PluginState};
use stride_core::event::{HostEvent, PluginEvent};
use wasmi::{core::ValType, Caller, Extern, Func, Linker, Module, Store};
use wasmi_wasi::{WasiCtx, WasiCtxBuilder};
use zip::ZipArchive;

mod error;
mod logging;
pub mod manager;
pub mod manifest;

pub use error::{Error, Result};

pub use manager::PluginManager;

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
    events: VecDeque<PluginEvent>,
}

#[derive(Debug)]
pub struct EventQueue {
    plugin_name: String,
    events: VecDeque<PluginEvent>,
}

#[derive(Debug)]
pub struct ParsePlugin {
    pub manifest: PluginManifest<PluginState>,
    pub code: Box<[u8]>,
}

#[derive(Debug)]
pub struct Plugin {
    pub manifest: PluginManifest<PluginState>,
}

impl PluginManager {
    #[must_use]
    pub fn plugin(&self, plugin_name: &str) -> Option<&Plugin> {
        self.plugins.get(plugin_name)
    }
    #[must_use]
    pub fn plugin_mut(&mut self, plugin_name: &str) -> Option<&mut Plugin> {
        self.plugins.get_mut(plugin_name)
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

    pub fn parse_plugin(&self, plugin_archive_path: &Path) -> Result<ParsePlugin> {
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

        Ok(ParsePlugin {
            manifest: PluginManifest::<PluginState> {
                api: manifest.api,
                name: manifest.name,
                events: manifest.events,
                permissions: manifest.permissions,
                state: PluginState::default(),
            },
            code: code_content.into_boxed_slice(),
        })
    }

    pub fn import(&mut self, plugin_archive_path: &Path) -> Result<()> {
        let plugin = self.parse_plugin(plugin_archive_path)?;

        self.install(plugin)?;
        Ok(())
    }

    pub fn toggle(&mut self, plugin_name: &str) -> Result<bool> {
        let Some(plugin) = self.plugins.get_mut(plugin_name) else {
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

    fn install(&mut self, plugin: ParsePlugin) -> Result<()> {
        let plugin_path = self.plugins_path.join(&plugin.manifest.name);
        let source_path = plugin_path.join("source");
        std::fs::create_dir_all(&source_path)?;

        let manifest_path = source_path.join("manifest.toml");
        let manifest_content =
            toml::to_string_pretty(&plugin.manifest).map_err(Error::Serialize)?;
        std::fs::write(&manifest_path, manifest_content)?;

        let code_path = source_path.join("code.wasm");
        std::fs::write(&code_path, &plugin.code)?;

        self.plugins.insert(
            plugin.manifest.name.to_string(),
            Plugin {
                manifest: plugin.manifest,
            },
        );
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::too_many_lines)]
    pub fn emit_event(&mut self, event: &HostEvent) -> Result<()> {
        for plugin in self.plugins.values() {
            if !plugin.manifest.state.is_enabled() {
                continue;
            }

            match event {
                HostEvent::TaskCreate { .. } if !plugin.manifest.events.task.create => continue,
                HostEvent::TaskModify { .. } if !plugin.manifest.events.task.modify => continue,
                HostEvent::TaskSync if !plugin.manifest.events.task.sync => continue,
                HostEvent::TaskCreate { .. }
                | HostEvent::TaskModify { .. }
                | HostEvent::TaskSync => {}
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
                    let data = data.get(event_data..event_data + event_len).unwrap();

                    let event: PluginEvent = serde_json::from_slice(data).unwrap();

                    caller.data_mut().events.push_back(event);
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

            let event_data = serde_json::to_string(&event).unwrap();
            store.set_fuel(100_000).unwrap();
            let ret = stride_allocate
                .call(&mut store, event_data.len() as i32)
                .unwrap() as usize;

            let memory = instance.get_memory(&mut store, "memory").unwrap();
            memory.data_mut(&mut store)[ret..ret + event_data.len()]
                .copy_from_slice(event_data.as_bytes());

            // And finally we can call the wasm!
            store.set_fuel(1_000_000).unwrap();
            event_handler
                .call(&mut store, (ret as i32, event_data.len() as i32))
                .unwrap();

            stride_deallocate
                .call(&mut store, (ret as i32, event_data.len() as i32))
                .unwrap();

            self.plugin_events.push_back(EventQueue {
                plugin_name: plugin.manifest.name.clone(),
                events: std::mem::take(&mut store.data_mut().events),
            });
        }

        Ok(())
    }
}
