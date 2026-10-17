use std::{
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, Mutex, OnceLock},
};

use stride_api as api;
use stride_backend_git::{
    GitBackend, SshHostListHandler, SshHostRemoveHandler, SshKeyAddHandler, SshKeyGenerateHandler,
    SshKeyListHandler, SshKeyRemoveHandler, SshKeyUpdateHandler, method::SshHostAddHandler,
};
use stride_engine::{Backend, Engine, KnownPaths, Notifier};

use crate::{
    ErrorKind, RustError,
    api::settings::{application_cache_path, application_support_path},
    frb_generated::StreamSink,
};

pub(crate) static ENGINE: OnceLock<Arc<Engine>> = OnceLock::new();
static STREAM: LazyLock<Mutex<Option<StreamSink<String>>>> = LazyLock::new(Mutex::default);

#[derive(Debug)]
struct FlutterNotifier;

impl Notifier for FlutterNotifier {
    fn notify(
        &self,
        _: &Arc<Engine>,
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

pub fn create_context(stream: StreamSink<String>, support_path: String, cache_path: String) {
    let known_paths = KnownPaths::new(PathBuf::from(&support_path), PathBuf::from(cache_path));

    // HACK: We need an ~/.ssh directory for libssh2. On android and iOS
    // that means redefining the HOME dir, so that we can control
    // it. <2025-02-25>
    //
    // SAFETY: We do this initially when the applicattion is initialized,
    //         so this should be thread-safe.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    unsafe {
        std::env::set_var("HOME", &support_path);
    }

    // TODO: move this to ssh.key methods
    let ssh_path = Path::new(&support_path).join(".ssh");
    std::fs::create_dir_all(&ssh_path).expect("should not fail");

    stride_logging::init(&Path::new(&support_path).join("logs").join("log.txt"));

    ENGINE.get_or_init(|| {
        Engine::builder(known_paths)
            .expect("unable to build engine")
            .notifier(Box::new(FlutterNotifier))
            .insert_default_methods()
            .backend(GitBackend::handler())
            .command("ssh.host.add", SshHostAddHandler)
            .command("ssh.host.list", SshHostListHandler)
            .command("ssh.host.remove", SshHostRemoveHandler)
            .command("ssh.key.list", SshKeyListHandler)
            .command("ssh.key.generate", SshKeyGenerateHandler)
            .command("ssh.key.add", SshKeyAddHandler)
            .command("ssh.key.update", SshKeyUpdateHandler)
            .command("ssh.key.remove", SshKeyRemoveHandler)
            .build()
    });

    let mut stream_lock = STREAM.lock().unwrap();
    *stream_lock = Some(stream);
}

pub fn execute(method: &str, args: &str) -> Result<String, RustError> {
    #[derive(Debug, serde::Deserialize)]
    struct Params {
        params: api::Value,
    }

    let params: Params = serde_json::from_str(args).map_err(|e| ErrorKind::Other {
        message: format!("Failed to parse args: {e}").into(),
    })?;

    let result = ENGINE
        .wait()
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
