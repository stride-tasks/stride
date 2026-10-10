//! Stride's api implementations.

mod value;

pub use value::{Map, Value};

pub trait Method: Sized + serde::Serialize + for<'de> serde::Deserialize<'de> {
    const NAME: &'static str;

    type Result: serde::Serialize + for<'de> serde::Deserialize<'de>;
}

pub trait Notification: Sized + serde::Serialize + for<'de> serde::Deserialize<'de> {
    const NAME: &'static str;
}

include!(concat!(env!("OUT_DIR"), "/stride_schema.rs"));
