//! Core component library of a stride plugin.

pub use stride_api::{self as api, constant::StorageErrorCode};

pub type PluginMessage = api::Value;

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn stride__emit(data: *const u8, len: usize);
    fn stride__storage_get(key: *const u8, key_len: usize, value: *mut u8) -> i32;
    fn stride__storage_set(
        key: *const u8,
        key_len: usize,
        value: *const u8,
        value_len: usize,
    ) -> i32;
    fn stride__storage_remove(key: *const u8, key_len: usize) -> i32;
}

/// Errors returned while accessing plugin storage.
#[derive(Debug)]
pub enum StorageError {
    Host(StorageErrorCode),
    UnknownHostCode(i32),
    Serde(serde_json::Error),
    ChangedSize { expected: usize, actual: usize },
}

impl StorageError {
    fn from_host_code(code: i32) -> Self {
        match code {
            value if value == StorageErrorCode::MissingMemoryExport as i32 => {
                Self::Host(StorageErrorCode::MissingMemoryExport)
            }
            value if value == StorageErrorCode::InvalidKey as i32 => {
                Self::Host(StorageErrorCode::InvalidKey)
            }
            value if value == StorageErrorCode::InvalidValue as i32 => {
                Self::Host(StorageErrorCode::InvalidValue)
            }
            value if value == StorageErrorCode::OutOfStorage as i32 => {
                Self::Host(StorageErrorCode::OutOfStorage)
            }
            value if value == StorageErrorCode::Permission as i32 => {
                Self::Host(StorageErrorCode::Permission)
            }
            _ => Self::UnknownHostCode(code),
        }
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Host(code) => write!(f, "plugin storage host error: {code:?}"),
            Self::UnknownHostCode(code) => write!(f, "unknown plugin storage host error: {code}"),
            Self::Serde(error) => write!(f, "plugin storage serialization error: {error}"),
            Self::ChangedSize { expected, actual } => write!(
                f,
                "plugin storage value changed size from {expected} to {actual} while reading"
            ),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<serde_json::Error> for StorageError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
    }
}

/// Access storage provided by the plugin host.
#[derive(Clone, Copy, Debug)]
pub struct Storage;

impl Storage {
    /// Retrieve a raw value, or `None` if the key does not exist.
    ///
    /// # Errors
    ///
    /// Returns a host error or a size-change error if the value cannot be read.
    pub fn get_raw(key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let size = unsafe { stride__storage_get(key.as_ptr(), key.len(), std::ptr::null_mut()) };
        if size == StorageErrorCode::NotFound as i32 {
            return Ok(None);
        }
        if size < 0 {
            return Err(StorageError::from_host_code(size));
        }

        let expected = usize::try_from(size).map_err(|_| StorageError::UnknownHostCode(size))?;
        let mut result = vec![0; expected];
        let actual = unsafe { stride__storage_get(key.as_ptr(), key.len(), result.as_mut_ptr()) };
        if actual < 0 {
            return Err(StorageError::from_host_code(actual));
        }
        let actual = usize::try_from(actual).map_err(|_| StorageError::UnknownHostCode(actual))?;
        if actual != expected {
            return Err(StorageError::ChangedSize { expected, actual });
        }
        Ok(Some(result))
    }

    /// Retrieve and deserialize a JSON value.
    ///
    /// # Errors
    ///
    /// Returns a host error or a JSON deserialization error.
    pub fn get<T: serde::de::DeserializeOwned>(key: &str) -> Result<Option<T>, StorageError> {
        Self::get_raw(key)?
            .map(|content| serde_json::from_slice(&content).map_err(StorageError::from))
            .transpose()
    }

    /// Store a raw value.
    #[must_use]
    pub fn set_raw(key: &str, value: &[u8]) -> bool {
        unsafe { stride__storage_set(key.as_ptr(), key.len(), value.as_ptr(), value.len()) == 0 }
    }

    /// Serialize and store a JSON value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value cannot be serialized.
    pub fn set<T: serde::Serialize>(key: &str, value: &T) -> Result<bool, StorageError> {
        let json = serde_json::to_vec(value)?;
        Ok(Self::set_raw(key, &json))
    }

    /// Remove a stored value; returns whether a value was removed.
    #[must_use]
    pub fn remove(key: &str) -> bool {
        unsafe { stride__storage_remove(key.as_ptr(), key.len()) == 1 }
    }
}

/// Stateful plugin implementation used by [`plugin!`].
pub trait Plugin: Sized {
    fn init() -> Self;
    fn event(&mut self, event: PluginMessage) -> bool;
}

/// Export the plugin ABI for a [`Plugin`] implementation.
#[macro_export]
macro_rules! plugin {
    ($plugin:ty) => {
        static PLUGIN_INSTANCE: std::sync::LazyLock<std::sync::Mutex<$plugin>> =
            std::sync::LazyLock::new(|| std::sync::Mutex::new(<$plugin as $crate::Plugin>::init()));

        #[allow(unreachable_pub)]
        #[unsafe(no_mangle)]
        pub extern "C" fn stride__init() {
            let _ = &*PLUGIN_INSTANCE;
        }

        #[allow(unreachable_pub)]
        #[unsafe(no_mangle)]
        pub extern "C" fn stride__allocate(size: usize) -> *mut u8 {
            Box::into_raw(vec![0; size].into_boxed_slice()).cast()
        }

        /// # Safety
        /// `pointer` must have been returned by `stride__allocate` with the same size.
        #[allow(unreachable_pub)]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn stride__deallocate(pointer: *mut u8, size: usize) {
            let allocation = std::ptr::slice_from_raw_parts_mut(pointer, size);
            unsafe {
                drop(Box::from_raw(allocation));
            }
        }

        /// # Safety
        /// `data` must point to `len` readable bytes.
        #[allow(unreachable_pub)]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn stride__event_handler(data: *const u8, len: usize) -> i32 {
            let data = unsafe { std::slice::from_raw_parts(data, len) };
            let event: $crate::PluginMessage =
                serde_json::from_slice(data).expect("host passed an invalid plugin message");
            let mut plugin = PLUGIN_INSTANCE
                .lock()
                .expect("couldn't lock plugin instance");
            i32::from(<$plugin as $crate::Plugin>::event(&mut *plugin, event))
        }
    };
}

#[derive(serde::Serialize)]
struct MethodCall<'a, M> {
    method: &'static str,
    params: &'a M,
}

/// Emit a schema-backed API method call to the host.
///
/// # Errors
///
/// Returns an error if the method cannot be serialized.
pub fn emit_method<M: api::Method>(method: &M) -> Result<(), serde_json::Error> {
    let call = MethodCall {
        method: M::NAME,
        params: method,
    };
    let data = serde_json::to_vec(&call)?;

    unsafe {
        stride__emit(data.as_ptr(), data.len());
    }
    Ok(())
}
