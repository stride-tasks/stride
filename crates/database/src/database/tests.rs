use std::sync::Arc;

use chrono::DateTime;
use stride_core::task::{Task, TaskStatus};
use stride_crdt::{
    actor::ActorId,
    change::{Change, Operation, Sequence, TaskOperation},
    hlc::{Clock, SystemTimeProvider},
};
use uuid::Uuid;

use crate::{Database, Result};

fn init() -> Result<Database> {
    let time_provider = Arc::new(SystemTimeProvider::default());
    let clock = Clock::new(time_provider);

    let actor_id = ActorId::new(Uuid::max());
    let mut db = Database::open_in_memory(actor_id, clock)?;
    db.apply_migrations()?;
    Ok(db)
}

#[test]
fn add_operations_task_create() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = 10_000_000_000;
    let date = DateTime::from_timestamp_micros(timestamp).unwrap();

    let mut db = init()?;

    // let operations = vec![
    //     Operation {
    //         timestamp: date,
    //         kind: Some(OperationKind::TaskCreate { id: Uuid::nil() }),
    //     },
    //     Operation {
    //         timestamp: date,
    //         kind: Some(OperationKind::TaskModifyTitle {
    //             id: Uuid::nil(),
    //             new: Some("Testing".into()),
    //             old: None,
    //         }),
    //     },
    //     Operation {
    //         timestamp: date,
    //         kind: Some(OperationKind::TaskModifyEntry {
    //             id: Uuid::nil(),
    //             new: Some(date),
    //             old: None,
    //         }),
    //     },
    // ];
    let change = Change {
        actor_id: db.actor_id,
        sequence: Sequence::new(1),
        timestamp: db.clock.tick()?,
        operations: vec![
            Operation {
                row_id: Uuid::nil().into(),
                kind: TaskOperation::ModifyTitle {
                    title: "Testing".into(),
                }
                .into(),
            },
            Operation {
                row_id: Uuid::nil().into(),
                kind: TaskOperation::ModifyStatus {
                    status: TaskStatus::Pending,
                }
                .into(),
            },
            Operation {
                row_id: Uuid::nil().into(),
                kind: TaskOperation::ModifyEntry { entry: date }.into(),
            },
        ],
    };
    let mut transaction = db.transaction()?;
    transaction.apply_change(&change)?;
    transaction.commit()?;

    let tasks = db.all_tasks()?;

    assert_eq!(tasks.len(), 1);

    let mut task = Task::with_id(Uuid::nil(), "Testing".into());
    task.entry = Some(date);
    assert_eq!(tasks[0], task);

    Ok(())
}
