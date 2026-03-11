use std::{collections::HashSet, hash::Hash};

use stride_core::task::{Annotation, Task};
use uuid::Uuid;

use crate::change::{AnnotationOperation, Operation, TaskOperation};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DifferenceType {
    Addition,
    Deletion,
}

fn difference_between<'a, T>(previous: &'a [T], current: &'a [T]) -> Vec<(&'a T, DifferenceType)>
where
    T: Eq + Hash,
{
    let previous = previous.iter().collect::<HashSet<_>>();
    let current = current.iter().collect::<HashSet<_>>();

    previous
        .difference(&current)
        .map(|value| (*value, DifferenceType::Deletion))
        .chain(
            current
                .difference(&previous)
                .map(|value| (*value, DifferenceType::Addition)),
        )
        .collect()
}

#[allow(clippy::too_many_lines)]
pub fn push_task_diff_operations(
    id: Uuid,
    current: &Task,
    previous: Option<&Task>,
    ops: &mut Vec<Operation>,
) {
    let old_ops_len = ops.len();
    if let Some(title) = &current.title {
        if let Some(previous) = previous {
            if previous.title != current.title {
                ops.push(Operation::new(
                    id,
                    TaskOperation::ModifyTitle {
                        title: title.clone().into_boxed_str(),
                    },
                ));
            }
        } else {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyTitle {
                    title: title.clone().into_boxed_str(),
                },
            ));
        }
    }

    if let Some(entry) = &current.entry {
        if let Some(previous) = previous {
            if previous.entry != current.entry {
                ops.push(Operation::new(
                    id,
                    TaskOperation::ModifyEntry { entry: *entry },
                ));
            }
        } else {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyEntry { entry: *entry },
            ));
        }
    }

    if let Some(status) = &current.status {
        if let Some(previous) = previous {
            if previous.status != current.status {
                ops.push(Operation::new(
                    id,
                    TaskOperation::ModifyStatus { status: *status },
                ));
            }
        } else {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyStatus { status: *status },
            ));
        }
    }

    match previous {
        Some(previous) if current.due != previous.due => {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyDue { due: current.due },
            ));
        }
        None if current.due.is_some() => {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyDue { due: current.due },
            ));
        }
        _ => {}
    }
    match previous {
        Some(previous) if current.wait != previous.wait => {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyWait { wait: current.wait },
            ));
        }
        None if current.wait.is_some() => {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyWait { wait: current.wait },
            ));
        }
        _ => {}
    }
    match previous {
        Some(previous) if current.project != previous.project => {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyProject {
                    project: current.project.clone().map(Into::into),
                },
            ));
        }
        None if current.project.is_some() => {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyProject {
                    project: current.project.clone().map(Into::into),
                },
            ));
        }
        _ => {}
    }
    match previous {
        Some(previous) if current.priority != previous.priority => {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyPriority {
                    priority: current.priority,
                },
            ));
        }
        None if current.priority.is_some() => {
            ops.push(Operation::new(
                id,
                TaskOperation::ModifyPriority {
                    priority: current.priority,
                },
            ));
        }
        _ => {}
    }

    match previous {
        Some(previous) if current.tags != previous.tags => {
            for (tag, ty) in difference_between(&previous.tags, &current.tags) {
                let kind = match ty {
                    DifferenceType::Addition => TaskOperation::AddTag {
                        tag: tag.clone().into(),
                    },
                    DifferenceType::Deletion => TaskOperation::RemoveTag {
                        tag: tag.clone().into(),
                    },
                };
                ops.push(Operation::new(id, kind));
            }
        }
        None => {
            for tag in &current.tags {
                ops.push(Operation::new(
                    id,
                    TaskOperation::AddTag {
                        tag: tag.clone().into(),
                    },
                ));
            }
        }
        _ => {}
    }

    push_annotation_diff_operations(
        id,
        &current.annotations,
        previous.map_or(&[], |previous| &previous.annotations),
        ops,
    );

    // Only set the modified date if something changed.
    if old_ops_len != ops.len() {
        match previous {
            Some(previous) if current.modified != previous.modified => {
                ops.push(Operation::new(
                    id,
                    TaskOperation::ModifyModified {
                        modified: current.modified,
                    },
                ));
            }
            None if current.modified.is_some() => {
                ops.push(Operation::new(
                    id,
                    TaskOperation::ModifyModified {
                        modified: current.modified,
                    },
                ));
            }
            _ => {}
        }
    }
}

pub fn push_annotation_diff_operations(
    id: Uuid,
    current: &[Annotation],
    previous: &[Annotation],
    ops: &mut Vec<Operation>,
) {
    for current_annotation in current {
        let Some(annotation) = previous.iter().find(|ann| ann.id == current_annotation.id) else {
            ops.push(Operation::new(
                current_annotation.id,
                AnnotationOperation::ModifyTaskId { task_id: id },
            ));
            ops.push(Operation::new(
                current_annotation.id,
                AnnotationOperation::ModifyEntry {
                    entry: current_annotation.entry,
                },
            ));
            ops.push(Operation::new(
                current_annotation.id,
                AnnotationOperation::ModifyText {
                    text: current_annotation.text.clone().into_boxed_str(),
                },
            ));
            continue;
        };

        if current_annotation.entry != annotation.entry {
            ops.push(Operation::new(
                current_annotation.id,
                AnnotationOperation::ModifyEntry {
                    entry: current_annotation.entry,
                },
            ));
        }
        if current_annotation.text != annotation.text {
            ops.push(Operation::new(
                current_annotation.id,
                AnnotationOperation::ModifyText {
                    text: current_annotation.text.clone().into_boxed_str(),
                },
            ));
        }
    }

    for previous_annotation in previous {
        if current.iter().any(|ann| ann.id == previous_annotation.id) {
            continue;
        }

        ops.push(Operation::new(
            previous_annotation.id,
            AnnotationOperation::Delete,
        ));
    }
}
