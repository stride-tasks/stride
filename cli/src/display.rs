use chrono::{DateTime, Datelike, Utc};
use comfy_table::{Cell, Color, ContentArrangement, Row, Table, presets::NOTHING};
use stride_core::task::Task;

#[derive(Debug, Clone)]
pub struct TaskItem {
    pub index: usize,
    pub task: Task,
}

pub type CustomTaskField = Box<dyn Fn(&TaskItem) -> Option<Box<str>>>;

#[allow(missing_debug_implementations)]
pub enum TaskField {
    Index,
    Id,
    Age,
    Tags,
    Due,
    Priority,
    Title,
    Urgency,
    Custom { f: CustomTaskField },
}

impl TaskField {
    fn get(&self, item: &TaskItem) -> Option<Box<str>> {
        match self {
            Self::Index => Some(item.index.to_string().into()),
            Self::Id => Some(item.task.id.to_string().into()),
            Self::Age => {
                let now = Utc::now();
                item.task
                    .entry
                    .or(item.task.modified)
                    .map(|date| format_date_difference(date, now))
            }
            Self::Tags if item.task.tags.is_empty() => None,
            Self::Tags => Some(item.task.tags.join(" ").into()),
            Self::Due => item.task.due.map(|due| {
                due.with_timezone(&chrono::Local)
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
                    .into_boxed_str()
            }),
            Self::Priority => item.task.priority.map(|priority| priority.as_str().into()),
            Self::Title => item.task.title.clone().map(Into::into),
            Self::Urgency => Some(format!("{:.2}", item.task.urgency()).into()),
            Self::Custom { f } => f(item),
        }
    }
}

#[derive(Default)]
#[allow(missing_debug_implementations)]
pub struct TaskTable {
    headers: Vec<(Box<str>, TaskField)>,
}

impl TaskTable {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn include(mut self, header: &str, item: TaskField) -> Self {
        self.headers.push((header.into(), item));
        self
    }

    #[must_use]
    pub fn build(&self, tasks: &[TaskItem]) -> Table {
        let mut rows = Vec::with_capacity(tasks.len());
        let mut has_values = vec![false; self.headers.len()];
        for task in tasks {
            let mut row = Vec::with_capacity(self.headers.len());
            for ((_, item), has_value) in self.headers.iter().zip(&mut has_values) {
                let value = item.get(task);
                *has_value |= value.is_some();
                row.push(value);
            }
            rows.push(row);
        }

        let mut table = Table::new();
        table
            .load_style(NOTHING)
            .set_content_arrangement(ContentArrangement::Dynamic);

        let mut header_row = Row::new();
        for ((header, _), has_values) in self.headers.iter().zip(&has_values) {
            if *has_values {
                let cell = Cell::new(header).add_attribute(comfy_table::Attribute::Underlined);
                header_row.add_cell(cell);
            }
        }

        table.set_header(header_row);
        for (index, data) in rows.into_iter().enumerate() {
            let mut row = Row::new();
            for (data_cell, has_value) in data.into_iter().zip(&has_values) {
                if *has_value {
                    let mut cell = Cell::new(data_cell.as_deref().unwrap_or(""));

                    // Apply a gray background to every second (even) row.
                    if index % 2 == 1 {
                        cell = cell.bg(Color::DarkGrey);
                    }

                    row.add_cell(cell);
                }
            }

            table.add_row(row);
        }
        table
    }
}

fn format_date_difference(start: DateTime<Utc>, end: DateTime<Utc>) -> Box<str> {
    // Ensure start is the earlier date
    if start > end {
        return format_date_difference(end, start);
    }

    let duration = end - start;
    let total_seconds = duration.num_seconds();

    // 1. Seconds (if less than 60 seconds)
    if total_seconds < 60 {
        return format!("{total_seconds}s").into();
    }

    // 2. Minutes (if less than 60 minutes)
    let total_minutes = duration.num_minutes();
    if total_minutes < 60 {
        return format!("{total_minutes}min").into();
    }

    // 3. Hours (if less than 24 hours)
    let total_hours = duration.num_hours();
    if total_hours < 24 {
        return format!("{total_hours}h").into();
    }

    // 4. Days (if less than 30 days)
    let total_days = duration.num_days();
    if total_days < 30 {
        return format!("{total_days}d").into();
    }

    // Calculate rough month difference
    // NOTE: i32 cast is fine since the range is from 1-12
    #[allow(clippy::cast_possible_wrap)]
    let mut months = (end.year() - start.year()) * 12 + (end.month() as i32 - start.month() as i32);

    // Adjust if the end day of the month is less than the start day
    if end.day() < start.day() {
        months -= 1;
    }

    // 2. Return Years (with one decimal place) if 12 months or more
    if months >= 12 {
        let years = f64::from(months) / 12.0;
        // Format to 1 decimal place, strip trailing .0 if you want exact integers
        let formatted_years = format!("{years:.1}");
        return format!("{}y", formatted_years.trim_end_matches(".0")).into();
    }

    // 3. Return Months if between 30 days and 1 year
    format!("{months}mo").into()
}
