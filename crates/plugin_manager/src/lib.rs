//! Plugin Manager used in `stride`.

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]

use std::{collections::VecDeque, fmt::Debug, fs::File, io::Read, path::Path};

use error::ToPluginError;
use logging::PluginLogger;
use stride_api::{
    self as api, Method, Notification, PluginManifest, PluginManifestInstance, PluginState,
};
use wasmi::{Caller, Extern, Func, FuncType, Linker, Module, Store, ValType};
use wasmi_wasi::{WasiCtx, WasiCtxBuilder};
use zip::ZipArchive;

pub mod async_wasm;
mod error;
mod logging;
pub mod manager;
mod storage;

pub use error::{Error, Result};
use storage::{is_valid_plugin_name, sanitize_plugin_storage_call};

pub use async_wasm::{AsyncFunc, AsyncTypedFunc};
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
    calls: VecDeque<(Box<str>, api::Value)>,
    call_error: Option<Box<str>>,
}

#[derive(Debug)]
pub struct PluginCallQueue {
    pub plugin_name: Box<str>,
    pub calls: VecDeque<(Box<str>, api::Value)>,
}

#[derive(Debug)]
pub struct ParsePlugin {
    pub instance: PluginManifestInstance,
    pub code: Box<[u8]>,
}

#[derive(Debug)]
pub struct Plugin {
    pub instance: PluginManifestInstance,
}

impl Plugin {
    #[must_use]
    pub fn can_accept_notification(&self, notification_name: &str) -> bool {
        // TOOD: Implement is_* functions for enums in rust (api_codegen).
        if !matches!(self.instance.state, PluginState::Enable {}) {
            return false;
        }
        let Some(events) = &self.instance.manifest.event.task else {
            return false;
        };
        match notification_name {
            api::RepositoryTaskAddedNotification::NAME => events.create,
            api::RepositoryTaskUpdatedNotification::NAME => events.modify,
            api::RepositoryChangedNotification::NAME => events.sync,
            _ => false,
        }
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

    fn validate_wasm_code(&self, manifest: &PluginManifest, wasm: &[u8]) -> Result<()> {
        let module = Module::new(&self.engine, wasm).to_error(&manifest.name)?;

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

        if manifest.name.len() > 255 || !is_valid_plugin_name(&manifest.name) {
            return Err(Error::InvalidName {
                name: manifest.name.to_string(),
            });
        }

        let Some(code_content) = code_content else {
            return Err(Error::MissingCode);
        };

        self.validate_wasm_code(&manifest, &code_content)?;

        Ok(ParsePlugin {
            instance: PluginManifestInstance {
                manifest,
                state: PluginState::Enable {},
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

        plugin.instance.state = match plugin.instance.state {
            PluginState::Disable { .. } => PluginState::Enable {},
            PluginState::Enable {} => PluginState::Disable { reason: None },
        };

        let plugin_path = self
            .plugins_path
            .join(plugin.instance.manifest.name.as_ref());
        let source_path = plugin_path.join("source");
        std::fs::create_dir_all(&source_path)?;

        let manifest_path = source_path.join("manifest.toml");
        let manifest_content =
            toml::to_string_pretty(&plugin.instance).map_err(Error::Serialize)?;
        std::fs::write(&manifest_path, manifest_content)?;

        Ok(true)
    }

    fn install(&mut self, plugin: ParsePlugin) -> Result<()> {
        let plugin_path = self
            .plugins_path
            .join(plugin.instance.manifest.name.as_ref());
        let source_path = plugin_path.join("source");
        std::fs::create_dir_all(&source_path)?;

        let manifest_path = source_path.join("manifest.toml");
        let manifest_content =
            toml::to_string_pretty(&plugin.instance).map_err(Error::Serialize)?;
        std::fs::write(&manifest_path, manifest_content)?;

        let code_path = source_path.join("code.wasm");
        std::fs::write(&code_path, &plugin.code)?;

        self.plugins.insert(
            plugin.instance.manifest.name.clone(),
            Plugin {
                instance: plugin.instance,
            },
        );
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::too_many_lines)]
    pub fn process_plugin_message(&mut self, plugin: &str, message: &api::Value) -> Result<bool> {
        let Some(plugin) = self.plugins.get(plugin) else {
            return Ok(false);
        };

        if !matches!(plugin.instance.state, PluginState::Enable {}) {
            return Ok(false);
        }

        let plugin_path = self
            .plugins_path
            .join(plugin.instance.manifest.name.as_ref());
        let source_path = plugin_path.join("source");
        let code_path = source_path.join("code.wasm");
        let wasm = std::fs::read(code_path)?;
        let module = Module::new(&self.engine, &wasm).to_error(&plugin.instance.manifest.name)?;

        let wasi_ctx = wasi_context(&plugin.instance.manifest.name);
        let host_state = HostState {
            wasi: wasi_ctx,
            calls: VecDeque::default(),
            call_error: None,
        };
        let mut store = Store::new(&self.engine, host_state);

        let mut linker = <Linker<HostState>>::new(&self.engine);

        wasmi_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi).map_err(Error::Wasi)?;

        let stride_emit = Func::wrap(
            &mut store,
            |mut caller: Caller<'_, HostState>, call_data: i32, call_len: i32| {
                let Some(memory_export) = caller.get_export("memory").and_then(Extern::into_memory)
                else {
                    caller.data_mut().call_error =
                        Some("plugin call requires exported memory".into());
                    return;
                };

                let call_data = usize::try_from(call_data).ok();
                let call_len = usize::try_from(call_len).ok();
                let Some(data) = call_data
                    .zip(call_len)
                    .and_then(|(start, len)| start.checked_add(len).map(|end| start..end))
                    .and_then(|range| memory_export.data(&caller).get(range))
                else {
                    caller.data_mut().call_error = Some("plugin call points outside memory".into());
                    return;
                };

                let Some(index) = data.iter().position(|&b| b == b':') else {
                    caller.data_mut().call_error = Some("plugin call missing prefix".into());
                    return;
                };

                let (method, data) = data.split_at(index);
                let data = &data[1..]; // skip the ':' separator

                let Some(method) = std::str::from_utf8(method).ok().map(Into::into) else {
                    caller.data_mut().call_error = Some("plugin call method not valid UTF8".into());
                    return;
                };

                match serde_json::from_slice::<api::Value>(data) {
                    Ok(call) => caller.data_mut().calls.push_back((method, call)),
                    Err(error) => caller.data_mut().call_error = Some(error.to_string().into()),
                }
            },
        );
        linker.define("env", "stride__emit", stride_emit)?;

        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .to_error(&plugin.instance.manifest.name)?;

        let stride_allocate = instance
            .get_typed_func::<i32, i32>(&store, "stride__allocate")
            .to_error(&plugin.instance.manifest.name)?;

        let stride_deallocate = instance
            .get_typed_func::<(i32, i32), ()>(&store, "stride__deallocate")
            .to_error(&plugin.instance.manifest.name)?;

        let stride_init = instance
            .get_typed_func::<(), ()>(&store, "stride__init")
            .to_error(&plugin.instance.manifest.name)?;

        let event_handler = instance
            .get_typed_func::<(i32, i32), i32>(&store, EVENT_HANDLER_NAME)
            .to_error(&plugin.instance.manifest.name)?;

        store
            .set_fuel(100_000)
            .to_error(&plugin.instance.manifest.name)?;
        stride_init
            .call(&mut store, ())
            .to_error(&plugin.instance.manifest.name)?;

        let event_data = serde_json::to_string(&message).expect("shouldn't fail");
        store
            .set_fuel(100_000)
            .to_error(&plugin.instance.manifest.name)?;
        let ret = stride_allocate
            .call(&mut store, event_data.len() as i32)
            .to_error(&plugin.instance.manifest.name)? as usize;

        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("already checked in validation");
        memory.data_mut(&mut store)[ret..ret + event_data.len()]
            .copy_from_slice(event_data.as_bytes());

        // TODO: Add computation limit.
        store
            .set_fuel(100_000_000_000)
            .to_error(&plugin.instance.manifest.name)?;
        event_handler
            .call(&mut store, (ret as i32, event_data.len() as i32))
            .to_error(&plugin.instance.manifest.name)?;

        if let Some(error) = store.data_mut().call_error.take() {
            return Err(Error::InvalidPluginCall(error));
        }

        stride_deallocate
            .call(&mut store, (ret as i32, event_data.len() as i32))
            .to_error(&plugin.instance.manifest.name)?;

        let plugin_name = plugin.instance.manifest.name.clone();
        let mut calls = std::mem::take(&mut store.data_mut().calls);
        for (method, call) in &mut calls {
            if matches!(
                method.as_ref(),
                api::StorageListMethod::NAME
                    | api::StorageGetMethod::NAME
                    | api::StorageSetMethod::NAME
                    | api::StorageRemoveMethod::NAME
            ) {
                if let Some(error) = self.method_permission_error(&plugin_name, method) {
                    return Err(Error::InvalidPluginCall(error));
                }
                sanitize_plugin_storage_call(&plugin_name, method, call)
                    .map_err(Error::InvalidPluginCall)?;
            }
        }

        self.plugin_calls
            .push_back(PluginCallQueue { plugin_name, calls });

        Ok(true)
    }
}
