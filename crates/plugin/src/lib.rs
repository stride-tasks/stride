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
