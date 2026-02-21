use stride_core::task::{Annotation, Date, Task, TaskPriority, TaskStatus, Uda};
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub(crate) fn task_to_data(task: &Task) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend_from_slice(b"\0".as_slice());
    result.extend_from_slice(task.uuid.as_bytes());
    if let Some(entry) = task.entry {
        result.push(b'e');
        result.extend_from_slice(&entry.timestamp_micros().to_be_bytes());
    }
    if let Some(title) = &task.title {
        result.push(b'T');
        result.extend_from_slice(&(title.len() as u32).to_be_bytes());
        result.extend_from_slice(title.as_bytes());
    }
    if let Some(modified) = task.modified {
        result.push(b'm');
        result.extend(&modified.timestamp_micros().to_be_bytes());
    }
    if let Some(due) = task.due {
        result.push(b'd');
        result.extend(&due.timestamp_micros().to_be_bytes());
    }
    if let Some(wait) = task.wait {
        result.push(b'w');
        result.extend(&wait.timestamp_micros().to_be_bytes());
    }
    if let Some(project) = &task.project {
        result.push(b'p');

        let len = project.len() as u32;
        result.extend_from_slice(&len.to_be_bytes());
        result.extend_from_slice(project.as_bytes());
    }
    if let Some(priority) = task.priority {
        result.push(b'r');
        result.push(priority.as_str().as_bytes()[0]);
    }
    for tag in &task.tags {
        result.push(b't');

        let len = tag.len() as u32;
        result.extend_from_slice(&len.to_be_bytes());
        result.extend_from_slice(tag.as_bytes());
    }
    for depend in &task.depends {
        result.push(b'n');
        result.extend_from_slice(depend.as_bytes());
    }
    if !task.annotations.is_empty() {
        result.push(b'a');
        result.extend_from_slice(&(task.annotations.len() as u32).to_be_bytes());
        for annotation in &task.annotations {
            result.extend(&annotation.entry.timestamp_micros().to_be_bytes());
            result.extend_from_slice(&(annotation.description.len() as u32).to_be_bytes());
            result.extend_from_slice(annotation.description.as_bytes());
        }
    }
    if !task.udas.is_empty() {
        result.push(b'u');
        result.extend_from_slice(&(task.udas.len() as u32).to_be_bytes());
        for uda in &task.udas {
            result.extend_from_slice(&(uda.key.len() as u32).to_be_bytes());
            result.extend_from_slice(uda.key.as_bytes());
            result.extend_from_slice(&(uda.value.len() as u32).to_be_bytes());
            result.extend_from_slice(uda.value.as_bytes());
        }
    }
    result
}

// TODO(HalidOdat): Return Result<> with error indicating what is wrong.
#[allow(clippy::too_many_lines)]
#[must_use]
pub(crate) fn task_from_data(input: &[u8]) -> Option<Task> {
    let (version, input) = input.split_first_chunk::<1>()?;
    if version[0] != 0 {
        return None;
    }

    let (uuid_bytes, input) = input.split_first_chunk::<16>()?;
    let uuid = Uuid::from_bytes(*uuid_bytes);

    let mut entry = None;
    let mut title = None;
    let mut modified = None;
    let mut due = None;
    let mut project = None;
    let mut priority = None;
    let mut wait = None;
    let mut depends = Vec::new();
    let mut tags = Vec::new();
    let mut annotations = Vec::new();
    let mut udas = Vec::new();
    let mut i = 0;
    while i < input.len() {
        let Some(typ) = input.get(i).copied() else {
            break;
        };
        i += 1;
        match typ {
            b'T' => {
                let len = u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?)
                    as usize;
                i += size_of::<u32>();

                let bytes = input.get(i..i + len)?;
                i += len;
                let value = std::str::from_utf8(bytes).ok()?.to_string();

                title = Some(value);
            }
            b'e' | b'm' | b'd' | b'w' => {
                let timestamp = input.get(i..(i + size_of::<i64>()))?;
                let timestamp = i64::from_be_bytes(timestamp.try_into().ok()?);
                let datetime = Date::from_timestamp_micros(timestamp)?;

                i += size_of::<i64>();

                match typ {
                    b'e' => entry = Some(datetime),
                    b'm' => modified = Some(datetime),
                    b'd' => due = Some(datetime),
                    b'w' => wait = Some(datetime),
                    _ => unreachable!(),
                }
            }
            b'n' => {
                let uuid = Uuid::from_bytes(input.get(i..i + size_of::<Uuid>())?.try_into().ok()?);
                depends.push(uuid);
                i += size_of::<Uuid>();
            }
            b't' => {
                let len = u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?)
                    as usize;
                i += size_of::<u32>();

                let bytes = input.get(i..i + len)?;
                i += len;
                let value = std::str::from_utf8(bytes).ok()?.to_string();

                if !tags.contains(&value) {
                    tags.push(value);
                }
            }
            b'p' => {
                let len = u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?)
                    as usize;
                i += size_of::<u32>();

                let bytes = input.get(i..i + len)?;
                i += len;
                let value = std::str::from_utf8(bytes).ok()?.to_string();

                project = Some(value);
            }
            b'r' => {
                let value = match input.get(i)? {
                    b'H' => TaskPriority::H,
                    b'M' => TaskPriority::M,
                    b'L' => TaskPriority::L,
                    _ => return None,
                };
                i += 1;
                priority = Some(value);
            }
            b'a' => {
                let len = u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?);
                i += size_of::<u32>();

                for _ in 0..len {
                    let timestamp = input.get(i..(i + size_of::<i64>()))?;
                    let timestamp = i64::from_be_bytes(timestamp.try_into().ok()?);
                    let datetime = Date::from_timestamp_micros(timestamp)?;
                    i += size_of::<i64>();

                    let title_len =
                        u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?)
                            as usize;
                    i += size_of::<u32>();

                    let title_bytes = input.get(i..i + title_len)?;
                    i += title_len;

                    let title = std::str::from_utf8(title_bytes).ok()?;

                    annotations.push(Annotation {
                        entry: datetime,
                        description: title.to_string(),
                    });
                }
            }
            b'u' => {
                let len = u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?);
                i += size_of::<u32>();

                for _ in 0..len {
                    let key_len =
                        u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?)
                            as usize;
                    i += size_of::<u32>();

                    let key_bytes = input.get(i..i + key_len)?;
                    i += key_len;

                    let key = std::str::from_utf8(key_bytes).ok()?;

                    let value_len =
                        u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?)
                            as usize;
                    i += size_of::<u32>();

                    let value_bytes = input.get(i..i + value_len)?;
                    i += value_len;

                    let value = std::str::from_utf8(value_bytes).ok()?;

                    udas.push(Uda {
                        key: key.into(),
                        value: value.into(),
                    });
                }
            }
            _ => return None,
        }
    }

    Some(Task {
        uuid,
        entry,
        title,
        status: TaskStatus::Pending,
        modified,
        due,
        project,
        tags,
        annotations,
        priority,
        wait,
        depends,
        udas,
    })
}
