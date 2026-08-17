use std::sync::{Arc, LazyLock, Mutex, OnceLock};

use stride_api::{self as api, PROMPT_METHOD};
use stride_backend_git::method::SshHostAddHandler;
use stride_engine as engine;
use uuid::Uuid;

use crate::{ErrorKind, RustError, api::repository::Repository, frb_generated::StreamSink};

static STATE: OnceLock<Arc<dyn api::Context + Send + Sync>> = OnceLock::new();
static STREAM: LazyLock<Mutex<Option<StreamSink<String>>>> = LazyLock::new(Mutex::default);

#[derive(Debug)]
struct RepositorySyncHandler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RepositorySpec {
    pub(crate) id: Uuid,
}

impl api::CommandHandler for RepositorySyncHandler {
    fn handle(&self, context: Arc<dyn api::Context>, args: api::Value) -> api::Result<Box<str>> {
        let args = serde_json::to_string(&args).map_err(Box::new)?;
        let repository: RepositorySpec = serde_json::from_str(&args).map_err(Box::new)?;

        let mut repository = Repository::open(repository.id).map_err(Box::new)?;
        repository.sync(&context).map_err(Box::new)?;
        Ok("".into())
    }
}

#[derive(Debug)]
struct FlutterNotifier;

impl api::Notifier for FlutterNotifier {
    fn notify(&self, _: Arc<dyn api::Context>, notification: api::Notification) -> api::Result<()> {
        let map = match notification {
            api::Notification::Prompt(prompt) => {
                let map = serde_json::json!({
                    "method": PROMPT_METHOD,
                    "params": {
                        "target": prompt.target(),
                        "inputs": prompt.inputs(),
                        "summary": prompt.summary(),
                        "description": prompt.description(),
                    }
                });
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

    println!("Executing method: {method} with args: {args}");

    let params: Params = serde_json::from_str(args).map_err(|e| ErrorKind::Other {
        message: format!("Failed to parse args: {e}").into(),
    })?;

    context
        .clone()
        .execute(method, params.params)
        .map_err(|err| {
            ErrorKind::Other {
                message: format!(
                    "Failed to execute method: {method} with args: {args}. Error: {err}"
                )
                .into(),
            }
            .into()
        })
}
