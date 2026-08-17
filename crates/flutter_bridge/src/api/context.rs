use std::sync::{Arc, LazyLock, Mutex, OnceLock};

use stride_api::{self as api, PROMPT_METHOD};
use stride_backend_git::known_hosts::{Host, KnownHosts};
use stride_engine as engine;
use uuid::Uuid;

use crate::{
    ErrorKind, RustError,
    api::repository::Repository,
    frb_generated::StreamSink,
};

static STATE: OnceLock<Arc<dyn api::Context + Send + Sync>> = OnceLock::new();
static STREAM: LazyLock<Mutex<Option<StreamSink<String>>>> = LazyLock::new(Mutex::default);

#[derive(Debug)]
struct RepositorySyncHandler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RepositorySpec {
    pub(crate) id: Uuid,
}

impl api::CommandHandler for RepositorySyncHandler {
    fn handle(&self, context: Arc<dyn api::Context>, args: &str) -> api::Result<Box<str>> {
        let repository: RepositorySpec = serde_json::from_str(args).unwrap();

        let mut repository = Repository::open(repository.id).map_err(Box::new)?;
        repository.sync(&context).map_err(Box::new)?;
        Ok("".into())
    }
}

#[derive(Debug)]
struct SshHostAddHandler;

#[derive(Debug, serde::Deserialize)]
struct SshHostAddArgs {
    host: Host,
}

impl api::CommandHandler for SshHostAddHandler {
    fn handle(&self, _: Arc<dyn api::Context>, args: &str) -> api::Result<Box<str>> {
        let ssh_host_add_args: SshHostAddArgs = serde_json::from_str(args).map_err(Box::new)?;
        let mut known_hosts = KnownHosts::read_standard_file().map_err(Box::new)?;
        known_hosts.add(ssh_host_add_args.host);
        known_hosts.write_standard_file().map_err(Box::new)?;
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
        params: serde_json::Value,
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
    let params = serde_json::to_string(&params.params).map_err(|e| ErrorKind::Other {
        message: format!("Failed to serialize params: {e}").into(),
    })?;

    context.clone().execute(method, &params).map_err(|err| {
        ErrorKind::Other {
            message: format!("Failed to execute method: {method} with args: {args}. Error: {err}")
                .into(),
        }
        .into()
    })
}
