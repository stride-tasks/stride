use std::{collections::HashSet, hash::Hash};

use chrono::Utc;
use stride_core::task::{Task, TaskStatus};

use crate::operation::{Operation, OperationKind};

pub(crate) enum DifferenceType {
    Addition,
    Deletion,
}

pub(crate) fn difference_between<'a, T>(
    previous: &'a [T],
    current: &'a [T],
) -> Vec<(&'a T, DifferenceType)>
where
    T: Eq + Hash,
{
    if current != previous {
        return Vec::new();
    }

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
pub fn push_operations_diff_task(
    current: Option<&Task>,
    previous: Option<&Task>,
    ops: &mut Vec<Operation>,
) {
    let timestamp = Utc::now();

    match (current, previous) {
        (Some(current), Some(previous)) if current.title != previous.title => {
            ops.push(
                OperationKind::TaskModifyTitle {
                    id: current.uuid,
                    new: current.title.clone().into(),
                    old: previous.title.clone().into(),
                }
                .with_timestamp(timestamp),
            );
        }
        (Some(current), None) => {
            ops.push(
                OperationKind::TaskCreate {
                    id: current.uuid,
                    title: current.title.clone().into(),
                    entry: current.entry,
                }
                .with_timestamp(timestamp),
            );
        }
        (None, Some(_previous)) => {
            // ops.push(OperationKind::TaskRemove);
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.entry != previous.entry => {
            ops.push(
                OperationKind::TaskModifyEntry {
                    id: current.uuid,
                    new: current.entry,
                    old: previous.entry,
                }
                .with_timestamp(timestamp),
            );
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.status != previous.status => {
            ops.push(
                OperationKind::TaskModifyStatus {
                    id: current.uuid,
                    new: current.status,
                    old: previous.status,
                }
                .with_timestamp(timestamp),
            );
        }
        (Some(current), None) if current.status != TaskStatus::Pending => {
            ops.push(
                OperationKind::TaskModifyStatus {
                    id: current.uuid,
                    new: current.status,
                    old: TaskStatus::Pending,
                }
                .with_timestamp(timestamp),
            );
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.active != previous.active => {
            ops.push(
                OperationKind::TaskModifyActive {
                    id: current.uuid,
                    new: current.active,
                    old: previous.active,
                }
                .with_timestamp(timestamp),
            );
        }
        (Some(current), None) if current.active => {
            ops.push(
                OperationKind::TaskModifyActive {
                    id: current.uuid,
                    new: current.active,
                    old: false,
                }
                .with_timestamp(timestamp),
            );
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.modified != previous.modified => {
            ops.push(
                OperationKind::TaskModifyModified {
                    id: current.uuid,
                    new: current.modified,
                    old: previous.modified,
                }
                .with_timestamp(timestamp),
            );
        }
        (Some(current), None) if current.modified.is_some() => {
            ops.push(
                OperationKind::TaskModifyModified {
                    id: current.uuid,
                    new: current.modified,
                    old: None,
                }
                .with_timestamp(timestamp),
            );
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.due != previous.due => {
            ops.push(
                OperationKind::TaskModifyDue {
                    id: current.uuid,
                    new: current.due,
                    old: previous.due,
                }
                .with_timestamp(timestamp),
            );
        }
        (Some(current), None) if current.due.is_some() => {
            ops.push(
                OperationKind::TaskModifyDue {
                    id: current.uuid,
                    new: current.due,
                    old: None,
                }
                .with_timestamp(timestamp),
            );
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.wait != previous.wait => {
            ops.push(
                OperationKind::TaskModifyWait {
                    id: current.uuid,
                    new: current.wait,
                    old: previous.wait,
                }
                .with_timestamp(timestamp),
            );
        }
        (Some(current), None) if current.wait.is_some() => {
            ops.push(
                OperationKind::TaskModifyWait {
                    id: current.uuid,
                    new: current.wait,
                    old: None,
                }
                .with_timestamp(timestamp),
            );
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.project != previous.project => {
            ops.push(
                OperationKind::TaskModifyProject {
                    id: current.uuid,
                    new: current.project.clone().map(Into::into),
                    old: previous.project.clone().map(Into::into),
                }
                .with_timestamp(timestamp),
            );
        }
        (Some(current), None) if current.project.is_some() => {
            ops.push(
                OperationKind::TaskModifyProject {
                    id: current.uuid,
                    new: current.project.clone().map(Into::into),
                    old: None,
                }
                .with_timestamp(timestamp),
            );
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.priority != previous.priority => {
            ops.push(
                OperationKind::TaskModifyPriority {
                    id: current.uuid,
                    new: current.priority,
                    old: previous.priority,
                }
                .with_timestamp(timestamp),
            );
        }
        (Some(current), None) if current.priority.is_some() => {
            ops.push(
                OperationKind::TaskModifyPriority {
                    id: current.uuid,
                    new: current.priority,
                    old: None,
                }
                .with_timestamp(timestamp),
            );
        }
        _ => {}
    }

    match (current, previous) {
        (Some(current), Some(previous)) if current.tags != previous.tags => {
            for (tag, ty) in difference_between(&previous.tags, &current.tags) {
                let kind = match ty {
                    DifferenceType::Addition => OperationKind::TaskModifyAddTag {
                        id: current.uuid,
                        tag: tag.clone().into(),
                    },
                    DifferenceType::Deletion => OperationKind::TaskModifyRemoveTag {
                        id: current.uuid,
                        tag: tag.clone().into(),
                    },
                };
                ops.push(kind.with_now());
            }
        }
        (Some(current), None) => {
            for tag in &current.tags {
                ops.push(
                    OperationKind::TaskModifyAddTag {
                        id: current.uuid,
                        tag: tag.clone().into(),
                    }
                    .with_timestamp(timestamp),
                );
            }
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.annotations != previous.annotations => {
            for (annotation, ty) in difference_between(&previous.annotations, &current.annotations)
            {
                let kind = match ty {
                    DifferenceType::Addition => OperationKind::TaskModifyAddAnnotation {
                        id: current.uuid,
                        annotation: Box::new(annotation.clone()),
                    },
                    DifferenceType::Deletion => OperationKind::TaskModifyRemoveAnnotation {
                        id: current.uuid,
                        annotation: Box::new(annotation.clone()),
                    },
                };
                ops.push(kind.with_timestamp(timestamp));
            }
        }
        (Some(current), None) => {
            for annotation in &current.annotations {
                ops.push(
                    OperationKind::TaskModifyAddAnnotation {
                        id: current.uuid,
                        annotation: Box::new(annotation.clone()),
                    }
                    .with_timestamp(timestamp),
                );
            }
        }
        _ => {}
    }
    match (current, previous) {
        (Some(current), Some(previous)) if current.udas != previous.udas => {
            for (uda, ty) in difference_between(&previous.udas, &current.udas) {
                let kind = match ty {
                    DifferenceType::Addition => OperationKind::TaskModifyAddUda {
                        id: current.uuid,
                        uda: Box::new(uda.clone()),
                    },
                    DifferenceType::Deletion => OperationKind::TaskModifyRemoveUda {
                        id: current.uuid,
                        uda: Box::new(uda.clone()),
                    },
                };
                ops.push(kind.with_timestamp(timestamp));
            }
        }
        (Some(current), None) => {
            for uda in &current.udas {
                ops.push(
                    OperationKind::TaskModifyAddUda {
                        id: current.uuid,
                        uda: Box::new(uda.clone()),
                    }
                    .with_timestamp(timestamp),
                );
            }
        }
        _ => {}
    }
}
