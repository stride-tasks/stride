use chrono::{DateTime, Days};
use stride_core::task::Task;
use uuid::Uuid;

use crate::{
    Database,
    operation::{Operation, OperationKind},
};

#[test]
fn add_operations_task_create() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = 10_000_000_000;
    let date = DateTime::from_timestamp_micros(timestamp).unwrap();

    let mut db = Database::open_in_memory()?;
    db.apply_migrations()?;

    let operation = Operation {
        timestamp: date,
        kind: Some(OperationKind::TaskCreate {
            id: Uuid::nil(),
            title: "Testing".into(),
            entry: date,
        }),
    };
    let mut transaction = db.transaction()?;
    transaction.apply(vec![operation.clone()])?;
    transaction.commit()?;

    let tasks = db.all_tasks()?;

    assert_eq!(tasks.len(), 1);

    let mut task = Task::with_uuid(Uuid::nil(), "Testing".into());
    task.entry = date;
    assert_eq!(tasks[0], task);

    let operations = db
        .undoable_operation(usize::MAX)?
        .into_iter()
        .map(|(_, operation)| operation)
        .collect::<Vec<_>>();

    assert_eq!(operations.len(), 1);

    assert_eq!(operations[0], operation);

    Ok(())
}

#[test]
fn add_operations_modify_entry() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = 10_000_000_000;
    let date = DateTime::from_timestamp_micros(timestamp).unwrap();
    let new_date = date.checked_add_days(Days::new(10)).unwrap();

    let task_id = Uuid::nil();

    let mut db = Database::open_in_memory()?;
    db.apply_migrations()?;

    let operations = vec![
        Operation {
            timestamp: date,
            kind: Some(OperationKind::TaskCreate {
                id: task_id,
                title: "Testing".into(),
                entry: date,
            }),
        },
        Operation {
            timestamp: new_date,
            kind: Some(OperationKind::TaskModifyEntry {
                id: task_id,
                new: new_date,
                old: date,
            }),
        },
    ];
    let mut transaction = db.transaction()?;
    transaction.apply(operations.clone())?;
    transaction.commit()?;

    let tasks = db.all_tasks()?;

    assert_eq!(tasks.len(), 1);

    let mut task = Task::with_uuid(task_id, "Testing".into());
    task.entry = new_date;
    assert_eq!(tasks[0], task);

    let db_operations = db
        .undoable_operation(usize::MAX)?
        .into_iter()
        .map(|(_, operation)| operation)
        .rev()
        .collect::<Vec<_>>();

    assert_eq!(db_operations, operations);

    Ok(())
}

#[test]
fn add_operations_modify_title() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = 10_000_000_000;
    let date = DateTime::from_timestamp_micros(timestamp).unwrap();
    let new_date = date.checked_add_days(Days::new(10)).unwrap();

    let task_id = Uuid::nil();

    let mut db = Database::open_in_memory()?;
    db.apply_migrations()?;

    let operations = vec![
        Operation {
            timestamp: date,
            kind: Some(OperationKind::TaskCreate {
                id: task_id,
                title: "Testing".into(),
                entry: date,
            }),
        },
        Operation {
            timestamp: new_date,
            kind: Some(OperationKind::TaskModifyTitle {
                id: task_id,
                new: "123".into(),
                old: "Testing".into(),
            }),
        },
    ];
    let mut transaction = db.transaction()?;
    transaction.apply(operations.clone())?;
    transaction.commit()?;

    let tasks = db.all_tasks()?;

    assert_eq!(tasks.len(), 1);

    let mut task = Task::with_uuid(task_id, "123".into());
    task.entry = date;
    assert_eq!(tasks[0], task);

    let db_operations = db
        .undoable_operation(usize::MAX)?
        .into_iter()
        .map(|(_, operation)| operation)
        .rev()
        .collect::<Vec<_>>();

    assert_eq!(db_operations, operations);

    Ok(())
}
