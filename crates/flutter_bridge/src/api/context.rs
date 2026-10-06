use std::sync::{Arc, LazyLock, Mutex, OnceLock};

use stride_api as api;
use stride_backend_git::method::SshHostAddHandler;
use stride_engine::{Engine, Notifier};

use crate::{
    ErrorKind, RustError,
    frb_generated::StreamSink,
    method::{RepositoryProjectListHandler, RepositorySyncHandler, RepositoryTagListHandler},
};

static STATE: OnceLock<Arc<Engine>> = OnceLock::new();
static STREAM: LazyLock<Mutex<Option<StreamSink<String>>>> = LazyLock::new(Mutex::default);

#[derive(Debug)]
struct FlutterNotifier;

impl Notifier for FlutterNotifier {
    fn notify(
        &self,
        _: Arc<Engine>,
        notification: Box<dyn stride_engine::Notification>,
    ) -> stride_engine::Result<()> {
        let name = notification.name();
        let value = notification.to_value();
        let map = serde_json::json!({
            "method": name,
            "params": value,
        });

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

pub fn execute(method: &str, args: &str) -> Result<String, RustError> {
    #[derive(Debug, serde::Deserialize)]
    struct Params {
        params: api::Value,
    }

    let context = STATE.get_or_init(|| {
        Engine::builder()
            .notifier(Box::new(FlutterNotifier))
            .command("repository.sync", RepositorySyncHandler)
            .command("repository.tag.list", RepositoryTagListHandler)
            .command("repository.project.list", RepositoryProjectListHandler)
            .command("ssh.host.add", SshHostAddHandler)
            .build()
    });

    let params: Params = serde_json::from_str(args).map_err(|e| ErrorKind::Other {
        message: format!("Failed to parse args: {e}").into(),
    })?;

    let result = context
        .clone()
        .execute_erased(method, params.params)
        .map_err(|err| ErrorKind::Other {
            message: format!("Failed to execute method: {method} with args: {args}. Error: {err}")
                .into(),
        })?;

    let result = serde_json::to_string(&result).map_err(|e| ErrorKind::Other {
        message: format!("Failed to serialize result: {e}").into(),
    })?;

    Ok(result)
}
