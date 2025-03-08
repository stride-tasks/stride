use std::marker::PhantomData;

use stride_core::constant::StorageErrorCode;
use stride_core::event::{HostEvent, PluginEvent};

pub use stride_core::event;
pub use stride_core::task;

fn default_event_handler(_event: HostEvent) -> bool {
    false
}

pub type EventHandler = fn(event: HostEvent) -> bool;

pub static mut EVENT_HANDLER: EventHandler = default_event_handler;

#[no_mangle]
pub extern "C" fn stride__allocate(size: usize) -> *mut core::ffi::c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    pointer
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn stride__deallocate(pointer: *mut core::ffi::c_void, capacity: usize) {
    unsafe {
        drop(Vec::from_raw_parts(pointer, 0, capacity));
    }
}

/// # Safety
#[no_mangle]
#[allow(clippy::missing_panics_doc)]
pub unsafe extern "C" fn stride__event_handler(event: *const u8, event_len: usize) -> bool {
    let event_data = unsafe { core::slice::from_raw_parts(event, event_len) };
    let event: HostEvent =
        serde_json::from_slice(event_data).expect("data passed from host should be valid");

    unsafe { EVENT_HANDLER(event) }
}

unsafe extern "C" {
    pub fn stride__emit(event: *const u8, event_len: usize);

    pub fn stride__storage_get(key: *const u8, key_len: usize, value: *mut u8) -> i32;
    pub fn stride__storage_set(
        key: *const u8,
        key_len: usize,
        value: *const u8,
        value_len: usize,
    ) -> i32;
    pub fn stride__storage_remove(key: *const u8, key_len: usize) -> i32;
}

pub trait Plugin {
    fn init() -> Self;
    fn event(&mut self, event: HostEvent) -> bool;
}

#[macro_export]
macro_rules! plugin {
    ($plugin:ty) => {
        #[allow(non_snake_case)]
        /// # Safety
        #[no_mangle]
        pub unsafe extern "C" fn stride__init() {
            pub static PLUGIN_INSTANCE: std::sync::LazyLock<std::sync::Mutex<$plugin>> =
                std::sync::LazyLock::new(|| std::sync::Mutex::new(<$plugin>::init()));

            unsafe {
                $crate::EVENT_HANDLER = |event: $crate::event::HostEvent| {
                    PLUGIN_INSTANCE
                        .lock()
                        .expect("couldn't lock plugin instance")
                        .event(event)
                };
            }
        }
    };
}

#[allow(clippy::missing_panics_doc)]
pub fn emit(event: &PluginEvent) {
    let event = serde_json::to_string(event).expect("should not fail");

    unsafe {
        stride__emit(event.as_bytes().as_ptr(), event.len());
    }
}

#[derive(Debug)]
pub enum StorageError {
    Host(StorageErrorCode),
    Serde(serde_json::Error),
}

#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct Storage {
    _phantom: PhantomData<()>,
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_sign_loss)]
impl Storage {
    pub fn get_raw(key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let size = unsafe { stride__storage_get(key.as_ptr(), key.len(), std::ptr::null_mut()) };
        if size == StorageErrorCode::NotFound as i32 {
            return Ok(None);
        }
        if size < 0 {
            return Err(StorageError::Host(StorageErrorCode::from(size)));
        }
        let mut result = vec![0u8; size as usize];
        let second_size =
            unsafe { stride__storage_get(key.as_ptr(), key.len(), result.as_mut_ptr()) };
        assert_eq!(size, second_size);
        Ok(Some(result))
    }

    pub fn get<T: serde::de::DeserializeOwned>(key: &str) -> Result<Option<T>, StorageError> {
        let Some(content) = Self::get_raw(key)? else {
            return Ok(None);
        };
        serde_json::from_slice(&content)
            .map(Some)
            .map_err(StorageError::Serde)
    }

    #[must_use]
    pub fn set_raw(key: &str, value: &[u8]) -> bool {
        let retval =
            unsafe { stride__storage_set(key.as_ptr(), key.len(), value.as_ptr(), value.len()) };
        retval == 0
    }

    pub fn set<T: serde::Serialize>(key: &str, value: &T) -> Result<bool, StorageError> {
        let json = serde_json::to_string(value).map_err(StorageError::Serde)?;
        Ok(Self::set_raw(key, json.as_bytes()))
    }

    #[must_use]
    pub fn remove(key: &str) -> bool {
        let retval = unsafe { stride__storage_remove(key.as_ptr(), key.len()) };
        retval == 1
    }
}
