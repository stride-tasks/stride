use stride_core::task::{Date, TaskPriority, TaskStatus};
use uuid::Uuid;

use crate::error::Error;

#[cfg(test)]
mod tests;

pub trait ToBlob<'a> {
    fn to_blob(&self, blob: &mut Vec<u8>);
}

pub trait FromBlob<'a>: Sized {
    #[allow(clippy::missing_errors_doc)]
    fn from_blob(blob: &mut &'a [u8]) -> Result<Self, Error>;
}

impl ToBlob<'_> for bool {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        u8::from(*self).to_blob(blob);
    }
}
impl FromBlob<'_> for bool {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, Error> {
        Ok(u8::from_blob(blob)? != 0)
    }
}

impl ToBlob<'_> for u8 {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.push(*self);
    }
}
impl FromBlob<'_> for u8 {
    fn from_blob(input: &mut &[u8]) -> Result<Self, Error> {
        let (value, blob) = input.split_first_chunk::<1>().ok_or(Error::AbruptEnd)?;
        let value = value[0];
        *input = blob;
        Ok(value)
    }
}

impl ToBlob<'_> for u32 {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(vint64::encode(u64::from(*self)).as_ref());
    }
}
impl FromBlob<'_> for u32 {
    fn from_blob(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(vint64::decode(input)?.try_into()?)
    }
}

impl ToBlob<'_> for u64 {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(vint64::encode(*self).as_ref());
    }
}
impl FromBlob<'_> for u64 {
    fn from_blob(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(vint64::decode(input)?)
    }
}
impl ToBlob<'_> for i64 {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(vint64::signed::encode(*self).as_ref());
    }
}
impl FromBlob<'_> for i64 {
    fn from_blob(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(vint64::signed::decode(input)?)
    }
}

impl ToBlob<'_> for usize {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(vint64::encode(*self as u64).as_ref());
    }
}
impl FromBlob<'_> for usize {
    fn from_blob(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(vint64::decode(input)?.try_into()?)
    }
}

impl ToBlob<'_> for Uuid {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(self.as_bytes());
    }
}
impl FromBlob<'_> for Uuid {
    fn from_blob(input: &mut &[u8]) -> Result<Self, Error> {
        let (bytes, blob) = input.split_first_chunk::<16>().ok_or(Error::AbruptEnd)?;
        let value = Uuid::from_bytes(*bytes);
        *input = blob;
        Ok(value)
    }
}

impl ToBlob<'_> for Date {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        self.timestamp_micros().to_blob(blob);
    }
}
impl FromBlob<'_> for Date {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, Error> {
        let timestamp = i64::from_blob(blob)?;
        let datetime = Date::from_timestamp_micros(timestamp).ok_or(Error::InvalidTimestamp)?;
        Ok(datetime)
    }
}

impl<'a> FromBlob<'a> for &'a [u8] {
    fn from_blob(input: &mut &'a [u8]) -> Result<Self, Error> {
        let len = usize::from_blob(input)?;
        let (bytes, blob) = input.split_at_checked(len).ok_or(Error::AbruptEnd)?;
        *input = blob;
        Ok(bytes)
    }
}

impl<'a> ToBlob<'a> for &'a str {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        self.as_bytes().to_blob(blob);
    }
}
impl<'a> FromBlob<'a> for &'a str {
    fn from_blob(input: &mut &'a [u8]) -> Result<Self, Error> {
        let bytes = <&[u8]>::from_blob(input)?;
        let str = std::str::from_utf8(bytes).map_err(Error::InvalidUt8)?;
        Ok(str)
    }
}

impl<'a, T: ToBlob<'a>> ToBlob<'a> for Option<T> {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        if let Some(value) = self.as_ref() {
            blob.push(0x01);
            value.to_blob(blob);
        } else {
            blob.push(0x00);
        }
    }
}
impl<'a, T: FromBlob<'a>> FromBlob<'a> for Option<T> {
    fn from_blob(input: &mut &'a [u8]) -> Result<Self, Error> {
        let (has_value, mut blob) = input.split_first_chunk::<1>().ok_or(Error::AbruptEnd)?;
        let mut value = None;
        if has_value[0] != 0 {
            value = Some(T::from_blob(&mut blob)?);
        }
        *input = blob;
        Ok(value)
    }
}

impl ToBlob<'_> for TaskStatus {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        let value = match self {
            TaskStatus::Pending => 0,
            TaskStatus::Done => 1,
            TaskStatus::Deleted => 2,
        };
        blob.push(value);
    }
}
impl FromBlob<'_> for TaskStatus {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, Error> {
        let value = u8::from_blob(blob)?;
        Ok(match value {
            0 => TaskStatus::Pending,
            1 => TaskStatus::Done,
            2 => TaskStatus::Deleted,
            _ => return Err(Error::UnknownTaskStatus { kind: value }),
        })
    }
}

impl ToBlob<'_> for TaskPriority {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        let value = match self {
            TaskPriority::L => 0,
            TaskPriority::M => 1,
            TaskPriority::H => 2,
        };
        blob.push(value);
    }
}
impl FromBlob<'_> for TaskPriority {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, Error> {
        let value = u8::from_blob(blob)?;
        Ok(match value {
            0 => TaskPriority::L,
            1 => TaskPriority::M,
            2 => TaskPriority::H,
            _ => return Err(Error::UnknownTaskPriority { kind: value }),
        })
    }
}

#[allow(clippy::cast_possible_truncation)]
impl<'a, T: ToBlob<'a>> ToBlob<'a> for &'a [T] {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        self.len().to_blob(blob);
        for value in *self {
            value.to_blob(blob);
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
impl<'a, T: ToBlob<'a>> ToBlob<'a> for Vec<T> {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        self.len().to_blob(blob);
        for value in self {
            value.to_blob(blob);
        }
    }
}
impl<'a, T: FromBlob<'a>> FromBlob<'a> for Vec<T> {
    fn from_blob(blob: &mut &'a [u8]) -> Result<Self, Error> {
        let len = usize::from_blob(blob)?;
        let mut result = Vec::new();
        for _ in 0..len {
            result.push(T::from_blob(blob)?);
        }
        Ok(result)
    }
}
