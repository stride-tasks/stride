use stride_core::task::Date;
use uuid::Uuid;

use super::{FromBlob, ToBlob};

#[test]
fn u8_to_and_from_blob() {
    let mut blob = Vec::new();

    10u8.to_blob(&mut blob);
    assert_eq!(blob, [10]);

    let mut slice = &blob[..];
    let value = u8::from_blob(&mut slice).unwrap();
    assert_eq!(value, 10);
    assert!(slice.is_empty());
}

#[test]
fn u32_to_and_from_blob() {
    let mut blob = Vec::new();

    u32::MAX.to_blob(&mut blob);
    assert_eq!(blob, &[240, 255, 255, 255, 31]);

    let mut slice = &blob[..];
    let value = u32::from_blob(&mut slice).unwrap();
    assert_eq!(value, u32::MAX);
    assert!(slice.is_empty());
}

#[test]
fn date_to_and_from_blob() {
    let mut blob = Vec::new();

    let timestamp = 10_000_000_000;
    let date = Date::from_timestamp_micros(timestamp).unwrap();
    date.to_blob(&mut blob);
    assert_eq!(blob, &[16, 0, 249, 2, 149]);

    let mut slice = &blob[..];
    let value = Date::from_blob(&mut slice).unwrap();
    assert_eq!(value, date);
    assert!(slice.is_empty());
}

#[test]
fn u8_slice_to_and_from_blob() {
    let mut blob = Vec::new();

    let slice_value = [1u8, 2u8, 3u8].as_slice();
    slice_value.to_blob(&mut blob);
    assert_eq!(blob, vec![7, 1, 2, 3]);

    let mut slice = &blob[..];
    let value = <&[u8]>::from_blob(&mut slice).unwrap();
    assert_eq!(value, slice_value);
    assert!(slice.is_empty());
}

#[test]
fn str_to_and_from_blob() {
    let mut blob = Vec::new();

    let str = "Hello";
    str.to_blob(&mut blob);
    assert_eq!(blob, vec![11, b'H', b'e', b'l', b'l', b'o']);

    let mut slice = &blob[..];
    let value = <&str>::from_blob(&mut slice).unwrap();
    assert_eq!(value, str);
    assert!(slice.is_empty());
}

#[test]
fn uuid_to_and_from_blob() {
    let mut blob = Vec::new();

    let uuid = Uuid::now_v7();
    uuid.to_blob(&mut blob);
    assert_eq!(blob, uuid.as_bytes());

    let mut slice = &blob[..];
    let value = Uuid::from_blob(&mut slice).unwrap();
    assert_eq!(value, uuid);
    assert!(slice.is_empty());
}

#[test]
fn option_str_none_to_and_from_blob() {
    let mut blob = Vec::new();

    let str = None;
    str.to_blob(&mut blob);
    assert_eq!(blob, vec![0]);

    let mut slice = &blob[..];
    let value = <Option<&str>>::from_blob(&mut slice).unwrap();
    assert_eq!(value, str);
    assert!(slice.is_empty());
}

#[test]
fn option_str_some_to_and_from_blob() {
    let mut blob = Vec::new();

    let str = Some("Hello");
    str.to_blob(&mut blob);
    assert_eq!(blob, vec![1, 11, b'H', b'e', b'l', b'l', b'o']);

    let mut slice = &blob[..];
    let value = <Option<&str>>::from_blob(&mut slice).unwrap();
    assert_eq!(value, str);
    assert!(slice.is_empty());
}

// TODO: Add tests for Operation
