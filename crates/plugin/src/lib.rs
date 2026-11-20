//! Core component library of a stride plugin.

pub use stride_api::{self as api};

pub type PluginMessage = api::Value;

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn stride__emit(data: *const u8, len: usize);
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

/// Emit a schema-backed API method call to the host.
///
/// # Errors
///
/// Returns an error if the method cannot be serialized.
pub fn emit_method<M: api::Method>(method: &M) -> Result<(), serde_json::Error> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(M::NAME.as_bytes());
    buffer.push(b':');
    serde_json::to_writer(&mut buffer, &method)?;

    unsafe {
        stride__emit(buffer.as_ptr(), buffer.len());
    }
    Ok(())
}

#[derive(Debug)]
pub enum PluginStorageError {
    InvalidKey(Box<str>),
    Serialize(serde_json::Error),
}

impl std::fmt::Display for PluginStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidKey(key) => write!(f, "invalid plugin storage key '{key}'"),
            Self::Serialize(error) => {
                write!(f, "could not serialize plugin storage method: {error}")
            }
        }
    }
}

impl std::error::Error for PluginStorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidKey(_) => None,
            Self::Serialize(error) => Some(error),
        }
    }
}

impl From<serde_json::Error> for PluginStorageError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialize(error)
    }
}

/// Plugin-scoped access to global or repository storage.
///
/// Keys are unqualified; the plugin manager assigns each plugin its own group.
#[derive(Debug, Clone, Copy, Default)]
pub struct PluginStorage {
    repository_id: Option<uuid::Uuid>,
}

impl PluginStorage {
    /// Uses the application's global storage.
    #[must_use]
    pub const fn global() -> Self {
        Self {
            repository_id: None,
        }
    }

    /// Uses storage belonging to the given repository.
    #[must_use]
    pub const fn repository(repository_id: uuid::Uuid) -> Self {
        Self {
            repository_id: Some(repository_id),
        }
    }

    /// Emits a request to list this plugin's storage items.
    ///
    /// # Errors
    ///
    /// Returns an error if the method cannot be serialized.
    pub fn list(&self) -> Result<(), PluginStorageError> {
        emit_method(&api::StorageListMethod {
            repository_id: self.repository_id,
            group: None,
        })?;
        Ok(())
    }

    /// Emits a request to retrieve the given unqualified keys.
    ///
    /// # Errors
    ///
    /// Returns an error if a key is invalid or the method cannot be serialized.
    pub fn get<K: AsRef<str>>(&self, keys: &[K]) -> Result<(), PluginStorageError> {
        let items = keys
            .iter()
            .map(|key| unqualified_key(key.as_ref()))
            .collect::<Result<Vec<_>, _>>()?;
        emit_method(&api::StorageGetMethod {
            repository_id: self.repository_id,
            items,
        })?;
        Ok(())
    }

    /// Emits a request to set an unqualified key.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is invalid or the method cannot be serialized.
    pub fn set(&self, key: &str, value: api::ConfigValue) -> Result<(), PluginStorageError> {
        let key = unqualified_key(key)?;
        emit_method(&api::StorageSetMethod {
            repository_id: self.repository_id,
            key,
            value,
        })?;
        Ok(())
    }

    /// Emits a request to remove the given unqualified keys.
    ///
    /// # Errors
    ///
    /// Returns an error if a key is invalid or the method cannot be serialized.
    pub fn remove<K: AsRef<str>>(&self, keys: &[K]) -> Result<(), PluginStorageError> {
        let items = keys
            .iter()
            .map(|key| unqualified_key(key.as_ref()))
            .collect::<Result<Vec<_>, _>>()?;
        emit_method(&api::StorageRemoveMethod {
            repository_id: self.repository_id,
            items,
        })?;
        Ok(())
    }
}

fn unqualified_key(key: &str) -> Result<Box<str>, PluginStorageError> {
    if key.is_empty()
        || key.contains(':')
        || !key.split('.').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        })
    {
        return Err(PluginStorageError::InvalidKey(key.into()));
    }
    Ok(key.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_storage_keys_must_be_unqualified_names() {
        assert!(unqualified_key("settings.theme").is_ok());
        for key in ["", "group:key", "a..b", "bad key", "é"] {
            assert!(unqualified_key(key).is_err(), "{key}");
        }
    }
}
