use std::sync::{Arc, LazyLock, Mutex, OnceLock};

use stride_api as api;
use stride_backend_git::method::SshHostAddHandler;
use stride_engine as engine;

use crate::{ErrorKind, RustError, frb_generated::StreamSink, method::RepositorySyncHandler};

static STATE: OnceLock<Arc<dyn api::Context + Send + Sync>> = OnceLock::new();
static STREAM: LazyLock<Mutex<Option<StreamSink<String>>>> = LazyLock::new(Mutex::default);

#[derive(Debug)]
struct FlutterNotifier;

impl api::Notifier for FlutterNotifier {
    fn notify(&self, _: Arc<dyn api::Context>, notification: api::Notification) -> api::Result<()> {
        let map = match notification {
            api::Notification::Prompt(prompt) => {
                let map = serde_json::json!({
                    "method": api::PROMPT_METHOD,
                    "params": {
                        "target": prompt.target(),
                        "inputs": prompt.inputs(),
                        "summary": prompt.summary(),
                        "description": prompt.description(),
                    }
                });
                map
            }
            api::Notification::RepositoryChanged(changed) => {
                let map = serde_json::json!({
                    "method": "stride.notification.repository.changed",
                    "params": changed,
                });
                println!("{}", serde_json::to_string_pretty(&map).unwrap());
                map
            }
        };

        STREAM.clear_poison();
        let mut lock = STREAM.lock().unwrap();
        if let Some(stream) = lock.as_mut() {
            let result = stream.add(serde_json::to_string(&map).unwrap());
            drop(result);
        }
        Ok(())
    }
}

pub fn create_context(stream: StreamSink<String>) {
    let mut stream_lock = STREAM.lock().unwrap();
    *stream_lock = Some(stream);
}

pub fn execute(method: &str, args: &str) -> Result<(), RustError> {
    #[derive(Debug, serde::Deserialize)]
    struct Params {
        params: api::Value,
    }

    let context = STATE.get_or_init(|| {
        engine::EngineBuilder::new()
            .notifier(Box::new(FlutterNotifier))
            .command("stride.repository.sync", RepositorySyncHandler)
            .command("stride.ssh.host.add", SshHostAddHandler)
            .build()
    });

    let params: Params = serde_json::from_str(args).map_err(|e| ErrorKind::Other {
        message: format!("Failed to parse args: {e}").into(),
    })?;

    let result = context
        .clone()
        .execute(method, params.params)
        .map_err(|err| ErrorKind::Other {
            message: format!("Failed to execute method: {method} with args: {args}. Error: {err}")
                .into(),
        })?;

    let _result = serde_json::to_string(&result).map_err(|e| ErrorKind::Other {
        message: format!("Failed to serialize result: {e}").into(),
    })?;

    Ok(())
}
