use anyhow::{bail, Context};
use std::path::{Path, PathBuf};
use stride_flutter_bridge::{
    api::{
        filter::Filter,
        repository::TaskStorage,
        settings::{ApplicationPaths, Settings},
    },
    task::{Task, TaskStatus},
};

enum Mode {
    FilterList {
        filter: Filter,
    },
    Add {
        content: String,
    },
    Sync,
    Log {
        limit: Option<u32>,
        skip: Option<u32>,
    },
}

const APPLICATION_ID: &str = "org.stridetasks.stride";
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

#[allow(clippy::too_many_lines)]
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

    let mut repository = TaskStorage::new(&support_dir.join("repository").to_string_lossy());

    let mut args = std::env::args();
    let _program = args.next().expect("first argument should be program");
    let Some(action) = args.next() else {
        let tasks = repository.tasks()?;
        print_tasks(&tasks);
        return Ok(());
    };
    let mode = match action.as_str() {
        "add" => Mode::Add {
            content: args.collect::<Vec<_>>().join(" "),
        },
        "sync" => Mode::Sync,
        "log" => {
            let limit = args
                .next()
                .map(|value| {
                    if value == "-" {
                        u32::MAX.to_string()
                    } else {
                        value
                    }
                })
                .map(|s| s.parse::<u32>())
                .transpose()
                .context("invalid limit value")?;
            let skip = args
                .next()
                .map(|s| s.parse::<u32>())
                .transpose()
                .context("invalid limit value")?;
            Mode::Log { limit, skip }
        }
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
        Mode::Add { mut content } => {
            if content == "-" {
                content = String::new();
                std::io::stdin().read_line(&mut content)?;
            }

            if content.trim().is_empty() {
                bail!("Missing arguments");
            }

            let task = Task::new(content.trim().to_string());
            repository.add(task)?;
        }
        Mode::Sync => {
            repository.sync()?;
        }
        Mode::Log { limit, skip } => {
            /// This is to prevent going though the git history in one go which allocates uses a of memory.
            // TODO: Maybe figure out what is the best value.
            const CHUNK_COUNT: u32 = 10000;

            let mut last_oid = None;
            let mut count: u32 = 0;
            if let Some(skip) = skip {
                let Some(commits) = repository.log(last_oid, Some(skip))? else {
                    return Ok(());
                };
                for commit in commits {
                    last_oid = commit.parent;
                    count += 1;
                }

                // If we skipped though all the commits, when we can just stop here.
                if last_oid.is_none() {
                    return Ok(());
                }
            }

            let limit = count.saturating_add(limit.unwrap_or(u32::MAX));

            'outer: loop {
                let Some(commits) = repository.log(last_oid, Some(CHUNK_COUNT))? else {
                    return Ok(());
                };
                for commit in commits {
                    if count >= limit {
                        break 'outer;
                    }

                    // TODO: Make history formating configurable.
                    println!(
                        "{:4}. {} {} {} {}",
                        count + 1,
                        commit.oid,
                        commit.author,
                        commit.email,
                        commit.message.trim()
                    );

                    last_oid = commit.parent;
                    count += 1;
                }

                if last_oid.is_none() {
                    break;
                }
            }
        }
    }

    Ok(())
}
