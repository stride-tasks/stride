#![allow(clippy::cast_possible_truncation)]

use chrono::{DateTime, Utc};

use crate::task::{Annotation, Task, TaskPriority, TaskStatus, Uda};

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

const CONSTANT_UUID: uuid::Uuid = uuid::uuid!("01906b2f-ad90-7930-b4d7-24db034bc3c5");
const CONSTANT_UUID_BYTES: &[u8] = CONSTANT_UUID.as_bytes();
const CONSTANT_TIMESTAMP: i64 = 1_719_786_773_674_536;
const CONSTANT_DATETIME_BYTES: &[u8] = &CONSTANT_TIMESTAMP.to_be_bytes();
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

fn create_task(title: &str) -> Task {
    let mut task = Task::with_uuid(CONSTANT_UUID, title.to_string());
    task.entry = CONSTANT_DATETIME;
    task
}

#[test]
fn serialize_simple_task() {
    let task = create_task("Hello there!");

    let data = task.to_data();

    let title = b"Hello there!";
    assert_eq!(
        data,
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_slice(),
        ])
    );
}

#[test]
fn deserialize_simple_task() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
    ]))
    .unwrap();

    assert_eq!(task, create_task(title));
}

#[test]
fn serialize_title_with_emoji() {
    let title = "do something... maybe 🤔";
    let task = create_task(title);
    let data = task.to_data();
    assert_eq!(
        data,
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
        ])
    );
}

#[test]
fn deserialize_title_with_emoji() {
    let title = "do something... maybe 🤔";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
    ]))
    .unwrap();

    assert_eq!(task, create_task(title));
}

#[test]
fn serialize_title_with_escape_sequence() {
    let title = "descri\tion wit\t\"\0\n";
    let task = create_task(title);
    let data = task.to_data();
    assert_eq!(
        data,
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
        ])
    );
}

#[test]
fn deserialize_title_with_escape_sequence() {
    let title = "descri\tion wit\t\"\0\n";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
    ]))
    .unwrap();

    assert_eq!(task, create_task(title));
}

#[test]
fn serialize_task_with_dates() {
    let title = "Hello there!";
    let mut task = create_task(title);
    task.modified = Some(CONSTANT_DATETIME);
    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"m",
            CONSTANT_TIMESTAMP.to_be_bytes().as_slice()
        ])
    );

    task.due = Some(CONSTANT_DATETIME);
    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"m",
            CONSTANT_TIMESTAMP.to_be_bytes().as_slice(),
            b"d",
            CONSTANT_TIMESTAMP.to_be_bytes().as_slice()
        ])
    );
}

#[test]
fn deserialize_task_with_dates() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        b"m",
        CONSTANT_TIMESTAMP.to_be_bytes().as_slice(),
        b"d",
        CONSTANT_TIMESTAMP.to_be_bytes().as_slice(),
        b"w",
        CONSTANT_TIMESTAMP.to_be_bytes().as_slice(),
    ]))
    .unwrap();

    let mut expected = create_task(title);
    expected.modified = Some(CONSTANT_DATETIME);
    expected.due = Some(CONSTANT_DATETIME);
    expected.wait = Some(CONSTANT_DATETIME);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_tags() {
    let title = "Hello there!";
    let mut task = create_task(title);
    task.tags = vec!["tag1".into(), "tag22".into(), "tag333".into()];

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"t",
            &4u32.to_be_bytes(),
            b"tag1",
            b"t",
            &5u32.to_be_bytes(),
            b"tag22",
            b"t",
            &6u32.to_be_bytes(),
            b"tag333",
        ])
    );
}

#[test]
fn deserialize_task_with_tags() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        b"t",
        &4u32.to_be_bytes(),
        b"tag1",
        b"t",
        &5u32.to_be_bytes(),
        b"tag22",
        b"t",
        &6u32.to_be_bytes(),
        b"tag333",
    ]))
    .unwrap();

    let mut expected = create_task(title);
    expected.tags = vec!["tag1".into(), "tag22".into(), "tag333".into()];
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_project() {
    let title = "Hello there!";
    let mut task = create_task(title);
    task.project = Some("work".into());

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"p",
            &4u32.to_be_bytes(),
            b"work",
        ])
    );
}

#[test]
fn deserialize_task_with_project() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        b"p",
        &4u32.to_be_bytes(),
        b"work",
    ]))
    .unwrap();

    let mut expected = create_task(title);
    expected.project = Some("work".into());
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_priority() {
    let title = "Hello there!";
    let mut task = create_task(title);
    task.priority = Some(TaskPriority::M);

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"rM",
        ])
    );
}

#[test]
fn deserialize_task_with_priority() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        b"rL",
    ]))
    .unwrap();

    let mut expected = create_task("Hello there!");
    expected.priority = Some(TaskPriority::L);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_depends() {
    let title = "Hello there!";
    let mut task = create_task(title);
    task.depends.push(CONSTANT_UUID);
    task.depends.push(CONSTANT_UUID);

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"n",
            CONSTANT_UUID_BYTES,
            b"n",
            CONSTANT_UUID_BYTES,
        ])
    );
}

#[test]
fn deserialize_task_with_depends() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        b"n",
        CONSTANT_UUID_BYTES,
        b"n",
        CONSTANT_UUID_BYTES,
    ]))
    .unwrap();

    let mut expected = create_task(title);
    expected.depends.push(CONSTANT_UUID);
    expected.depends.push(CONSTANT_UUID);
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_annotations() {
    let title = "Hello there!";
    let mut task = create_task(title);
    task.annotations.push(Annotation {
        entry: CONSTANT_DATETIME,
        description: String::from("Hello"),
    });
    task.annotations.push(Annotation {
        entry: CONSTANT_DATETIME,
        description: String::from("World"),
    });

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"a",
            &2u32.to_be_bytes(),
            CONSTANT_DATETIME_BYTES,
            &5u32.to_be_bytes(),
            b"Hello",
            CONSTANT_DATETIME_BYTES,
            &5u32.to_be_bytes(),
            b"World",
        ])
    );
}

#[test]
fn deserialize_task_with_annotations() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        b"a",
        &2u32.to_be_bytes(),
        CONSTANT_DATETIME_BYTES,
        &5u32.to_be_bytes(),
        b"Hello",
        CONSTANT_DATETIME_BYTES,
        &5u32.to_be_bytes(),
        b"World",
    ]))
    .unwrap();

    let mut expected = create_task(title);
    expected.annotations.push(Annotation {
        entry: CONSTANT_DATETIME,
        description: String::from("Hello"),
    });
    expected.annotations.push(Annotation {
        entry: CONSTANT_DATETIME,
        description: String::from("World"),
    });
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_udas() {
    let title = "Hello there!";
    let mut task = create_task(title);
    task.udas.push(Uda {
        namespace: "Hello".into(),
        key: "World".into(),
        value: "!!".into(),
    });
    task.udas.push(Uda {
        namespace: "namespace".into(),
        key: "key".into(),
        value: b"value".into(),
    });

    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"u",
            &2u32.to_be_bytes(),
            &5u32.to_be_bytes(),
            b"Hello",
            &5u32.to_be_bytes(),
            b"World",
            &2u32.to_be_bytes(),
            b"!!",
            &9u32.to_be_bytes(),
            b"namespace",
            &3u32.to_be_bytes(),
            b"key",
            &5u32.to_be_bytes(),
            b"value",
        ])
    );
}

#[test]
fn deserialize_task_with_udas() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        b"u",
        &2u32.to_be_bytes(),
        &5u32.to_be_bytes(),
        b"Hello",
        &5u32.to_be_bytes(),
        b"World",
        &2u32.to_be_bytes(),
        b"!!",
        &9u32.to_be_bytes(),
        b"namespace",
        &3u32.to_be_bytes(),
        b"key",
        &5u32.to_be_bytes(),
        b"value",
    ]))
    .unwrap();

    let mut expected = create_task(title);
    expected.udas.push(Uda {
        namespace: "Hello".into(),
        key: "World".into(),
        value: "!!".into(),
    });
    expected.udas.push(Uda {
        namespace: "namespace".into(),
        key: "key".into(),
        value: b"value".into(),
    });
    assert_eq!(task, expected);
}

#[test]
fn serialize_task_with_all_attributes() {
    let title = "Hello there!";
    let mut task = create_task(title);
    task.active = true;
    task.depends.push(CONSTANT_UUID);
    task.depends.push(CONSTANT_UUID);
    task.modified = Some(CONSTANT_DATETIME);
    task.due = Some(CONSTANT_DATETIME);
    task.wait = Some(CONSTANT_DATETIME);
    task.tags = vec!["tag1".into(), "tag22".into(), "tag333".into()];
    task.project = Some("test".into());
    task.priority = Some(TaskPriority::L);
    task.annotations.push(Annotation {
        entry: CONSTANT_DATETIME,
        description: String::from("Hello"),
    });
    task.annotations.push(Annotation {
        entry: CONSTANT_DATETIME,
        description: String::from("World"),
    });
    task.udas.push(Uda {
        namespace: "Hello".into(),
        key: "World".into(),
        value: "!!".into(),
    });
    task.udas.push(Uda {
        namespace: "namespace".into(),
        key: "key".into(),
        value: b"value".into(),
    });
    assert_eq!(
        task.to_data(),
        concat(&[
            CONSTANT_UUID_BYTES,
            CONSTANT_DATETIME_BYTES,
            (title.len() as u32).to_be_bytes().as_slice(),
            title.as_bytes(),
            b"A",
            b"m",
            CONSTANT_DATETIME_BYTES,
            b"d",
            CONSTANT_DATETIME_BYTES,
            b"w",
            CONSTANT_DATETIME_BYTES,
            b"p",
            &4u32.to_be_bytes(),
            b"test",
            b"rL",
            b"t",
            4u32.to_be_bytes().as_slice(),
            b"tag1",
            b"t",
            5u32.to_be_bytes().as_slice(),
            b"tag22",
            b"t",
            6u32.to_be_bytes().as_slice(),
            b"tag333",
            b"n",
            CONSTANT_UUID_BYTES,
            b"n",
            CONSTANT_UUID_BYTES,
            b"a",
            &2u32.to_be_bytes(),
            CONSTANT_DATETIME_BYTES,
            &5u32.to_be_bytes(),
            b"Hello",
            CONSTANT_DATETIME_BYTES,
            &5u32.to_be_bytes(),
            b"World",
            b"u",
            &2u32.to_be_bytes(),
            &5u32.to_be_bytes(),
            b"Hello",
            &5u32.to_be_bytes(),
            b"World",
            &2u32.to_be_bytes(),
            b"!!",
            &9u32.to_be_bytes(),
            b"namespace",
            &3u32.to_be_bytes(),
            b"key",
            &5u32.to_be_bytes(),
            b"value",
        ])
    );
}

// TODO: Add the rest of the attributes
#[test]
fn deserialize_task_with_all_attributes() {
    let title = "Hello there!";
    let task = Task::from_data(&concat(&[
        CONSTANT_UUID_BYTES,
        CONSTANT_DATETIME_BYTES,
        (title.len() as u32).to_be_bytes().as_slice(),
        title.as_bytes(),
        b"A",
        b"m",
        CONSTANT_DATETIME_BYTES,
        b"d",
        CONSTANT_DATETIME_BYTES,
        b"w",
        CONSTANT_DATETIME_BYTES,
        b"p",
        &4u32.to_be_bytes(),
        b"test",
        b"rH",
        b"t",
        &4u32.to_be_bytes(),
        b"tag1",
        b"t",
        &5u32.to_be_bytes(),
        b"tag22",
        b"t",
        &6u32.to_be_bytes(),
        b"tag333",
        b"n",
        CONSTANT_UUID_BYTES,
        b"n",
        CONSTANT_UUID_BYTES,
        b"a",
        &2u32.to_be_bytes(),
        CONSTANT_DATETIME_BYTES,
        &5u32.to_be_bytes(),
        b"Hello",
        CONSTANT_DATETIME_BYTES,
        &5u32.to_be_bytes(),
        b"World",
        b"u",
        &2u32.to_be_bytes(),
        &5u32.to_be_bytes(),
        b"Hello",
        &5u32.to_be_bytes(),
        b"World",
        &2u32.to_be_bytes(),
        b"!!",
        &9u32.to_be_bytes(),
        b"namespace",
        &3u32.to_be_bytes(),
        b"key",
        &5u32.to_be_bytes(),
        b"value",
    ]))
    .unwrap();

    let mut expected = create_task("Hello there!");
    expected.active = true;
    expected.depends.push(CONSTANT_UUID);
    expected.depends.push(CONSTANT_UUID);
    expected.modified = Some(CONSTANT_DATETIME);
    expected.due = Some(CONSTANT_DATETIME);
    expected.wait = Some(CONSTANT_DATETIME);
    expected.tags = vec!["tag1".into(), "tag22".into(), "tag333".into()];
    expected.project = Some("test".into());
    expected.priority = Some(TaskPriority::H);
    expected.annotations.push(Annotation {
        entry: CONSTANT_DATETIME,
        description: String::from("Hello"),
    });
    expected.annotations.push(Annotation {
        entry: CONSTANT_DATETIME,
        description: String::from("World"),
    });
    expected.udas.push(Uda {
        namespace: "Hello".into(),
        key: "World".into(),
        value: "!!".into(),
    });
    expected.udas.push(Uda {
        namespace: "namespace".into(),
        key: "key".into(),
        value: b"value".into(),
    });
    assert_eq!(task, expected);
}
