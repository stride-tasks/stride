// TODO: Remove
#![allow(dead_code)]
#![allow(unused)]

use base64::Engine;

pub mod api;
pub mod git;
pub mod repository;
pub mod task;

pub(crate) mod escape;

mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

pub(crate) trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for uuid::Uuid {
    fn to_base64(&self) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(self.as_bytes())
    }
}
