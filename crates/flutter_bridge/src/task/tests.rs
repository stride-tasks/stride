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

    assert_eq!(task.description, "work on ...");
}

const CONSTANT_UUID: uuid::Uuid = uuid::uuid!("01906b2f-ad90-7930-b4d7-24db034bc3c5");
const CONSTANT_UUID_BASE64: &str = "AZBrL62QeTC01yTbA0vDxQ";
const CONSTANT_TIMESTAMP: i64 = 1_719_786_773_674_536;
const CONSTANT_DATETIME: DateTime<Utc> = {
    if let Some(datetime) = DateTime::from_timestamp_micros(CONSTANT_TIMESTAMP) {
        datetime
    } else {
        panic!("timestamp should be correct")
    }
};
const CONSTANT_DATETIME_BASE64: &str = "AAYcIw-7-ig";

#[test]
fn serialize_simple_task() {
    let task = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());

    let data = task.to_data();
    assert_eq!(data, format!("{CONSTANT_UUID_BASE64}Hello there!"));
}

#[test]
fn deserialize_simple_task() {
    let task = Task::from_data(&format!("{CONSTANT_UUID_BASE64}Hello there!")).unwrap();

    assert_eq!(
        task,
        Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned())
    );
}

#[test]
fn serialize_description_with_emoji() {
    let task = Task::with_uuid(CONSTANT_UUID, "do something... maybe 🤔".to_owned());

    let data = task.to_data();
    assert_eq!(
        data,
        format!("{CONSTANT_UUID_BASE64}do something... maybe 🤔")
    );
}

#[test]
fn deserialize_description_with_emoji() {
    let task = Task::from_data(&format!("{CONSTANT_UUID_BASE64}do something... maybe 🤔")).unwrap();

    assert_eq!(
        task,
        Task::with_uuid(CONSTANT_UUID, "do something... maybe 🤔".to_owned())
    );
}

#[test]
fn serialize_description_with_escape_sequence() {
    let task = Task::with_uuid(CONSTANT_UUID, "descri\tion wit\t\"\0\n".to_owned());

    let data = task.to_data();
    assert_eq!(
        data,
        format!("{CONSTANT_UUID_BASE64}descri\\tion wit\\t\"\\0\\n")
    );
}

#[test]
fn deserialize_description_with_escape_sequence() {
    let task = Task::from_data(&format!(
        "{CONSTANT_UUID_BASE64}descri\\tion wit\\t\"\\0\\n"
    ))
    .unwrap();

    assert_eq!(
        task,
        Task::with_uuid(CONSTANT_UUID, "descri\tion wit\t\"\0\n".to_owned())
    );
}

#[test]
fn serialize_task_with_dates() {
    let mut task = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());

    task.modified = Some(CONSTANT_DATETIME);
    assert_eq!(
        task.to_data(),
        format!("{CONSTANT_UUID_BASE64}Hello there!\tm{CONSTANT_DATETIME_BASE64}")
    );

    task.due = Some(CONSTANT_DATETIME);
    assert_eq!(
        task.to_data(),
        format!("{CONSTANT_UUID_BASE64}Hello there!\tm{CONSTANT_DATETIME_BASE64}\td{CONSTANT_DATETIME_BASE64}")
    );
}

#[test]
fn deserialize_task_with_dates() {
    let task = Task::from_data(&format!("{CONSTANT_UUID_BASE64}Hello there!\tm{CONSTANT_DATETIME_BASE64}\td{CONSTANT_DATETIME_BASE64}\tw{CONSTANT_DATETIME_BASE64}")).unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    expected.modified = Some(CONSTANT_DATETIME);
    expected.due = Some(CONSTANT_DATETIME);
    expected.wait = Some(CONSTANT_DATETIME);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_tags() {
    let mut task = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    task.tags = vec![0, 1, 2];

    assert_eq!(
        task.to_data(),
        format!("{CONSTANT_UUID_BASE64}Hello there!\tt0,1,2")
    );
}

#[test]
fn deserialize_task_with_tags() {
    let task = Task::from_data(&format!("{CONSTANT_UUID_BASE64}Hello there!\tt0,1,2")).unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    expected.tags = vec![0, 1, 2];
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_project() {
    let mut task = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());

    task.project = Some(2);
    assert_eq!(
        task.to_data(),
        format!("{CONSTANT_UUID_BASE64}Hello there!\tp2")
    );
}

#[test]
fn deserialize_task_with_project() {
    let task = Task::from_data(&format!("{CONSTANT_UUID_BASE64}Hello there!\tp2")).unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    expected.project = Some(2);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_priority() {
    let mut task = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());

    task.priority = Some(TaskPriority::M);
    assert_eq!(
        task.to_data(),
        format!("{CONSTANT_UUID_BASE64}Hello there!\trM")
    );
}

#[test]
fn deserialize_task_with_priority() {
    let task = Task::from_data(&format!("{CONSTANT_UUID_BASE64}Hello there!\trL")).unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    expected.priority = Some(TaskPriority::L);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_depends() {
    let mut task = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());

    task.depends.push(CONSTANT_UUID);
    task.depends.push(CONSTANT_UUID);
    assert_eq!(
        task.to_data(),
        format!(
            "{CONSTANT_UUID_BASE64}Hello there!\tn{CONSTANT_UUID_BASE64},{CONSTANT_UUID_BASE64}"
        )
    );
}

#[test]
fn deserialize_task_with_depends() {
    let task = Task::from_data(&format!(
        "{CONSTANT_UUID_BASE64}Hello there!\tn{CONSTANT_UUID_BASE64},{CONSTANT_UUID_BASE64}"
    ))
    .unwrap();

    let mut expected = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    expected.depends.push(CONSTANT_UUID);
    expected.depends.push(CONSTANT_UUID);
    assert_eq!(task, expected);
}

// TODO: Add the rest of the attributes
#[test]
fn serialize_task_with_all_attributes() {
    let mut task = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
    task.depends.push(CONSTANT_UUID);
    task.depends.push(CONSTANT_UUID);
    task.modified = Some(CONSTANT_DATETIME);
    task.due = Some(CONSTANT_DATETIME);
    task.wait = Some(CONSTANT_DATETIME);
    task.tags = vec![0, 1, 2];
    task.project = Some(30);
    task.priority = Some(TaskPriority::L);
    assert_eq!(task.to_data(), format!("{CONSTANT_UUID_BASE64}Hello there!\tm{CONSTANT_DATETIME_BASE64}\td{CONSTANT_DATETIME_BASE64}\tw{CONSTANT_DATETIME_BASE64}\tpu\trL\tt0,1,2\tn{CONSTANT_UUID_BASE64},{CONSTANT_UUID_BASE64}"));
}

// TODO: Add the rest of the attributes
#[test]
fn deserialize_task_with_all_attributes() {
    let task = Task::from_data(&format!("{CONSTANT_UUID_BASE64}Hello there!\tm{CONSTANT_DATETIME_BASE64}\td{CONSTANT_DATETIME_BASE64}\tw{CONSTANT_DATETIME_BASE64}\tpu\trH\tt0,1,2\tn{CONSTANT_UUID_BASE64},{CONSTANT_UUID_BASE64}")).unwrap();
    let mut expected = Task::with_uuid(CONSTANT_UUID, "Hello there!".to_owned());
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
