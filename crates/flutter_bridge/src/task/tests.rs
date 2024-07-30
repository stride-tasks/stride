#![allow(clippy::cast_possible_truncation)]

use chrono::{DateTime, Utc};

use crate::task::{Task, TaskPriority};

use super::TaskStatus;

#[test]
fn conversion_task_status() -> anyhow::Result<()> {
    assert_eq!(
        serde_json::to_string(&TaskStatus::Complete)?,
        "\"complete\""
    );
    assert_eq!(serde_json::to_string(&TaskStatus::Deleted)?, "\"deleted\"");
    assert_eq!(serde_json::to_string(&TaskStatus::Pending)?, "\"pending\"");
    assert_eq!(
        serde_json::to_string(&TaskStatus::Recurring)?,
        "\"recurring\""
    );
    assert_eq!(serde_json::to_string(&TaskStatus::Waiting)?, "\"waiting\"");
    Ok(())
}

#[test]
fn create_task() {
    let task = Task::new("work on ...".to_owned());

    assert_eq!(task.title, "work on ...");
}

const CONSTANT_UUID: uuid::Uuid = uuid::uuid!("01906b2f-ad90-7930-b4d7-24db034bc3c5");
const CONSTANT_TIMESTAMP: i64 = 1_719_786_773_674_536;
const CONSTANT_DATETIME: DateTime<Utc> = {
    if let Some(datetime) = DateTime::from_timestamp_micros(CONSTANT_TIMESTAMP) {
        datetime
    } else {
        panic!("timestamp should be correct")
    }
};

fn concat(data: &[&[u8]]) -> Vec<u8> {
    let mut result = Vec::new();
    for slice in data {
        result.extend_from_slice(slice);
    }
    result
}

#[test]
fn serialize_simple_task() {
    let task = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());

    let data = task.to_data();

    let title = b"Hello there!";
    assert_eq!(
        data,
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_slice(),
        ]) // format!("{CONSTANT_UUID_BASE64}Hello there!").as_bytes()
    );
}

#[test]
fn deserialize_simple_task() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
    ]))
    .unwrap();

    assert_eq!(task, Task::with_uuid(CONSTANT_UUID, title.to_owned()));
}

#[test]
fn serialize_title_with_emoji() {
    let title = "do something... maybe 🤔";
    let task = Task::with_uuid(CONSTANT_UUID, title.to_owned());

    let data = task.to_data();
    assert_eq!(
        data,
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
        ])
    );
}

#[test]
fn deserialize_title_with_emoji() {
    let title = "do something... maybe 🤔";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
    ]))
    .unwrap();

    assert_eq!(task, Task::with_uuid(CONSTANT_UUID, title.to_owned()));
}

#[test]
fn serialize_title_with_escape_sequence() {
    let title = "descri\tion wit\t\"\0\n";
    let task = Task::with_uuid(CONSTANT_UUID, title.to_owned());

    let data = task.to_data();
    assert_eq!(
        data,
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
        ])
    );
}

#[test]
fn deserialize_title_with_escape_sequence() {
    let title = "descri\tion wit\t\"\0\n";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
    ]))
    .unwrap();

    assert_eq!(task, Task::with_uuid(CONSTANT_UUID, title.to_owned()));
}

#[test]
fn serialize_task_with_dates() {
    let title = "Hello there!";
    let mut task = Task::with_uuid(CONSTANT_UUID, title.to_owned());

    task.modified = Some(CONSTANT_DATETIME);
    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            &[b'm'],
            CONSTANT_TIMESTAMP.to_be_bytes().as_slice()
        ])
    );

    task.due = Some(CONSTANT_DATETIME);
    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            &[b'm'],
            CONSTANT_TIMESTAMP.to_be_bytes().as_slice(),
            &[b'd'],
            CONSTANT_TIMESTAMP.to_be_bytes().as_slice()
        ])
    );
}

#[test]
fn deserialize_task_with_dates() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        &[b'm'],
        CONSTANT_TIMESTAMP.to_be_bytes().as_slice(),
        &[b'd'],
        CONSTANT_TIMESTAMP.to_be_bytes().as_slice(),
        &[b'w'],
        CONSTANT_TIMESTAMP.to_be_bytes().as_slice(),
    ]))
    .unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    expected.modified = Some(CONSTANT_DATETIME);
    expected.due = Some(CONSTANT_DATETIME);
    expected.wait = Some(CONSTANT_DATETIME);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_tags() {
    let title = "Hello there!";
    let mut task = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    task.tags = vec![0, 1, 2];

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            &[b't'],
            0u32.to_be_bytes().as_slice(),
            &[b't'],
            1u32.to_be_bytes().as_slice(),
            &[b't'],
            2u32.to_be_bytes().as_slice(),
        ])
    );
}

#[test]
fn deserialize_task_with_tags() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        &[b't'],
        0u32.to_be_bytes().as_slice(),
        &[b't'],
        1u32.to_be_bytes().as_slice(),
        &[b't'],
        2u32.to_be_bytes().as_slice(),
    ]))
    .unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    expected.tags = vec![0, 1, 2];
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_project() {
    let title = "Hello there!";
    let mut task = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    task.project = Some(2);

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            &[b'p'],
            2u32.to_be_bytes().as_slice(),
        ])
    );
}

#[test]
fn deserialize_task_with_project() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        &[b'p'],
        2u32.to_be_bytes().as_slice(),
    ]))
    .unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    expected.project = Some(2);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_priority() {
    let title = "Hello there!";
    let mut task = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    task.priority = Some(TaskPriority::M);

    assert_eq!(
        task.to_data(),
        // format!("{CONSTANT_UUID_BASE64}Hello there!\trM")
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            &[b'r', b'M'],
        ])
    );
}

#[test]
fn deserialize_task_with_priority() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        &[b'r', b'L'],
    ]))
    .unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    expected.priority = Some(TaskPriority::L);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_depends() {
    let title = "Hello there!";
    let mut task = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    task.depends.push(CONSTANT_UUID);
    task.depends.push(CONSTANT_UUID);

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            &[b'n'],
            CONSTANT_UUID.as_bytes(),
            &[b'n'],
            CONSTANT_UUID.as_bytes(),
        ])
    );
}

#[test]
fn deserialize_task_with_depends() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        &[b'n'],
        CONSTANT_UUID.as_bytes(),
        &[b'n'],
        CONSTANT_UUID.as_bytes(),
    ]))
    .unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    expected.depends.push(CONSTANT_UUID);
    expected.depends.push(CONSTANT_UUID);
    assert_eq!(task, expected);
}

// TODO: Add the rest of the attributes
#[test]
fn serialize_task_with_all_attributes() {
    let title = "Hello there!";
    let mut task = Task::with_uuid(CONSTANT_UUID, title.to_owned());
    task.active = true;
    task.depends.push(CONSTANT_UUID);
    task.depends.push(CONSTANT_UUID);
    task.modified = Some(CONSTANT_DATETIME);
    task.due = Some(CONSTANT_DATETIME);
    task.wait = Some(CONSTANT_DATETIME);
    task.tags = vec![0, 1, 2];
    task.project = Some(30);
    task.priority = Some(TaskPriority::L);
    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID.as_bytes(),
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            &[b'A'],
            &[b'm'],
            &CONSTANT_TIMESTAMP.to_be_bytes(),
            &[b'd'],
            &CONSTANT_TIMESTAMP.to_be_bytes(),
            &[b'w'],
            &CONSTANT_TIMESTAMP.to_be_bytes(),
            &[b'p'],
            &30u32.to_be_bytes(),
            &[b'r', b'L'],
            &[b't'],
            &0u32.to_be_bytes(),
            &[b't'],
            &1u32.to_be_bytes(),
            &[b't'],
            &2u32.to_be_bytes(),
            &[b'n'],
            CONSTANT_UUID.as_bytes(),
            &[b'n'],
            CONSTANT_UUID.as_bytes(),
        ])
    );
}

// TODO: Add the rest of the attributes
#[test]
fn deserialize_task_with_all_attributes() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID.as_bytes(),
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        &[b'A'],
        &[b'm'],
        &CONSTANT_TIMESTAMP.to_be_bytes(),
        &[b'd'],
        &CONSTANT_TIMESTAMP.to_be_bytes(),
        &[b'w'],
        &CONSTANT_TIMESTAMP.to_be_bytes(),
        &[b'p'],
        &30u32.to_be_bytes(),
        &[b'r', b'H'],
        &[b't'],
        &0u32.to_be_bytes(),
        &[b't'],
        &1u32.to_be_bytes(),
        &[b't'],
        &2u32.to_be_bytes(),
        &[b'n'],
        CONSTANT_UUID.as_bytes(),
        &[b'n'],
        CONSTANT_UUID.as_bytes(),
    ]))
    .unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    expected.active = true;
    expected.depends.push(CONSTANT_UUID);
    expected.depends.push(CONSTANT_UUID);
    expected.modified = Some(CONSTANT_DATETIME);
    expected.due = Some(CONSTANT_DATETIME);
    expected.wait = Some(CONSTANT_DATETIME);
    expected.tags = vec![0, 1, 2];
    expected.project = Some(30);
    expected.priority = Some(TaskPriority::H);
    assert_eq!(task, expected);
}
