use rusqlite::{
    Result, ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, Value, ValueRef},
};
use stride_core::task::{Annotation, Date, TaskPriority, TaskStatus};

use crate::AnnotationParseError;

pub(crate) struct Sql<T> {
    pub value: T,
}

impl<T> From<T> for Sql<T> {
    #[inline(always)]
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
        TaskStatus::Pending => 0,
        TaskStatus::Complete => 1,
        TaskStatus::Deleted => 2,
        TaskStatus::Waiting => 0,
        TaskStatus::Recurring => 0,
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

fn annotations_to_blob(annoations: &[Annotation]) -> Option<Vec<u8>> {
    if annoations.is_empty() {
        return None;
    }

    let mut result = Vec::new();
    result.push(0x00);
    let annotation_len = annoations.len() as u32;
    result.extend_from_slice(&annotation_len.to_be_bytes());

    for Annotation { entry, description } in annoations {
        let timestamp = entry.timestamp_micros();
        let text_len = description.len() as u32;

        result.extend_from_slice(&timestamp.to_be_bytes());
        result.extend_from_slice(&text_len.to_be_bytes());
        result.extend_from_slice(description.as_bytes());
    }
    Some(result)
}

fn blob_to_annotations(blob: &[u8]) -> Result<Option<Vec<Annotation>>, AnnotationParseError> {
    let Some((version, blob)) = blob.split_first_chunk::<1>() else {
        return Ok(None);
    };

    let version = u8::from_be_bytes(*version);
    if version != 0x00 {
        return Err(AnnotationParseError::UnknownVersion { version });
    }

    let Some((len, mut blob)) = blob.split_first_chunk::<4>() else {
        return Err(AnnotationParseError::MissingLength);
    };

    let len = u32::from_be_bytes(*len) as usize;

    let mut annotations = Vec::new();
    for _ in 0..len {
        let (entry_bytes, new_blob) = blob
            .split_first_chunk::<8>()
            .ok_or(AnnotationParseError::MissingEntryTimestamp)?;
        let entry_timestamp = i64::from_be_bytes(*entry_bytes);

        let (len_bytes, new_blob) = new_blob
            .split_first_chunk::<4>()
            .ok_or(AnnotationParseError::MissingLength)?;
        let len = u32::from_be_bytes(*len_bytes);

        let (text_bytes, new_blob) = new_blob
            .split_at_checked(len as usize)
            .ok_or(AnnotationParseError::MissingText)?;
        let text = std::str::from_utf8(text_bytes).map_err(|_| AnnotationParseError::InvalidUt8)?;

        annotations.push(Annotation {
            entry: Date::from_timestamp_micros(entry_timestamp)
                .ok_or(AnnotationParseError::InvalidTimestamp)?,
            description: text.into(),
        });

        blob = new_blob;
    }

    Ok(Some(annotations))
}

impl ToSql for Sql<&[Annotation]> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        if self.value.is_empty() {
            return Ok(ToSqlOutput::Owned(Value::Null));
        }

        let Some(blob) = annotations_to_blob(self.value) else {
            return Ok(ToSqlOutput::Owned(Value::Null));
        };

        Ok(ToSqlOutput::Owned(Value::Blob(blob)))
    }
}

impl FromSql for Sql<Vec<Annotation>> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let Some(blob) = value.as_blob_or_null()? else {
            return Ok(Vec::new().into());
        };

        match blob_to_annotations(blob) {
            Ok(annotations) => Ok(annotations.unwrap_or_default().into()),
            Err(err) => Err(FromSqlError::Other(Box::new(err))),
        }
    }
}
