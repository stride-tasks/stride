use std::path::{Path, PathBuf};

use anyhow::Context;
use stride_flutter_bridge::{
    api::{
        filter::Filter,
        repository::TaskStorage,
        settings::{ApplicationPaths, Settings},
    },
    task::{Task, TaskStatus},
};

enum Mode {
    FilterList { filter: Filter },
    Add { content: String },
    Sync,
}

const APPLICATION_ID: &str = "com.example.stride";
const APPLICATION_NAME: &str = "stride";

/// Choose correct path prefix based on availability.
///
/// Currently it seems that dart's `path_provider` package does not seem to be consitent
/// when creating the paths. Sometimes the path is the application id and other times it's
/// the application name.
fn choose_path_suffix(path: &Path) -> PathBuf {
    if path.join(APPLICATION_ID).exists() {
        return path.join(APPLICATION_ID);
    }

    if path.join(APPLICATION_NAME).exists() {
        return path.join(APPLICATION_NAME);
    }

    path.join(APPLICATION_ID)
}

fn print_tasks(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        let mut active_char = ' ';
        if task.active {
            active_char = '>';
        }
        println!("{active_char}{i:4}: {}", task.description);
    }
}

fn main() -> anyhow::Result<()> {
    let cache_dir =
        choose_path_suffix(&dirs::cache_dir().context("could not get cache directory")?);
    let document_dir =
        choose_path_suffix(&dirs::document_dir().context("could not get document directory")?);
    let support_dir =
        choose_path_suffix(&dirs::data_dir().context("could not get data directory")?);

    Settings::load(ApplicationPaths {
        support_path: support_dir.to_string_lossy().to_string(),
        document_path: document_dir.to_string_lossy().to_string(),
        cache_path: cache_dir.to_string_lossy().to_string(),
        log_path: cache_dir
            .join("logs")
            .join("log.txt")
            .to_string_lossy()
            .to_string(),
    })?;

    let mut repository = TaskStorage::new(&support_dir.to_string_lossy());

    let mut args = std::env::args();
    let _program = args.next().expect("first argument should be program");
    let Some(action) = args.next() else {
        let tasks = repository.tasks()?;
        print_tasks(&tasks);
        return Ok(());
    };
    let mode = match action.as_str() {
        "add" => {
            let task = args.collect::<Vec<_>>().join(" ");
            if task.is_empty() {
                Mode::FilterList {
                    filter: Filter {
                        search: action,
                        status: [TaskStatus::Pending].into(),
                        ..Default::default()
                    },
                }
            } else {
                Mode::Add { content: task }
            }
        }
        "sync" => Mode::Sync,
        _ => Mode::FilterList {
            filter: Filter {
                search: std::iter::once(action)
                    .chain(args)
                    .collect::<Vec<_>>()
                    .join(" "),
                status: [TaskStatus::Pending].into(),
                ..Default::default()
            },
        },
    };

    match mode {
        Mode::FilterList { filter } => {
            let tasks = repository.tasks_with_filter(&filter)?;
            print_tasks(&tasks);
        }
        Mode::Add { content } => {
            let task = Task::new(content);
            repository.add(task)?;
        }
        Mode::Sync => {
            repository.sync()?;
        }
    }

    Ok(())
}
