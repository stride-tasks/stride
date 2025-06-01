use std::{collections::HashSet, hash::Hash};

use stride_core::task::Task;

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
fn push_operations_diff_task_common(current: &Task, previous: &Task, ops: &mut Vec<Operation>) {
    if current.status != previous.status {
        ops.push(
            OperationKind::TaskModifyStatus {
                id: current.uuid,
                new: current.status,
                old: previous.status,
            }
            .with_now(),
        );
    }
    if current.active != previous.active {
        ops.push(
            OperationKind::TaskModifyActive {
                id: current.uuid,
                new: current.active,
                old: previous.active,
            }
            .with_now(),
        );
    }
    if current.modified != previous.modified {
        ops.push(
            OperationKind::TaskModifyModified {
                id: current.uuid,
                new: current.modified,
                old: previous.modified,
            }
            .with_now(),
        );
    }
    if current.due != previous.due {
        ops.push(
            OperationKind::TaskModifyDue {
                id: current.uuid,
                new: current.due,
                old: previous.due,
            }
            .with_now(),
        );
    }
    if current.wait != previous.wait {
        ops.push(
            OperationKind::TaskModifyWait {
                id: current.uuid,
                new: current.wait,
                old: previous.wait,
            }
            .with_now(),
        );
    }
    if current.project != previous.project {
        ops.push(
            OperationKind::TaskModifyProject {
                id: current.uuid,
                new: current.project.clone().map(String::into_boxed_str),
                old: previous.project.clone().map(String::into_boxed_str),
            }
            .with_now(),
        );
    }
    if current.priority != previous.priority {
        ops.push(
            OperationKind::TaskModifyPriority {
                id: current.uuid,
                new: current.priority,
                old: previous.priority,
            }
            .with_now(),
        );
    }
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
    for (annotation, ty) in difference_between(&previous.annotations, &current.annotations) {
        let kind = match ty {
            DifferenceType::Addition => OperationKind::TaskModifyAddAnnotation {
                id: current.uuid,
                annotation: annotation.clone().into(),
            },
            DifferenceType::Deletion => OperationKind::TaskModifyRemoveAnnotation {
                id: current.uuid,
                annotation: annotation.clone().into(),
            },
        };
        ops.push(kind.with_now());
    }
    for (uda, ty) in difference_between(&previous.udas, &current.udas) {
        let kind = match ty {
            DifferenceType::Addition => OperationKind::TaskModifyAddUda {
                id: current.uuid,
                uda: uda.clone().into(),
            },
            DifferenceType::Deletion => OperationKind::TaskModifyRemoveUda {
                id: current.uuid,
                uda: uda.clone().into(),
            },
        };
        ops.push(kind.with_now());
    }
}

pub(crate) fn push_operations_diff_task(current: &Task, previous: &Task, ops: &mut Vec<Operation>) {
    if current.title != previous.title {
        ops.push(
            OperationKind::TaskModifyTitle {
                id: current.uuid,
                new: current.title.clone().into_boxed_str(),
                old: previous.title.clone().into_boxed_str(),
            }
            .with_now(),
        );
    }
    if current.entry != previous.entry {
        ops.push(
            OperationKind::TaskModifyEntry {
                id: current.uuid,
                new: current.entry,
                old: previous.entry,
            }
            .with_now(),
        );
    }
    push_operations_diff_task_common(current, previous, ops);
}

pub(crate) fn push_operations_diff_task_with_default(current: &Task, ops: &mut Vec<Operation>) {
    ops.push(
        OperationKind::TaskCreate {
            id: current.uuid,
            title: current.title.clone().into(),
        }
        .with_now(),
    );

    let previous = Task::default();
    push_operations_diff_task_common(current, &previous, ops);

    // task.depends;
}
