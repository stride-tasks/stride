//! Stride flutter bindings.

use std::{
    any::Any,
    ffi::{CStr, CString, c_char},
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    ptr,
    sync::{Arc, Mutex, PoisonError, RwLock},
};

use stride_api as api;
use stride_backend_git::{
    GitBackend, SshHostListHandler, SshHostRemoveHandler, SshKeyAddHandler, SshKeyGenerateHandler,
    SshKeyListHandler, SshKeyRemoveHandler, SshKeyUpdateHandler, method::SshHostAddHandler,
};
use stride_engine::{Backend, Engine, KnownPaths, Notifier};

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid argument: {message}")]
    InvalidArgument { message: Box<str> },
    #[error("Rust context has not been initialized")]
    NotInitialized,
    #[error("{0}")]
    Engine(#[from] stride_engine::Error),
    #[error("other error: {message}")]
    Other { message: Box<str> },
}

/// Stable machine-readable error code that the Dart side maps to `RustErrorKind`.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum ErrorCode {
    Io,
    InvalidArgument,
    NotInitialized,
    MethodNotFound,
    MethodFailed,
    Other,
    Panic,
}

impl ErrorKind {
    fn code(&self) -> ErrorCode {
        match self {
            Self::Io(_) => ErrorCode::Io,
            Self::InvalidArgument { .. } => ErrorCode::InvalidArgument,
            Self::NotInitialized => ErrorCode::NotInitialized,
            Self::Engine(stride_engine::Error::HandlerNotFound { .. }) => ErrorCode::MethodNotFound,
            Self::Engine(_) => ErrorCode::MethodFailed,
            Self::Other { .. } => ErrorCode::Other,
        }
    }
}

#[derive(Debug)]
pub struct RustError {
    /// Reduce the size of Error because it's passed around in a lot of functions.
    repr: Box<ErrorKind>,
}

impl RustError {
    fn code(&self) -> ErrorCode {
        self.repr.code()
    }
}

impl std::error::Error for RustError {}
impl std::fmt::Display for RustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.repr.fmt(f)
    }
}

impl<T: Into<ErrorKind>> From<T> for RustError {
    fn from(value: T) -> Self {
        Self {
            repr: Box::new(value.into()),
        }
    }
}

type StreamCallback = extern "C" fn(*mut c_char);
type OptionalStreamCallback = Option<StreamCallback>;

static ENGINE: RwLock<Option<Arc<Engine>>> = RwLock::new(None);
static STREAM_CALLBACK: Mutex<OptionalStreamCallback> = Mutex::new(None);

#[derive(Debug)]
struct FlutterNotifier;

impl Notifier for FlutterNotifier {
    fn notify(&self, _: &Arc<Engine>, notification: Box<dyn stride_engine::Notification>) {
        let name = notification.name();
        let value = notification.to_value();
        let map = serde_json::json!({
            "method": name,
            "params": value,
        });

        // The lock is held during the call so that `stride__clear_stream_callback`
        // returns only after no call to the callback is in flight.
        let guard = STREAM_CALLBACK
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(callback) = *guard {
            let message = serde_json::to_string(&map)
                .map_err(|error| stride_engine::Error::from(Box::new(error)))
                .expect("method serialization should not fail");
            let message = CString::new(message)
                .map_err(|error| stride_engine::Error::from(Box::new(error)))
                .expect("serialized JSON should not contain any null character");
            callback(message.into_raw());
        }
    }
}

fn create_context(
    callback: OptionalStreamCallback,
    support_path: &Path,
    cache_path: &Path,
) -> Result<(), RustError> {
    let known_paths = KnownPaths::new(PathBuf::from(&support_path), PathBuf::from(&cache_path));

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

    let ssh_path = support_path.join(".ssh");
    std::fs::create_dir_all(&ssh_path)?;

    stride_logging::init(&support_path.join("logs").join("log.txt"));

    if let Some(callback) = callback {
        *STREAM_CALLBACK
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(callback);
    }

    // Acquire a write lock on the ENGINE to replace the previous engine.
    let mut lock = ENGINE.write().unwrap_or_else(PoisonError::into_inner);

    let engine = Engine::builder(known_paths)
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
        .build();

    // Replaces the previous engine; calls already running keep using theirs.
    *lock = Some(engine);

    Ok(())
}

fn execute_inner(method: &str, args: &str) -> Result<String, RustError> {
    #[derive(Debug, serde::Deserialize)]
    struct Params {
        params: api::Value,
    }

    let params: Params = serde_json::from_str(args).map_err(|e| ErrorKind::InvalidArgument {
        message: format!("Failed to parse args: {e}").into(),
    })?;

    let engine = ENGINE
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
        .ok_or(ErrorKind::NotInitialized)?;

    // The error deliberately does not repeat `args`: they can contain secrets.
    let result = engine.execute_erased(method, params.params)?;

    let result = serde_json::to_string(&result).map_err(|e| ErrorKind::Other {
        message: format!("Failed to serialize result: {e}").into(),
    })?;

    Ok(result)
}

/// # Safety
///
/// `value` must be a valid null-terminated UTF-8 string.
unsafe fn read_string(value: *const c_char, name: &str) -> Result<String, RustError> {
    if value.is_null() {
        return Err(ErrorKind::InvalidArgument {
            message: format!("{name} must not be null").into(),
        }
        .into());
    }

    // SAFETY: The caller passes a valid NUL-terminated UTF-8 string.
    let value = unsafe { CStr::from_ptr(value) };
    value.to_str().map(str::to_owned).map_err(|error| {
        ErrorKind::InvalidArgument {
            message: format!("{name} is not valid UTF-8: {error}").into(),
        }
        .into()
    })
}

fn panic_message(payload: &(dyn Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_owned()
    } else {
        "Rust bridge operation panicked".to_owned()
    }
}

fn into_raw_string(value: &str) -> *mut c_char {
    CString::new(value.replace('\0', "\\0"))
        .expect("FFI strings must not contain NUL bytes")
        .into_raw()
}

fn report_create_stream_error(
    result: Result<Result<(), RustError>, Box<dyn Any + Send>>,
) -> *mut c_char {
    match result {
        Ok(Ok(())) => ptr::null_mut(),
        Ok(Err(error)) => into_raw_string(&error.to_string()),
        Err(payload) => into_raw_string(&panic_message(&*payload)),
    }
}

/// Initializes the engine and registers an optional notification callback.
/// Returns a library-owned error string, or null when initialization succeeds.
///
/// # Safety
///
/// `support_path` and `cache_path` must be valid null-terminated UTF-8 strings.
/// `callback` must be a valid function pointer or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stride__create_stream(
    support_path: *const c_char,
    cache_path: *const c_char,
    callback: OptionalStreamCallback,
) -> *mut c_char {
    let result = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller must ensure that `support_path` is a valid null-terminated UTF-8 string.
        let support_path = unsafe { read_string(support_path, "support_path") }?;

        // SAFETY: The caller must ensure that `cache_path` is a valid null-terminated UTF-8 string.
        let cache_path = unsafe { read_string(cache_path, "cache_path") }?;

        create_context(callback, Path::new(&support_path), Path::new(&cache_path))
    }));
    report_create_stream_error(result)
}

/// Executes an API method and returns a JSON response containing `ok` and
/// either `result` or `error`.
///
/// # Safety
///
/// `method` and `args` must be valid null-terminated UTF-8 strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stride__execute(
    method: *const c_char,
    args: *const c_char,
) -> *mut c_char {
    let result = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller must ensure that `method` and `args` are valid null-terminated UTF-8 strings.
        let method = unsafe { read_string(method, "method") }?;

        // SAFETY: The caller must ensure that `args` is a valid null-terminated UTF-8 string.
        let args = unsafe { read_string(args, "args") }?;

        execute_inner(&method, &args)
    }));

    let response = match result {
        Ok(Ok(result)) => serde_json::json!({"ok": true, "result": result}),
        Ok(Err(error)) => {
            serde_json::json!({"ok": false, "kind": error.code(), "error": error.to_string()})
        }
        Err(payload) => {
            serde_json::json!({"ok": false, "kind": ErrorCode::Panic, "error": panic_message(&*payload)})
        }
    };
    into_raw_string(&response.to_string())
}

/// Unregisters the notification callback. When this returns, the callback is
/// no longer being called and can be released.
#[unsafe(no_mangle)]
pub extern "C" fn stride__clear_stream_callback() {
    *STREAM_CALLBACK
        .lock()
        .unwrap_or_else(PoisonError::into_inner) = None;
}

/// Frees a string allocated by this library.
///
/// # Safety
/// `value` must be null or a string returned by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stride__free_string(value: *mut c_char) {
    if !value.is_null() {
        // SAFETY: This pointer must come from `CString::into_raw` in this library.
        drop(unsafe { CString::from_raw(value) });
    }
}
