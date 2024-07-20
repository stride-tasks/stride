use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub mod annotation;

pub type Date = DateTime<Utc>;

pub use annotation::Annotation;
use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    escape::{escape, unescape},
    ToBase64Array,
};
pub type TagIndex = u32;
pub type ProjectIndex = u32;
pub type PriorityIndex = u32;

#[cfg(test)]
mod tests;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TaskStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "recurring")]
    Recurring,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "complete")]
    Complete,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TaskPriority {
    #[default]
    #[serde(rename = "H")]
    H,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "L")]
    L,
}

impl TaskPriority {
    fn as_str(self) -> &'static str {
        match self {
            TaskPriority::H => "H",
            TaskPriority::M => "M",
            TaskPriority::L => "L",
        }
    }
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub uuid: Uuid,
    pub status: TaskStatus,
    pub description: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub active: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectIndex>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagIndex>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotation>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriority>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<Uuid>,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub uda: HashMap<String, String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            status: TaskStatus::Pending,
            description: String::new(),
            active: false,
            modified: None,
            due: None,
            project: None,
            tags: Vec::new(),
            annotations: Vec::new(),
            priority: None,
            wait: None,
            depends: Vec::new(),
            uda: HashMap::new(),
        }
    }
}

fn to_string_with_radix(mut input: u32, radix: u32, out: &mut String) {
    let mut result = [0u8; 8];
    let mut used = 0;
    loop {
        let m = input % radix;
        input /= radix;

        result[used] = std::char::from_digit(m, radix).expect("radix must be > 2 and < 36") as u8;
        used += 1;

        if input == 0 {
            break;
        }
    }

    for c in result[..used].iter().rev() {
        out.push(char::from(*c));
    }
}

impl Task {
    #[must_use]
    pub fn entry(&self) -> Date {
        let timestamp = self
            .uuid
            .get_timestamp()
            .expect("uuid is v7 so this should not fail");
        let (secs, nsecs) = timestamp.to_unix();

        #[allow(clippy::cast_possible_wrap)]
        DateTime::from_timestamp(secs as i64, nsecs).expect("uuidv7 has a valid timestamp")
    }

    pub(crate) fn to_data(&self) -> String {
        let mut result = String::new();
        result.extend(self.uuid.to_base64_array().into_iter().map(char::from));
        escape(&self.description, &mut result);
        if self.active {
            result.push_str("\tA");
        }
        if let Some(modified) = self.modified {
            result.push_str("\tm");
            result.extend(modified.to_base64_array().into_iter().map(char::from));
        }
        if let Some(due) = self.due {
            result.push_str("\td");
            result.extend(due.to_base64_array().into_iter().map(char::from));
        }
        if let Some(wait) = self.wait {
            result.push_str("\tw");
            result.extend(wait.to_base64_array().into_iter().map(char::from));
        }
        if let Some(project) = self.project {
            result.push_str("\tp");
            to_string_with_radix(project, 36, &mut result);
        }
        if let Some(priority) = self.priority {
            result.push_str("\tr");
            result.push_str(priority.as_str());
        }
        if !self.tags.is_empty() {
            result.push_str("\tt");
            for (i, tag) in self.tags.iter().enumerate() {
                to_string_with_radix(*tag, 36, &mut result);
                if i + 1 != self.tags.len() {
                    result.push(',');
                }
            }
        }
        if !self.depends.is_empty() {
            result.push_str("\tn");
            for (i, depend) in self.depends.iter().enumerate() {
                result.extend(depend.to_base64_array().into_iter().map(char::from));
                if i + 1 != self.depends.len() {
                    result.push(',');
                }
            }
        }
        if !self.annotations.is_empty() {
            todo!("Annotation not implemented")
        }
        if !self.uda.is_empty() {
            todo!("UDA not implemented")
        }
        result
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn from_data(input: &str) -> Option<Task> {
        let uuid_bytes = input.get(0..22)?;
        let uuid = Uuid::from_base64_array(uuid_bytes.as_bytes().try_into().ok()?)?;
        let timestamp = uuid.get_timestamp()?;
        let (secs, nsecs) = timestamp.to_unix();
        let _entry = DateTime::from_timestamp(secs.try_into().ok()?, nsecs)?;

        let input = input.get(22..)?;
        let mut description_len = 0;
        for c in input.chars() {
            if c == '\t' {
                break;
            }

            description_len += c.len_utf8();
        }
        let description_non_escaped = input.get(..description_len)?;
        let mut input = input.get(description_len..)?;

        let mut iter = input.char_indices();
        let mut active = false;
        let mut modified = None;
        let mut due = None;
        let mut project = None;
        let mut priority = None;
        let mut wait = None;
        let mut depends = Vec::new();
        let mut tags = Vec::new();
        while let Some((_, '\t')) = iter.next() {
            let (position, type_) = iter.next()?;
            let start = position + 1;
            match type_ {
                'm' | 'd' | 'w' => {
                    let data = input.get(start..start + 11)?.as_bytes();
                    let date = Date::from_base64_array(data.try_into().ok()?)?;
                    match type_ {
                        'm' => modified = Some(date),
                        'd' => due = Some(date),
                        'w' => wait = Some(date),
                        _ => unreachable!(),
                    }
                    input = input.get(start + 11..)?;
                    iter = input.char_indices();
                }
                'n' => {
                    let uuid = Uuid::from_base64_array(
                        input.get(start..start + 22)?.as_bytes().try_into().ok()?,
                    )?;
                    depends.push(uuid);
                    input = input.get(start + 22..)?;
                    while let Some(',') = input.chars().nth(0) {
                        let start = 1;
                        let uuid = Uuid::from_base64_array(
                            input.get(start..start + 22)?.as_bytes().try_into().ok()?,
                        )?;
                        depends.push(uuid);
                        input = input.get(start + 22..)?;
                    }
                    iter = input.char_indices();
                }
                't' => {
                    input = input.get(start..)?;
                    iter = input.char_indices();
                    loop {
                        let mut value = 0;
                        let mut next = iter.next();
                        let mut position = 0;
                        while let Some((pos, digit)) =
                            next.and_then(|(pos, c)| Some((pos, c.to_digit(36)?)))
                        {
                            value = value * 36 + digit;
                            next = iter.next();
                            position = pos;
                        }

                        tags.push(value);

                        if let Some((position, ',')) = next {
                            input = input.get(position + 1..)?;
                            iter = input.char_indices();
                            continue;
                        }

                        input = input.get(position + 1..)?;
                        iter = input.char_indices();

                        break;
                    }
                }
                'p' => {
                    let mut value = 0;
                    let mut next = iter.next();
                    let mut position = 0;
                    while let Some((pos, digit)) =
                        next.and_then(|(pos, c)| Some((pos, c.to_digit(36)?)))
                    {
                        value = value * 36 + digit;
                        next = iter.next();
                        position = pos;
                    }

                    input = input.get(position + 1..)?;
                    iter = input.char_indices();
                    project = Some(value);
                }
                'r' => {
                    let value = match iter.next()?.1 {
                        'H' => TaskPriority::H,
                        'M' => TaskPriority::M,
                        'L' => TaskPriority::L,
                        _ => {
                            return None;
                        }
                    };
                    priority = Some(value);
                }
                'A' => {
                    active = true;
                }
                _ => return None,
            }
        }

        let mut description = String::new();
        unescape(description_non_escaped, &mut description);

        Some(Task {
            uuid,
            description,
            status: TaskStatus::Pending,
            active,
            modified,
            due,
            project,
            tags,
            annotations: Vec::default(),
            priority,
            wait,
            depends,
            uda: HashMap::default(),
        })
    }
}
