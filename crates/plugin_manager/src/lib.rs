//! Plugin Manager used in `stride`.

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]

use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use logging::PluginLogger;
use manifest::{PluginManifest, PluginState};
use stride_core::{
    constant::StorageErrorCode,
    event::{HostEvent, PluginEvent},
};
use wasmi::{core::ValType, Caller, Extern, Func, FuncType, Linker, Module, Store};
use wasmi_wasi::{WasiCtx, WasiCtxBuilder};
use zip::ZipArchive;

mod error;
mod logging;
pub mod manager;
pub mod manifest;

pub use error::{Error, Result};

pub use manager::PluginManager;

const EVENT_HANDLER_NAME: &str = "stride__event_handler";
const STORAGE_SET_NAME: &str = "stride__storage_set";
const STORAGE_GET_NAME: &str = "stride__storage_get";
const STORAGE_REMOVE_NAME: &str = "stride__storage_remove";

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

struct StorageState {
    data: HashMap<Box<[u8]>, Box<[u8]>>,
    max: usize,
    size: usize,
    needs_save: bool,
}

struct HostState {
    wasi: WasiCtx,
    events: VecDeque<PluginEvent>,
    storage: Option<StorageState>,
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

impl Plugin {
    fn can_accept_event(&self, event: &HostEvent) -> bool {
        if !self.manifest.state.is_enabled() {
            return false;
        }
        match event {
            HostEvent::TaskCreate { .. } if !self.manifest.events.task.create => return false,
            HostEvent::TaskModify { .. } if !self.manifest.events.task.modify => return false,
            HostEvent::TaskSync if !self.manifest.events.task.sync => return false,
            HostEvent::Timer { interval } => {
                let Some(timer) = &self.manifest.events.timer else {
                    return false;
                };
                return timer.interval == *interval;
            }
            HostEvent::TaskQuery { .. } if !self.manifest.permissions.task.query => return false,
            HostEvent::NetworkResponse { host, .. } => {
                let Some(network) = &self.manifest.permissions.network else {
                    return false;
                };

                if !network.urls.contains(host) {
                    return false;
                }
            }
            HostEvent::TaskCreate { .. }
            | HostEvent::TaskModify { .. }
            | HostEvent::TaskSync
            | HostEvent::TaskQuery { .. } => {}
        }
        true
    }
}

fn check_signature_match(
    name: &str,
    func: &FuncType,
    expected_params: &[ValType],
    expected_return: &[ValType],
) -> Result<()> {
    let params = func.params();
    let mut matches = true;
    if params != [ValType::I32, ValType::I32] {
        matches = false;
    }

    let results = func.results();
    if results != [ValType::I32] {
        matches = false;
    }

    if matches {
        return Ok(());
    }

    Err(Error::ExportFunctionSignature {
        function_name: name.to_string(),
        expected_params: expected_params.to_vec().into_boxed_slice(),
        expected_return: expected_return.to_vec().into_boxed_slice(),
        actual_params: func.params().to_vec().into_boxed_slice(),
        actual_return: func.results().to_vec().into_boxed_slice(),
    })
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
        let module = Module::new(&self.engine, wasm)?;

        let mut has_memory_export = false;
        for export in module.exports() {
            let name = export.name();

            if export.ty().memory().is_some() && name == "memory" {
                has_memory_export = true;
            }
            let Some(func) = export.ty().func() else {
                continue;
            };

            if name == EVENT_HANDLER_NAME {
                check_signature_match(name, func, &[ValType::I32, ValType::I32], &[ValType::I32])?;
            }
            if name == STORAGE_GET_NAME {
                check_signature_match(
                    name,
                    func,
                    &[ValType::I32, ValType::I32, ValType::I32],
                    &[ValType::I32],
                )?;
            }
            if name == STORAGE_SET_NAME {
                check_signature_match(
                    name,
                    func,
                    &[ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                    &[ValType::I32],
                )?;
            }
            if name == STORAGE_REMOVE_NAME {
                check_signature_match(name, func, &[ValType::I32, ValType::I32], &[ValType::I32])?;
            }
        }
        if !has_memory_export {
            return Err(Error::MissingMemoryExport);
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
    pub fn emit_event(&mut self, plugin_name: Option<&str>, event: &HostEvent) -> Result<()> {
        if let Some(plugin_name) = plugin_name {
            let plugin = self.plugin(plugin_name).ok_or("plugin not found").unwrap();
            if plugin.can_accept_event(event) {
                self.host_events
                    .push_back((plugin.manifest.name.to_string(), event.clone()));
            }
            return Ok(());
        }
        for plugin in self.plugins.values() {
            if !plugin.can_accept_event(event) {
                continue;
            }

            self.host_events
                .push_back((plugin.manifest.name.to_string(), event.clone()));
        }

        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::too_many_lines)]
    pub fn process_host_event(&mut self) -> Result<bool> {
        let Some((plugin, event)) = self.host_events.pop_front() else {
            return Ok(false);
        };

        let Some(plugin) = self.plugins.get(&plugin) else {
            return Ok(false);
        };

        if !plugin.manifest.state.is_enabled() {
            return Ok(false);
        }

        let plugin_path = self.plugins_path.join(&plugin.manifest.name);
        let source_path = plugin_path.join("source");
        let code_path = source_path.join("code.wasm");
        let wasm = std::fs::read(code_path)?;
        let module = Module::new(&self.engine, &wasm).expect("already validated");

        let has_storage_permission = plugin.manifest.permissions.storage.is_some();

        let storage_filepath = plugin_path.join("store");

        let mut storage = HashMap::new();
        let mut storage_size = 0;
        if has_storage_permission && storage_filepath.exists() {
            let file = File::open(&storage_filepath)?;
            let mut reader = BufReader::new(file);
            loop {
                let mut size = 0u32.to_be_bytes();
                let count = reader.read(&mut size)?;
                if count == 0 {
                    break;
                }
                let size = u32::from_be_bytes(size) as usize;
                let mut key = vec![0u8; size];
                reader.read_exact(&mut key)?;

                let mut size = 0u32.to_be_bytes();
                reader.read_exact(&mut size)?;
                let size = u32::from_be_bytes(size) as usize;
                let mut value = vec![0u8; size];
                reader.read_exact(&mut value)?;

                storage_size += key.len();
                storage_size += value.len();

                storage.insert(key.into_boxed_slice(), value.into_boxed_slice());
            }
        }

        let wasi_ctx = wasi_context(&plugin.manifest.name);
        let host_state = HostState {
            wasi: wasi_ctx,
            events: VecDeque::default(),
            storage: has_storage_permission.then(|| StorageState {
                data: storage,
                size: storage_size,
                max: plugin
                    .manifest
                    .permissions
                    .storage
                    .map_or(0, |storage| storage.max_size as usize)
                    * 1024,
                needs_save: false,
            }),
        };
        let mut store = Store::new(&self.engine, host_state);

        let mut linker = <Linker<HostState>>::new(&self.engine);

        wasmi_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi).map_err(Error::Wasi)?;

        let stride_emit = Func::wrap(
            &mut store,
            |mut caller: Caller<'_, HostState>, event_data: i32, event_len: i32| {
                let Some(memory_export) = caller.get_export("memory").and_then(Extern::into_memory)
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
        linker.define("env", "stride__emit", stride_emit)?;

        let stride_storage_get = Func::wrap(
            &mut store,
            |mut caller: Caller<'_, HostState>, key: i32, key_len: i32, value_ptr: i32| -> i32 {
                let export = caller.get_export("memory");
                let Some(memory_export) = export.and_then(Extern::into_memory) else {
                    return StorageErrorCode::MissingMemoryExport as i32;
                };

                let (memory, host_state) = memory_export.data_and_store_mut(&mut caller);

                let Some(storage) = &mut host_state.storage else {
                    return StorageErrorCode::Permission as i32;
                };

                let key = key as usize;
                let key_len = key_len as usize;
                let key = memory.get(key..key + key_len);

                let Some(key) = key else {
                    return StorageErrorCode::InvalidKey as i32;
                };

                if value_ptr == 0 {
                    if let Some(value) = storage.data.get(key) {
                        return value.len() as i32;
                    }
                    return StorageErrorCode::NotFound as i32;
                }
                if let Some(value) = storage.data.get(key) {
                    let value_ptr = value_ptr as usize;
                    memory[value_ptr..value_ptr + value.len()].copy_from_slice(value);
                    return value.len() as i32;
                }

                StorageErrorCode::NotFound as i32
            },
        );
        linker.define("env", "stride__storage_get", stride_storage_get)?;

        let stride_storage_set = Func::wrap(
            &mut store,
            |mut caller: Caller<'_, HostState>,
             key: i32,
             key_len: i32,
             value_ptr: i32,
             value_len: i32|
             -> i32 {
                let export = caller.get_export("memory");
                let Some(memory_export) = export.and_then(Extern::into_memory) else {
                    return StorageErrorCode::MissingMemoryExport as i32;
                };

                let (memory, host_state) = memory_export.data_and_store_mut(&mut caller);

                let Some(storage) = &mut host_state.storage else {
                    return StorageErrorCode::Permission as i32;
                };

                let key = key as usize;
                let key_len = key_len as usize;
                let key = memory.get(key..key + key_len);

                let Some(key) = key else {
                    return StorageErrorCode::InvalidKey as i32;
                };

                let value =
                    memory.get((value_ptr as usize)..(value_ptr as usize) + value_len as usize);

                let Some(value) = value else {
                    return StorageErrorCode::InvalidValue as i32;
                };

                storage.size += key.len();
                storage.size += value.len();

                if storage.size > storage.max {
                    return StorageErrorCode::OutOfStorage as i32;
                }

                storage.data.insert(
                    key.to_vec().into_boxed_slice(),
                    value.to_vec().into_boxed_slice(),
                );
                storage.needs_save = true;

                0
            },
        );
        linker.define("env", "stride__storage_set", stride_storage_set)?;

        let stride_storage_remove = Func::wrap(
            &mut store,
            |mut caller: Caller<'_, HostState>, key: i32, key_len: i32| -> i32 {
                let export = caller.get_export("memory");
                let Some(memory_export) = export.and_then(Extern::into_memory) else {
                    return StorageErrorCode::MissingMemoryExport as i32;
                };

                let (memory, host_state) = memory_export.data_and_store_mut(&mut caller);

                let Some(storage) = &mut host_state.storage else {
                    return StorageErrorCode::Permission as i32;
                };

                let key = key as usize;
                let key_len = key_len as usize;
                let key = memory.get(key..key + key_len);

                let Some(key) = key else {
                    return StorageErrorCode::InvalidKey as i32;
                };

                let value = storage.data.remove(key);
                if let Some(value) = &value {
                    storage.size = storage.size.saturating_sub(key.len());
                    storage.size = storage.size.saturating_sub(value.len());
                }
                storage.needs_save = true;
                i32::from(value.is_some())
            },
        );
        linker.define("env", "stride__storage_remove", stride_storage_remove)?;

        let instance = linker
            .instantiate(&mut store, &module)?
            .ensure_no_start(&mut store)?;

        let stride_allocate = instance.get_typed_func::<i32, i32>(&store, "stride__allocate")?;

        let stride_deallocate =
            instance.get_typed_func::<(i32, i32), ()>(&store, "stride__deallocate")?;

        let stride_init = instance.get_typed_func::<(), ()>(&store, "stride__init")?;

        let event_handler =
            instance.get_typed_func::<(i32, i32), i32>(&store, EVENT_HANDLER_NAME)?;

        store.set_fuel(100_000)?;
        stride_init.call(&mut store, ())?;

        let event_data = serde_json::to_string(&event).expect("shouldn't fail");
        store.set_fuel(100_000)?;
        let ret = stride_allocate.call(&mut store, event_data.len() as i32)? as usize;

        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("already checked in validation");
        memory.data_mut(&mut store)[ret..ret + event_data.len()]
            .copy_from_slice(event_data.as_bytes());

        // TODO: Add computation limit.
        store.set_fuel(100_000_000_000)?;
        event_handler.call(&mut store, (ret as i32, event_data.len() as i32))?;

        stride_deallocate.call(&mut store, (ret as i32, event_data.len() as i32))?;

        if let Some(storage) = store.data_mut().storage.take() {
            if storage.needs_save {
                let file = File::create(&storage_filepath)?;
                let mut writer = BufWriter::new(file);

                for (key, value) in storage.data {
                    let key_len = key.len() as u32;
                    let value_len = value.len() as u32;
                    writer.write_all(&key_len.to_be_bytes())?;
                    writer.write_all(&key)?;
                    writer.write_all(&value_len.to_be_bytes())?;
                    writer.write_all(&value)?;
                }
            }
        }

        self.plugin_events.push_back(EventQueue {
            plugin_name: plugin.manifest.name.clone(),
            events: std::mem::take(&mut store.data_mut().events),
        });

        Ok(true)
    }
}
