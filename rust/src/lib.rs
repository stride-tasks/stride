// TODO: Remove
#![allow(dead_code)]
#![allow(unused)]

use base64::Engine;
use chrono::{DateTime, Datelike, Timelike};
use task::Date;
use uuid::Uuid;

pub mod api;
pub mod git;
pub mod repository;
pub mod task;

pub(crate) mod escape;

mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

pub(crate) trait ToBase64 {
    fn to_base64(&self) -> String;
}

pub(crate) trait ToBase64Array<const N: usize> {
    fn to_base64_array(&self) -> [u8; N];
    fn from_base64_array(input: &[u8; N]) -> Option<Self>
    where
        Self: Sized;
}

impl ToBase64 for uuid::Uuid {
    fn to_base64(&self) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(self.as_bytes())
    }
}

impl ToBase64Array<22> for uuid::Uuid {
    fn to_base64_array(&self) -> [u8; 22] {
        let mut result = [0u8; 22];
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode_slice(self.as_bytes(), &mut result)
            .expect("should fit in buffer");
        result
    }
    fn from_base64_array(input: &[u8; 22]) -> Option<Self> {
        let mut bytes = [0u8; 16];
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode_slice(input, &mut bytes)
            .ok()?;

        Some(Uuid::from_bytes(bytes))
    }
}

impl ToBase64Array<11> for Date {
    fn to_base64_array(&self) -> [u8; 11] {
        let mut result = [0u8; 11];
        let timestamp = self.timestamp_micros();
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode_slice(timestamp.to_be_bytes(), &mut result)
            .expect("should fit in buffer");
        result
    }
    fn from_base64_array(input: &[u8; 11]) -> Option<Self> {
        let mut bytes = [0u8; 8];
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode_slice(input, &mut bytes)
            .ok()?;

        let timestamp = i64::from_be_bytes(bytes);

        DateTime::from_timestamp_micros(timestamp)
    }
}

impl ToBase64Array<6> for u32 {
    fn to_base64_array(&self) -> [u8; 6] {
        let mut result = [0u8; 6];
        let bytes = self.to_be_bytes();
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode_slice(bytes, &mut result)
            .expect("should fit in buffer");
        result
    }
    fn from_base64_array(input: &[u8; 6]) -> Option<Self> {
        let mut bytes = [0u8; 4];
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode_slice(input, &mut bytes)
            .ok()?;

        Some(u32::from_be_bytes(bytes))
    }
}
