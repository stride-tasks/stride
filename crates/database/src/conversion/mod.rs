use rusqlite::{
    Result, ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, Value, ValueRef},
};
use stride_core::task::{Date, TaskPriority, TaskStatus};
use stride_crdt::change::RowId;
use stride_serialize::{FromBlob, ToBlob};
use uuid::Uuid;

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
        TaskStatus::Pending => 0,
        TaskStatus::Done => 1,
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
            1 => TaskStatus::Done,
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

impl FromSql for Sql<Option<TaskStatus>> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let Some(value) = value.as_i64_or_null()? else {
            return Ok(None.into());
        };

        Ok(Some(match value {
            0 => TaskStatus::Pending,
            1 => TaskStatus::Done,
            2 => TaskStatus::Deleted,
            _ => return Err(FromSqlError::OutOfRange(value)),
        })
        .into())
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

impl ToSql for Sql<&'_ RowId> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        match &self.value {
            RowId::Uuid(value) => Ok(ToSqlOutput::Borrowed(ValueRef::Blob(value.as_bytes()))),
            RowId::String(value) => Ok(ToSqlOutput::Borrowed(ValueRef::Text(value.as_bytes()))),
        }
    }
}
