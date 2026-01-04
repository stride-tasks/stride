use rusqlite::{
    Result, ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, Value, ValueRef},
};
use stride_core::task::{Annotation, Date, TaskPriority, TaskStatus, Uda};
use uuid::Uuid;

use crate::{
    error::{BlobError, BlobVersionedKind},
    operation::OperationKind,
};

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

#[must_use]
pub fn task_priority_to_sql(priority: TaskPriority) -> i64 {
    match priority {
        TaskPriority::L => 0,
        TaskPriority::M => 1,
        TaskPriority::H => 2,
    }
}

impl ToSql for Sql<Option<TaskPriority>> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let Some(priority) = self.value else {
            return Ok(ToSqlOutput::Owned(Value::Null));
        };

        let value = task_priority_to_sql(priority);
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

impl ToSql for Sql<&[Uuid]> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        if self.value.is_empty() {
            return Ok(ToSqlOutput::Owned(Value::Null));
        }

        let mut blob = Vec::new();
        self.value.to_blob(&mut blob);
        Ok(ToSqlOutput::Owned(Value::Blob(blob)))
    }
}

impl FromSql for Sql<Vec<Uuid>> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let Some(mut blob) = value.as_blob_or_null()? else {
            return Ok(Vec::new().into());
        };

        match Vec::<Uuid>::from_blob(&mut blob) {
            Ok(uuids) => Ok(uuids.into()),
            Err(err) => Err(FromSqlError::Other(Box::new(err))),
        }
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

impl ToBlob<'_> for TaskStatus {
    fn to_blob(&self, blob: &mut Vec<u8>) {
        let value = match self {
            TaskStatus::Pending | TaskStatus::Complete | TaskStatus::Deleted => 0,
            TaskStatus::Waiting => 1,
            TaskStatus::Recurring => 2,
        };
        blob.push(value);
    }
}
impl FromBlob<'_> for TaskStatus {
    fn from_blob(blob: &mut &[u8]) -> Result<Self, BlobError> {
        let value = u8::from_blob(blob)?;
        Ok(match value {
            0 => TaskStatus::Pending,
            1 => TaskStatus::Waiting,
            2 => TaskStatus::Recurring,
            _ => return Err(BlobError::UnknownTaskStatus { kind: value }),
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
    fn from_blob(blob: &mut &[u8]) -> Result<Self, BlobError> {
        let value = u8::from_blob(blob)?;
        Ok(match value {
            0 => TaskPriority::L,
            1 => TaskPriority::M,
            2 => TaskPriority::H,
            _ => return Err(BlobError::UnknownTaskPriority { kind: value }),
        })
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
                kind: BlobVersionedKind::Annotation,
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
                kind: BlobVersionedKind::Uda,
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

const OPERATION_TASK_CREATE: u8 = 0x00;
const OPERATION_TASK_PURGE: u8 = 0x01;
const OPERATION_TASK_MODIFY_ENTRY: u8 = 0x02;
const OPERATION_TASK_MODIFY_TITLE: u8 = 0x03;
const OPERATION_TASK_MODIFY_STATUS: u8 = 0x04;
const OPERATION_TASK_MODIFY_ACTIVE: u8 = 0x05;
const OPERATION_TASK_MODIFY_PRIORITY: u8 = 0x06;
const OPERATION_TASK_MODIFY_PROJECT: u8 = 0x07;
const OPERATION_TASK_MODIFY_MODIFIED: u8 = 0x08;
const OPERATION_TASK_MODIFY_DUE: u8 = 0x09;
const OPERATION_TASK_MODIFY_WAIT: u8 = 0x0A;
const OPERATION_TASK_MODIFY_ADD_TAG: u8 = 0x0B;
const OPERATION_TASK_MODIFY_REMOVE_TAG: u8 = 0x0C;
const OPERATION_TASK_MODIFY_ADD_DEPENDENCY: u8 = 0x0D;
const OPERATION_TASK_MODIFY_REMOVE_DEPENDENCY: u8 = 0x0E;
const OPERATION_TASK_MODIFY_ADD_ANNOTATION: u8 = 0x0F;
const OPERATION_TASK_MODIFY_REMOVE_ANNOTATION: u8 = 0x10;
const OPERATION_TASK_MODIFY_ADD_UDA: u8 = 0x11;
const OPERATION_TASK_MODIFY_REMOVE_UDA: u8 = 0x12;

impl ToBlob<'_> for OperationKind {
    #[allow(clippy::too_many_lines)]
    fn to_blob(&self, blob: &mut Vec<u8>) {
        blob.push(0x00); // version
        match self {
            OperationKind::TaskCreate { id, title } => {
                blob.push(OPERATION_TASK_CREATE);
                id.to_blob(blob);
                title.as_ref().to_blob(blob);
            }
            OperationKind::TaskPurge { id } => {
                blob.push(OPERATION_TASK_PURGE);
                id.to_blob(blob);
            }
            OperationKind::TaskModifyEntry { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_ENTRY);
                id.to_blob(blob);
                new.to_blob(blob);
                old.to_blob(blob);
            }
            OperationKind::TaskModifyTitle { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_TITLE);
                id.to_blob(blob);
                new.as_ref().to_blob(blob);
                old.as_ref().to_blob(blob);
            }
            OperationKind::TaskModifyStatus { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_STATUS);
                id.to_blob(blob);
                new.to_blob(blob);
                old.to_blob(blob);
            }
            OperationKind::TaskModifyActive { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_ACTIVE);
                id.to_blob(blob);
                new.to_blob(blob);
                old.to_blob(blob);
            }
            OperationKind::TaskModifyPriority { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_PRIORITY);
                id.to_blob(blob);
                new.to_blob(blob);
                old.to_blob(blob);
            }
            OperationKind::TaskModifyProject { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_PROJECT);
                id.to_blob(blob);

                new.as_deref().to_blob(blob);
                old.as_deref().to_blob(blob);
            }
            OperationKind::TaskModifyModified { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_MODIFIED);
                id.to_blob(blob);
                new.to_blob(blob);
                old.to_blob(blob);
            }
            OperationKind::TaskModifyDue { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_DUE);
                id.to_blob(blob);
                new.to_blob(blob);
                old.to_blob(blob);
            }
            OperationKind::TaskModifyWait { id, new, old } => {
                blob.push(OPERATION_TASK_MODIFY_WAIT);
                id.to_blob(blob);
                new.to_blob(blob);
                old.to_blob(blob);
            }
            OperationKind::TaskModifyAddTag { id, tag } => {
                blob.push(OPERATION_TASK_MODIFY_ADD_TAG);
                id.to_blob(blob);
                tag.as_ref().to_blob(blob);
            }
            OperationKind::TaskModifyRemoveTag { id, tag } => {
                blob.push(OPERATION_TASK_MODIFY_REMOVE_TAG);
                id.to_blob(blob);
                tag.as_ref().to_blob(blob);
            }
            OperationKind::TaskModifyAddDependency { id, depend } => {
                blob.push(OPERATION_TASK_MODIFY_ADD_DEPENDENCY);
                id.to_blob(blob);
                depend.to_blob(blob);
            }
            OperationKind::TaskModifyRemoveDependency { id, depend } => {
                blob.push(OPERATION_TASK_MODIFY_REMOVE_DEPENDENCY);
                id.to_blob(blob);
                depend.to_blob(blob);
            }
            OperationKind::TaskModifyAddAnnotation { id, annotation } => {
                blob.push(OPERATION_TASK_MODIFY_ADD_ANNOTATION);
                id.to_blob(blob);
                annotation.as_ref().to_blob(blob);
            }
            OperationKind::TaskModifyRemoveAnnotation { id, annotation } => {
                blob.push(OPERATION_TASK_MODIFY_REMOVE_ANNOTATION);
                id.to_blob(blob);
                annotation.as_ref().to_blob(blob);
            }
            OperationKind::TaskModifyAddUda { id, uda } => {
                blob.push(OPERATION_TASK_MODIFY_ADD_UDA);
                id.to_blob(blob);
                uda.as_ref().to_blob(blob);
            }
            OperationKind::TaskModifyRemoveUda { id, uda } => {
                blob.push(OPERATION_TASK_MODIFY_REMOVE_UDA);
                id.to_blob(blob);
                uda.as_ref().to_blob(blob);
            }
        }
    }
}
impl FromBlob<'_> for OperationKind {
    #[allow(clippy::too_many_lines)]
    fn from_blob(blob: &mut &[u8]) -> Result<Self, BlobError> {
        let version = u8::from_blob(blob)?;

        if version != 0x00 {
            return Err(BlobError::UnknownVersion {
                version,
                kind: BlobVersionedKind::Operation,
            });
        }

        let ty = u8::from_blob(blob)?;
        Ok(match ty {
            OPERATION_TASK_CREATE => {
                let id = Uuid::from_blob(blob)?;
                let title = <&str>::from_blob(blob)?;
                OperationKind::TaskCreate {
                    id,
                    title: title.into(),
                }
            }
            OPERATION_TASK_PURGE => {
                let id = Uuid::from_blob(blob)?;
                OperationKind::TaskPurge { id }
            }
            OPERATION_TASK_MODIFY_ENTRY => {
                let id = Uuid::from_blob(blob)?;
                let new = Date::from_blob(blob)?;
                let old = Date::from_blob(blob)?;
                OperationKind::TaskModifyEntry { id, new, old }
            }
            OPERATION_TASK_MODIFY_TITLE => {
                let id = Uuid::from_blob(blob)?;
                let new = <&str>::from_blob(blob)?;
                let old = <&str>::from_blob(blob)?;
                OperationKind::TaskModifyTitle {
                    id,
                    new: new.into(),
                    old: old.into(),
                }
            }
            OPERATION_TASK_MODIFY_STATUS => {
                let id = Uuid::from_blob(blob)?;
                let new = TaskStatus::from_blob(blob)?;
                let old = TaskStatus::from_blob(blob)?;
                OperationKind::TaskModifyStatus { id, new, old }
            }
            OPERATION_TASK_MODIFY_ACTIVE => {
                let id = Uuid::from_blob(blob)?;
                let new = bool::from_blob(blob)?;
                let old = bool::from_blob(blob)?;
                OperationKind::TaskModifyActive { id, new, old }
            }
            OPERATION_TASK_MODIFY_PRIORITY => {
                let id = Uuid::from_blob(blob)?;
                let new = Option::<TaskPriority>::from_blob(blob)?;
                let old = Option::<TaskPriority>::from_blob(blob)?;
                OperationKind::TaskModifyPriority { id, new, old }
            }
            OPERATION_TASK_MODIFY_PROJECT => {
                let id = Uuid::from_blob(blob)?;
                let new = Option::<&str>::from_blob(blob)?;
                let old = Option::<&str>::from_blob(blob)?;
                OperationKind::TaskModifyProject {
                    id,
                    new: new.map(Into::into),
                    old: old.map(Into::into),
                }
            }
            OPERATION_TASK_MODIFY_MODIFIED => {
                let id = Uuid::from_blob(blob)?;
                let new = Option::<Date>::from_blob(blob)?;
                let old = Option::<Date>::from_blob(blob)?;
                OperationKind::TaskModifyModified { id, new, old }
            }
            OPERATION_TASK_MODIFY_DUE => {
                let id = Uuid::from_blob(blob)?;
                let new = Option::<Date>::from_blob(blob)?;
                let old = Option::<Date>::from_blob(blob)?;
                OperationKind::TaskModifyDue { id, new, old }
            }
            OPERATION_TASK_MODIFY_WAIT => {
                let id = Uuid::from_blob(blob)?;
                let new = Option::<Date>::from_blob(blob)?;
                let old = Option::<Date>::from_blob(blob)?;
                OperationKind::TaskModifyWait { id, new, old }
            }
            OPERATION_TASK_MODIFY_ADD_TAG => {
                let id = Uuid::from_blob(blob)?;
                let tag = <&str>::from_blob(blob)?;
                OperationKind::TaskModifyAddTag {
                    id,
                    tag: tag.into(),
                }
            }
            OPERATION_TASK_MODIFY_REMOVE_TAG => {
                let id = Uuid::from_blob(blob)?;
                let tag = <&str>::from_blob(blob)?;
                OperationKind::TaskModifyRemoveTag {
                    id,
                    tag: tag.into(),
                }
            }
            OPERATION_TASK_MODIFY_ADD_DEPENDENCY => {
                let id = Uuid::from_blob(blob)?;
                let depend = Uuid::from_blob(blob)?;
                OperationKind::TaskModifyAddDependency { id, depend }
            }
            OPERATION_TASK_MODIFY_REMOVE_DEPENDENCY => {
                let id = Uuid::from_blob(blob)?;
                let depend = Uuid::from_blob(blob)?;
                OperationKind::TaskModifyRemoveDependency { id, depend }
            }
            OPERATION_TASK_MODIFY_ADD_ANNOTATION => {
                let id = Uuid::from_blob(blob)?;
                let annotation = Annotation::from_blob(blob)?;
                OperationKind::TaskModifyAddAnnotation {
                    id,
                    annotation: Box::new(annotation),
                }
            }
            OPERATION_TASK_MODIFY_REMOVE_ANNOTATION => {
                let id = Uuid::from_blob(blob)?;
                let annotation = Annotation::from_blob(blob)?;
                OperationKind::TaskModifyRemoveAnnotation {
                    id,
                    annotation: Box::new(annotation),
                }
            }
            OPERATION_TASK_MODIFY_ADD_UDA => {
                let id = Uuid::from_blob(blob)?;
                let uda = Uda::from_blob(blob)?;
                OperationKind::TaskModifyAddUda {
                    id,
                    uda: Box::new(uda),
                }
            }
            OPERATION_TASK_MODIFY_REMOVE_UDA => {
                let id = Uuid::from_blob(blob)?;
                let uda = Uda::from_blob(blob)?;
                OperationKind::TaskModifyRemoveUda {
                    id,
                    uda: Box::new(uda),
                }
            }
            _ => return Err(BlobError::UnknownOperationKind { kind: ty }),
        })
    }
}
