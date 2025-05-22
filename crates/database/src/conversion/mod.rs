use rusqlite::{
    Result, ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, Value, ValueRef},
};
use stride_core::task::{Annotation, Date, TaskPriority, TaskStatus, Uda};
use uuid::Uuid;

use crate::error::{BlobError, PrimitiveVersionedKind};

#[cfg(test)]
mod tests;

pub(crate) struct Sql<T> {
    pub value: T,
}

impl<T> From<T> for Sql<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self { value }
    }
}

impl ToSql for Sql<Date> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let timestamp = self.value.timestamp_millis();
        Ok(ToSqlOutput::from(timestamp))
    }
}

impl FromSql for Sql<Date> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let timestamp = value.as_i64()?;
        Date::from_timestamp_millis(timestamp)
            .ok_or(FromSqlError::OutOfRange(timestamp))
            .map(Into::into)
    }
}

impl ToSql for Sql<Option<Date>> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let Some(datetime) = self.value else {
            return Ok(ToSqlOutput::Owned(Value::Null));
        };

        let timestamp = datetime.timestamp_millis();
        Ok(ToSqlOutput::from(timestamp))
    }
}

impl FromSql for Sql<Option<Date>> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let Some(timestamp) = value.as_i64_or_null()? else {
            return Ok(None.into());
        };
        Date::from_timestamp_millis(timestamp)
            .ok_or(FromSqlError::OutOfRange(timestamp))
            .map(Some)
            .map(Into::into)
    }
}

pub(crate) fn task_status_to_sql(status: TaskStatus) -> i64 {
    match status {
        TaskStatus::Pending | TaskStatus::Waiting | TaskStatus::Recurring => 0,
        TaskStatus::Complete => 1,
        TaskStatus::Deleted => 2,
    }
}

impl ToSql for Sql<TaskStatus> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let value = task_status_to_sql(self.value);
        Ok(ToSqlOutput::Owned(Value::Integer(value)))
    }
}

impl FromSql for Sql<TaskStatus> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let value = value.as_i64()?;

        Ok(match value {
            0 => TaskStatus::Pending,
            1 => TaskStatus::Complete,
            2 => TaskStatus::Deleted,
            _ => return Err(FromSqlError::OutOfRange(value)),
        }
        .into())
    }
}

impl ToSql for Sql<Option<TaskPriority>> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let Some(priority) = self.value else {
            return Ok(ToSqlOutput::Owned(Value::Null));
        };

        let value = match priority {
            TaskPriority::L => 0,
            TaskPriority::M => 1,
            TaskPriority::H => 2,
        };

        Ok(ToSqlOutput::Owned(Value::Integer(value)))
    }
}

impl FromSql for Sql<Option<TaskPriority>> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let Some(value) = value.as_i64_or_null()? else {
            return Ok(None.into());
        };

        Ok(Some(match value {
            0 => TaskPriority::L,
            1 => TaskPriority::M,
            2 => TaskPriority::H,
            _ => return Err(FromSqlError::OutOfRange(value)),
        })
        .into())
    }
}

impl ToSql for Sql<&[Annotation]> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        if self.value.is_empty() {
            return Ok(ToSqlOutput::Owned(Value::Null));
        }

        let mut blob = Vec::new();
        self.value.to_blob(&mut blob);
        Ok(ToSqlOutput::Owned(Value::Blob(blob)))
    }
}

impl FromSql for Sql<Vec<Annotation>> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let Some(mut blob) = value.as_blob_or_null()? else {
            return Ok(Vec::new().into());
        };

        match Vec::<Annotation>::from_blob(&mut blob) {
            Ok(annotations) => Ok(annotations.into()),
            Err(err) => Err(FromSqlError::Other(Box::new(err))),
        }
    }
}

impl ToSql for Sql<&[Uda]> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        if self.value.is_empty() {
            return Ok(ToSqlOutput::Owned(Value::Null));
        }

        let mut blob = Vec::new();
        self.value.to_blob(&mut blob);
        Ok(ToSqlOutput::Owned(Value::Blob(blob)))
    }
}

impl FromSql for Sql<Vec<Uda>> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let Some(mut blob) = value.as_blob_or_null()? else {
            return Ok(Vec::new().into());
        };

        match Vec::<Uda>::from_blob(&mut blob) {
            Ok(udas) => Ok(udas.into()),
            Err(err) => Err(FromSqlError::Other(Box::new(err))),
        }
    }
}

pub trait ToBlob<'a> {
    fn to_blob(&self, blob: &mut Vec<u8>);
}

pub trait FromBlob<'a>: Sized {
    fn from_blob(blob: &mut &'a [u8]) -> Result<Self, BlobError>;
}

impl ToBlob<'_> for bool {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        u8::from(*self).to_blob(blob);
    }
}
impl FromBlob<'_> for bool {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, BlobError> {
        Ok(u8::from_blob(blob)? != 0)
    }
}

impl ToBlob<'_> for u8 {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.push(*self);
    }
}
impl FromBlob<'_> for u8 {
    fn from_blob(input: &mut &[u8]) -> Result<Self, BlobError> {
        let (value, blob) = input.split_first_chunk::<1>().ok_or(BlobError::AbruptEnd)?;
        let value = value[0];
        *input = blob;
        Ok(value)
    }
}

impl ToBlob<'_> for u32 {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(&(*self).to_be_bytes());
    }
}
impl FromBlob<'_> for u32 {
    fn from_blob(input: &mut &[u8]) -> Result<Self, BlobError> {
        let (bytes, blob) = input.split_first_chunk::<4>().ok_or(BlobError::AbruptEnd)?;
        let value = u32::from_be_bytes(*bytes);
        *input = blob;
        Ok(value)
    }
}

impl ToBlob<'_> for i64 {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(&(*self).to_be_bytes());
    }
}
impl FromBlob<'_> for i64 {
    fn from_blob(input: &mut &[u8]) -> Result<Self, BlobError> {
        let (bytes, blob) = input.split_first_chunk::<8>().ok_or(BlobError::AbruptEnd)?;
        let value = i64::from_be_bytes(*bytes);
        *input = blob;
        Ok(value)
    }
}

impl ToBlob<'_> for Uuid {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(self.as_bytes());
    }
}
impl FromBlob<'_> for Uuid {
    fn from_blob(input: &mut &[u8]) -> Result<Self, BlobError> {
        let (bytes, blob) = input
            .split_first_chunk::<16>()
            .ok_or(BlobError::AbruptEnd)?;
        let value = Uuid::from_bytes(*bytes);
        *input = blob;
        Ok(value)
    }
}

impl ToBlob<'_> for Date {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.extend_from_slice(&self.timestamp_micros().to_be_bytes());
    }
}
impl FromBlob<'_> for Date {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, BlobError> {
        let timestamp = i64::from_blob(blob)?;
        let datetime = Date::from_timestamp_micros(timestamp).ok_or(BlobError::InvalidTimestamp)?;
        Ok(datetime)
    }
}

impl<'a> FromBlob<'a> for &'a [u8] {
    fn from_blob(input: &mut &'a [u8]) -> Result<Self, BlobError> {
        let len = u32::from_blob(input)? as usize;
        let (bytes, blob) = input.split_at_checked(len).ok_or(BlobError::AbruptEnd)?;
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
    fn from_blob(input: &mut &'a [u8]) -> Result<Self, BlobError> {
        let bytes = <&[u8]>::from_blob(input)?;
        let str = std::str::from_utf8(bytes).map_err(BlobError::InvalidUt8)?;
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
    fn from_blob(input: &mut &'a [u8]) -> Result<Self, BlobError> {
        let (has_value, mut blob) = input.split_first_chunk::<1>().ok_or(BlobError::AbruptEnd)?;
        let mut value = None;
        if has_value[0] != 0 {
            value = Some(T::from_blob(&mut blob)?);
        }
        *input = blob;
        Ok(value)
    }
}

#[allow(clippy::cast_possible_truncation)]
impl ToBlob<'_> for Annotation {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.push(0x00); // version

        let timestamp = self.entry.timestamp_micros();
        let text_len = self.description.len() as u32;

        blob.extend_from_slice(&timestamp.to_be_bytes());
        blob.extend_from_slice(&text_len.to_be_bytes());
        blob.extend_from_slice(self.description.as_bytes());
    }
}
impl FromBlob<'_> for Annotation {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, BlobError> {
        let version = u8::from_blob(blob)?;
        if version != 0x00 {
            return Err(BlobError::UnknownVersion {
                version,
                kind: PrimitiveVersionedKind::Annotation,
            });
        }
        let entry = Date::from_blob(blob)?;
        let text = <&str>::from_blob(blob)?;
        Ok(Annotation {
            entry,
            description: text.to_string(),
        })
    }
}

impl ToBlob<'_> for Uda {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.push(0x00); // version

        self.namespace.as_str().to_blob(blob);
        self.key.as_str().to_blob(blob);
        self.value.as_slice().to_blob(blob);
    }
}
impl FromBlob<'_> for Uda {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, BlobError> {
        let version = u8::from_blob(blob)?;
        if version != 0x00 {
            return Err(BlobError::UnknownVersion {
                version,
                kind: PrimitiveVersionedKind::Uda,
            });
        }
        let namespace = <&str>::from_blob(blob)?;
        let key = <&str>::from_blob(blob)?;
        let value = <&[u8]>::from_blob(blob)?;
        Ok(Self {
            namespace: namespace.to_string(),
            key: key.to_string(),
            value: value.to_vec(),
        })
    }
}

#[allow(clippy::cast_possible_truncation)]
impl<'a, T: ToBlob<'a>> ToBlob<'a> for &'a [T] {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        (self.len() as u32).to_blob(blob);
        for value in *self {
            value.to_blob(blob);
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
impl<'a, T: ToBlob<'a>> ToBlob<'a> for Vec<T> {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        (self.len() as u32).to_blob(blob);
        for value in self {
            value.to_blob(blob);
        }
    }
}
impl<'a, T: FromBlob<'a>> FromBlob<'a> for Vec<T> {
    fn from_blob(blob: &mut &'a [u8]) -> Result<Self, BlobError> {
        let len = u32::from_blob(blob)? as usize;
        let mut result = Vec::new();
        for _ in 0..len {
            result.push(T::from_blob(blob)?);
        }
        Ok(result)
    }
}
