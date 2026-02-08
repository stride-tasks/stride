use chrono::{DateTime, Utc};
use stride_core::task::{Annotation, Date, TaskPriority, TaskStatus, Uda};
use uuid::Uuid;

pub mod difference;

/// Heavily inspired by taskchampion's Operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationKind {
    TaskCreate {
        id: Uuid,
        title: Box<str>,
        entry: DateTime<Utc>,
    },
    TaskPurge {
        id: Uuid,
    },
    TaskModifyEntry {
        id: Uuid,
        new: Date,
        old: Date,
    },
    TaskModifyTitle {
        id: Uuid,
        new: Box<str>,
        old: Box<str>,
    },
    TaskModifyStatus {
        id: Uuid,
        new: TaskStatus,
        old: TaskStatus,
    },
    TaskModifyActive {
        id: Uuid,
        new: bool,
        old: bool,
    },
    TaskModifyPriority {
        id: Uuid,
        new: Option<TaskPriority>,
        old: Option<TaskPriority>,
    },
    TaskModifyProject {
        id: Uuid,
        new: Option<Box<str>>,
        old: Option<Box<str>>,
    },
    TaskModifyModified {
        id: Uuid,
        new: Option<Date>,
        old: Option<Date>,
    },
    TaskModifyDue {
        id: Uuid,
        new: Option<Date>,
        old: Option<Date>,
    },
    TaskModifyWait {
        id: Uuid,
        new: Option<Date>,
        old: Option<Date>,
    },
    TaskModifyAddTag {
        id: Uuid,
        tag: Box<str>,
    },
    TaskModifyRemoveTag {
        id: Uuid,
        tag: Box<str>,
    },
    TaskModifyAddDependency {
        id: Uuid,
        depend: Uuid,
    },
    TaskModifyRemoveDependency {
        id: Uuid,
        depend: Uuid,
    },
    TaskModifyAddAnnotation {
        id: Uuid,
        annotation: Box<Annotation>,
    },
    TaskModifyRemoveAnnotation {
        id: Uuid,
        annotation: Box<Annotation>,
    },
    TaskModifyAddUda {
        id: Uuid,
        uda: Box<Uda>,
    },
    TaskModifyRemoveUda {
        id: Uuid,
        uda: Box<Uda>,
    },
}

impl OperationKind {
    #[must_use]
    pub fn with_now(self) -> Operation {
        self.with_timestamp(Utc::now())
    }

    #[must_use]
    pub fn with_timestamp(self, timestamp: DateTime<Utc>) -> Operation {
        Operation {
            timestamp,
            kind: Some(self),
        }
    }

    #[must_use]
    pub fn invert(self) -> Option<Self> {
        Some(match self {
            Self::TaskCreate { .. } | Self::TaskPurge { .. } => return None,
            Self::TaskModifyEntry { id, new, old } => Self::TaskModifyEntry {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyTitle { id, new, old } => Self::TaskModifyTitle {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyStatus { id, new, old } => Self::TaskModifyStatus {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyActive { id, new, old } => Self::TaskModifyActive {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyPriority { id, new, old } => Self::TaskModifyPriority {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyProject { id, new, old } => Self::TaskModifyProject {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyModified { id, new, old } => Self::TaskModifyModified {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyDue { id, new, old } => Self::TaskModifyDue {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyWait { id, new, old } => Self::TaskModifyWait {
                id,
                new: old,
                old: new,
            },
            Self::TaskModifyAddTag { id, tag } => Self::TaskModifyRemoveTag { id, tag },
            Self::TaskModifyRemoveTag { id, tag } => Self::TaskModifyAddTag { id, tag },
            Self::TaskModifyAddDependency { id, depend } => {
                Self::TaskModifyRemoveDependency { id, depend }
            }
            Self::TaskModifyRemoveDependency { id, depend } => {
                Self::TaskModifyAddDependency { id, depend }
            }
            Self::TaskModifyAddAnnotation { id, annotation } => {
                Self::TaskModifyRemoveAnnotation { id, annotation }
            }
            Self::TaskModifyRemoveAnnotation { id, annotation } => {
                Self::TaskModifyAddAnnotation { id, annotation }
            }
            Self::TaskModifyAddUda { id, uda } => Self::TaskModifyRemoveUda { id, uda },
            Self::TaskModifyRemoveUda { id, uda } => Self::TaskModifyAddUda { id, uda },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation {
    pub timestamp: Date,
    pub kind: Option<OperationKind>,
}

impl Operation {
    #[must_use]
    pub fn undo_point_with_now() -> Self {
        Operation {
            timestamp: Utc::now(),
            kind: None,
        }
    }

    #[must_use]
    pub fn is_undo_point(&self) -> bool {
        self.kind.is_none()
    }

    #[must_use]
    pub fn invert(self) -> Self {
        let Some(kind) = self.kind else {
            return self;
        };

        Self {
            timestamp: self.timestamp,
            kind: kind.invert(),
        }
    }
}
