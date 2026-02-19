use chrono::{DateTime, Utc};
use stride_core::task::{TaskPriority, TaskStatus};
use uuid::Uuid;

use crate::{actor::ActorId, hlc::Timestamp};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sequence(u64);

impl Sequence {
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[derive(Debug, Clone)]
pub enum RowId {
    Uuid(Uuid),
    String(Box<str>),
}

impl From<Uuid> for RowId {
    fn from(value: Uuid) -> Self {
        Self::Uuid(value)
    }
}

impl From<Box<str>> for RowId {
    fn from(value: Box<str>) -> Self {
        Self::String(value)
    }
}

#[derive(Debug, Clone)]
pub struct Change {
    pub actor_id: ActorId,
    pub sequence: Sequence,
    pub timestamp: Timestamp,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub row_id: RowId,
    pub kind: OperationKind,
}

impl Operation {
    pub fn new(row_id: impl Into<RowId>, kind: impl Into<OperationKind>) -> Self {
        Self {
            row_id: row_id.into(),
            kind: kind.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperationKind {
    Task(TaskOperation),
    Annotation(AnnotationOperation),
}

#[derive(Debug, Clone)]
pub enum TaskOperation {
    Delete,
    ModifyEntry { entry: DateTime<Utc> },
    ModifyTitle { title: Box<str> },
    ModifyStatus { status: TaskStatus },
    ModifyPriority { priority: Option<TaskPriority> },
    ModifyProject { project: Option<Box<str>> },
    ModifyModified { modified: Option<DateTime<Utc>> },
    ModifyDue { due: Option<DateTime<Utc>> },
    ModifyWait { wait: Option<DateTime<Utc>> },
    AddTag { tag: Box<str> },
    RemoveTag { tag: Box<str> },
    AddDependency { depend: Uuid },
    RemoveDependency { depend: Uuid },
}

impl From<TaskOperation> for OperationKind {
    fn from(value: TaskOperation) -> Self {
        OperationKind::Task(value)
    }
}

#[derive(Debug, Clone)]
pub enum AnnotationOperation {
    Delete,
    ModifyTaskId { task_id: Uuid },
    ModifyEntry { entry: DateTime<Utc> },
    ModifyText { text: Box<str> },
}

impl From<AnnotationOperation> for OperationKind {
    fn from(value: AnnotationOperation) -> Self {
        OperationKind::Annotation(value)
    }
}

#[cfg(feature = "serialize")]
pub mod serialize {
    use chrono::{DateTime, Utc};
    use stride_core::task::{TaskPriority, TaskStatus};
    use stride_serialize::{FromBlob, ToBlob};
    use uuid::Uuid;

    use crate::change::{AnnotationOperation, OperationKind, Sequence, TaskOperation};

    impl ToBlob<'_> for Sequence {
        fn to_blob(&self, blob: &mut Vec<u8>) {
            self.get().to_blob(blob);
        }
    }

    impl FromBlob<'_> for Sequence {
        fn from_blob(blob: &mut &[u8]) -> stride_serialize::Result<Self> {
            Ok(Sequence::new(u64::from_blob(blob)?))
        }
    }

    pub fn operation_to_data(operation: &OperationKind, blob: &mut Vec<u8>) {
        match operation {
            OperationKind::Task(operation) => match operation {
                TaskOperation::Delete => {}
                TaskOperation::ModifyEntry { entry } => entry.to_blob(blob),
                TaskOperation::ModifyTitle { title } => title.as_ref().to_blob(blob),
                TaskOperation::ModifyStatus { status } => status.to_blob(blob),
                TaskOperation::ModifyPriority { priority } => priority.to_blob(blob),
                TaskOperation::ModifyProject { project } => project.as_deref().to_blob(blob),
                TaskOperation::ModifyModified { modified } => modified.to_blob(blob),
                TaskOperation::ModifyDue { due } => due.to_blob(blob),
                TaskOperation::ModifyWait { wait } => wait.to_blob(blob),
                TaskOperation::AddTag { tag } | TaskOperation::RemoveTag { tag } => {
                    tag.as_ref().to_blob(blob);
                }
                TaskOperation::AddDependency { depend }
                | TaskOperation::RemoveDependency { depend } => depend.to_blob(blob),
            },
            OperationKind::Annotation(operation) => match operation {
                AnnotationOperation::Delete => {}
                AnnotationOperation::ModifyTaskId { task_id } => task_id.to_blob(blob),
                AnnotationOperation::ModifyEntry { entry } => entry.to_blob(blob),
                AnnotationOperation::ModifyText { text } => text.as_ref().to_blob(blob),
            },
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn operation_from_data(
        typ: u32,
        blob: &mut &[u8],
    ) -> stride_serialize::Result<OperationKind> {
        let first = typ & 0xFF;
        let second = (typ >> 8) & 0xFF;
        Ok(match first {
            0 => match second {
                0 => TaskOperation::Delete,
                1 => TaskOperation::ModifyEntry {
                    entry: DateTime::<Utc>::from_blob(blob)?,
                },
                2 => TaskOperation::ModifyTitle {
                    title: <&str>::from_blob(blob)?.to_owned().into_boxed_str(),
                },
                3 => TaskOperation::ModifyStatus {
                    status: TaskStatus::from_blob(blob)?,
                },
                4 => TaskOperation::ModifyPriority {
                    priority: Option::<TaskPriority>::from_blob(blob)?,
                },
                5 => TaskOperation::ModifyProject {
                    project: Option::<&str>::from_blob(blob)?
                        .map(ToOwned::to_owned)
                        .map(String::into_boxed_str),
                },
                6 => TaskOperation::ModifyModified {
                    modified: Option::<DateTime<Utc>>::from_blob(blob)?,
                },
                7 => TaskOperation::ModifyDue {
                    due: Option::<DateTime<Utc>>::from_blob(blob)?,
                },
                8 => TaskOperation::ModifyWait {
                    wait: Option::<DateTime<Utc>>::from_blob(blob)?,
                },
                9 => TaskOperation::AddTag {
                    tag: <&str>::from_blob(blob)?.to_owned().into_boxed_str(),
                },
                10 => TaskOperation::RemoveTag {
                    tag: <&str>::from_blob(blob)?.to_owned().into_boxed_str(),
                },
                11 => TaskOperation::AddDependency {
                    depend: Uuid::from_blob(blob)?,
                },
                12 => TaskOperation::RemoveDependency {
                    depend: Uuid::from_blob(blob)?,
                },
                value => todo!("operation from data {value}"),
            }
            .into(),
            1 => match second {
                0 => AnnotationOperation::Delete,
                1 => AnnotationOperation::ModifyTaskId {
                    task_id: Uuid::from_blob(blob)?,
                },
                2 => AnnotationOperation::ModifyEntry {
                    entry: DateTime::<Utc>::from_blob(blob)?,
                },
                3 => AnnotationOperation::ModifyText {
                    text: <&str>::from_blob(blob)?.to_owned().into_boxed_str(),
                },
                value => todo!("unknown annoation operation type: {value}"),
            }
            .into(),
            value => todo!("unknown operation type: {value}"),
        })
    }

    #[allow(clippy::identity_op)]
    #[must_use]
    pub fn operation_type(kind: &OperationKind) -> u32 {
        match kind {
            OperationKind::Task(task) => {
                let value = match task {
                    TaskOperation::Delete => 0,
                    TaskOperation::ModifyEntry { .. } => 1,
                    TaskOperation::ModifyTitle { .. } => 2,
                    TaskOperation::ModifyStatus { .. } => 3,
                    TaskOperation::ModifyPriority { .. } => 4,
                    TaskOperation::ModifyProject { .. } => 5,
                    TaskOperation::ModifyModified { .. } => 6,
                    TaskOperation::ModifyDue { .. } => 7,
                    TaskOperation::ModifyWait { .. } => 8,
                    TaskOperation::AddTag { .. } => 9,
                    TaskOperation::RemoveTag { .. } => 10,
                    TaskOperation::AddDependency { .. } => 11,
                    TaskOperation::RemoveDependency { .. } => 12,
                };

                0 | (value << 8)
            }
            OperationKind::Annotation(annotation) => {
                let value = match annotation {
                    AnnotationOperation::Delete => 0,
                    AnnotationOperation::ModifyTaskId { .. } => 1,
                    AnnotationOperation::ModifyEntry { .. } => 2,
                    AnnotationOperation::ModifyText { .. } => 3,
                };

                1 | (value << 8)
            }
        }
    }
}
