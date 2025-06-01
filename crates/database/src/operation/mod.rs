use chrono::Utc;
use stride_core::task::{Annotation, Date, TaskPriority, TaskStatus, Uda};
use uuid::Uuid;

pub mod difference;

/// Heavily inspired by taskchampion's Operations.
#[derive(Debug, Clone)]
pub enum OperationKind {
    TaskCreate {
        id: Uuid,
        title: Box<str>,
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
        Operation {
            timestamp: Utc::now(),
            kind: Some(self),
        }
    }
}

#[derive(Debug, Clone)]
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
}
