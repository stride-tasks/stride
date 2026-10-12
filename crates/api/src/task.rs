#![allow(clippy::doc_markdown)]

use chrono::Utc;
use uuid::Uuid;

use crate::{Task, TaskPriority, TaskStatus};

impl TaskStatus {
    #[must_use]
    pub fn is_pending(&self) -> bool {
        *self == Self::Pending
    }
}

impl TaskPriority {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "H",
            Self::Medium => "M",
            Self::Low => "L",
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            status: None,
            title: None,
            entry: None,
            modified: None,
            due: None,
            project: None,
            tags: Vec::new(),
            annotations: Vec::new(),
            priority: None,
            wait: None,
            depends: Vec::new(),
            udas: Vec::new(),
        }
    }
}

impl Task {
    #[must_use]
    pub fn new(title: Box<str>) -> Self {
        Self {
            title: Some(title),
            entry: Some(Utc::now()),
            status: Some(TaskStatus::Pending),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_id(id: Uuid, title: Box<str>) -> Self {
        Task {
            id,
            title: Some(title),
            entry: Some(Utc::now()),
            status: Some(TaskStatus::Pending),
            ..Default::default()
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn urgency(&self) -> f32 {
        const THREE_DAYS: i64 = 3 * 24 * 60 * 60;

        let mut urgency = 0.0;
        if let Some(due) = self.due {
            let today = Utc::now();
            let delta = due - today;

            urgency += 1.0;

            let seconds = delta.num_seconds();
            if seconds < 0 {
                urgency += 11.0;
            } else if seconds <= THREE_DAYS {
                urgency += (seconds as f32 / THREE_DAYS as f32) * 11.0;
            }
        }
        if let Some(priority) = self.priority {
            match priority {
                TaskPriority::High => urgency += 6.0,
                TaskPriority::Medium => urgency += 3.0,
                TaskPriority::Low => urgency += -3.0,
            }
        }
        urgency
    }
}
