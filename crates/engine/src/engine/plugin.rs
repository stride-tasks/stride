use std::sync::{
    Arc, Mutex, MutexGuard, Weak,
    mpsc::{self, Receiver, Sender},
};

use stride_api as api;
use stride_plugin_manager::PluginManager;

use crate::{Engine, Error, Result};

#[derive(Debug)]
pub(super) struct PluginRuntime {
    manager: Mutex<PluginManager>,
    sender: Sender<PluginMessage>,
}

#[derive(Debug)]
struct PluginWork {
    name: Box<str>,
    value: api::Value,
}

#[derive(Debug)]
enum PluginMessage {
    Notification(PluginWork),
    Barrier(Sender<()>),
}

pub(super) struct PluginWorker {
    receiver: Receiver<PluginMessage>,
}

impl PluginRuntime {
    pub(super) fn new(manager: PluginManager) -> (Self, PluginWorker) {
        let (sender, receiver) = mpsc::channel();
        (
            Self {
                manager: Mutex::new(manager),
                sender,
            },
            PluginWorker { receiver },
        )
    }

    pub(super) fn start_worker(&self, engine: &Arc<Engine>, worker: PluginWorker) {
        let weak_engine = Arc::downgrade(engine);
        std::thread::Builder::new()
            .name("stride-plugin-messages".into())
            .spawn(move || worker.run(weak_engine))
            .expect("failed to start plugin message worker");
    }

    pub(super) fn lock_manager(&self) -> MutexGuard<'_, PluginManager> {
        self.manager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(super) fn notify(&self, name: &str, value: api::Value) -> Result<()> {
        self.sender
            .send(PluginMessage::Notification(PluginWork {
                name: name.into(),
                value,
            }))
            .map_err(|_| Error::PluginWorkerStopped)
    }

    pub(super) fn wait_for_messages(&self) -> Result<()> {
        let (sender, receiver) = mpsc::channel();
        self.sender
            .send(PluginMessage::Barrier(sender))
            .map_err(|_| Error::PluginWorkerStopped)?;
        receiver.recv().map_err(|_| Error::PluginWorkerStopped)
    }
}

impl PluginWorker {
    fn run(self, engine: Weak<Engine>) {
        while let Ok(message) = self.receiver.recv() {
            match message {
                PluginMessage::Notification(work) => {
                    let Some(engine) = engine.upgrade() else {
                        break;
                    };
                    if let Err(error) = process_plugin_work(&engine, work) {
                        log::error!("failed to process plugin message: {error}");
                    }
                }
                PluginMessage::Barrier(sender) => {
                    let _ = sender.send(());
                }
            }
        }
    }
}

fn process_plugin_work(engine: &Arc<Engine>, work: PluginWork) -> Result<()> {
    engine
        .lock_plugin_manager()
        .emit_notification(&work.name, &work.value)?;

    loop {
        let calls = {
            let mut manager = engine.lock_plugin_manager();
            loop {
                match manager.process_plugin_message() {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(error) => log::error!("plugin execution failed: {error}"),
                }
            }
            std::iter::from_fn(|| manager.process_plugin_call()).collect::<Vec<_>>()
        };

        if calls.is_empty() {
            break;
        }

        for (plugin_name, call) in calls {
            let (method, params) = match parse_plugin_method(call) {
                Ok(call) => call,
                Err(reason) => {
                    disable_plugin(engine, &plugin_name, reason)?;
                    continue;
                }
            };

            let permission_error = {
                let manager = engine.lock_plugin_manager();
                if manager.plugin(&plugin_name).is_none() {
                    continue;
                }
                manager.method_permission_error(&plugin_name, &method)
            };
            if let Some(reason) = permission_error {
                disable_plugin(engine, &plugin_name, reason)?;
                continue;
            }

            let result = match engine.execute_erased(&method, params) {
                Ok(result) => result,
                Err(error) => {
                    log::error!("plugin {plugin_name} method {method} failed: {error}");
                    continue;
                }
            };

            let mut response = api::Map::default();
            response.insert("method".into(), api::Value::String(method.clone()));
            response.insert("result".into(), result);
            engine
                .lock_plugin_manager()
                .enqueue_plugin_message(&plugin_name, api::Value::Map(response));
        }
    }

    Ok(())
}

fn parse_plugin_method(call: api::Value) -> std::result::Result<(Box<str>, api::Value), Box<str>> {
    let api::Value::Map(mut call) = call else {
        return Err("plugin call must be an API method object".into());
    };
    let Some(api::Value::String(method)) = call.shift_remove("method") else {
        return Err("plugin call is missing a string method name".into());
    };
    let Some(params) = call.shift_remove("params") else {
        return Err("plugin call is missing params".into());
    };
    Ok((method, params))
}

fn disable_plugin(engine: &Arc<Engine>, plugin_name: &str, reason: Box<str>) -> Result<()> {
    log::error!("disabling plugin {plugin_name}: {reason}");
    engine
        .lock_plugin_manager()
        .disable(plugin_name, Some(reason))?;
    Ok(())
}
