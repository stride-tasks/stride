use anyhow::{Context as AnyhowContext, bail};
use chrono::{NaiveDate, NaiveTime, Utc};
use clap::Parser;
use cli::{CliArgs, Mode};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
};
use stride_api::{
    self as api, RepositoryAddMethod, RepositoryListMethod, SettingGetMethod, SettingSetMethod,
    Task, TaskPriority, TaskStatus,
};
use stride_backend_git::{
    GitBackend, SshHostListHandler, SshHostRemoveHandler, SshKeyAddHandler, SshKeyGenerateHandler,
    SshKeyListHandler, SshKeyRemoveHandler, SshKeyUpdateHandler, known_hosts::KnownHosts,
    method::SshHostAddHandler, ssh_key::SshKey,
};
use stride_crdt::{
    actor::ActorId,
    hlc::{Clock, SystemTimeProvider},
};
use stride_database::Database;
use stride_engine::{Backend, Engine, KnownPaths};
use stride_logging::LogLevelGuard;
use uuid::Uuid;

use crate::{
    cli::{SshCommand, SshKeyCommand, SshKnownHostsCommand},
    display::{TaskField, TaskItem, TaskTable},
};

pub mod backend;
pub mod cli;
pub mod display;

const APPLICATION_ID: &str = "org.stridetasks.stride";
const APPLICATION_NAME: &str = "stride";

/// Choose correct path prefix based on availability.
///
/// Currently it seems that dart's `path_provider` package does not seem to be consistent
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

#[derive(Debug, Clone, Copy)]
struct CliNotifier;

impl stride_engine::Notifier for CliNotifier {
    fn notify(&self, context: &Arc<Engine>, notification: Box<dyn stride_engine::Notification>) {
        let notification_any: &dyn std::any::Any = &*notification;
        if let Some(prompt) = notification_any.downcast_ref::<api::UserPromptNotification>() {
            let mut confirm = inquire::Confirm::new(&prompt.summary).with_default(true);
            if let Some(description) = &prompt.description {
                confirm = confirm.with_help_message(description);
            }

            let input = {
                let _guard = LogLevelGuard::error();
                confirm.prompt_skippable()
            };
            match input {
                Err(err) => {
                    log::error!("Failed to prompt user: {err}");
                }
                Ok(Some(true)) => {
                    let result =
                        context.execute_erased(&prompt.target.method, prompt.target.params.clone());
                    if let Err(err) = result {
                        log::error!("Failed to execute prompt target: {err}");
                    }
                }
                Ok(None | Some(false)) => {
                    // User declined the prompt, do nothing.
                    // User skipped the prompt, do nothing.
                }
            }
        } else if let Some(notification) =
            notification_any.downcast_ref::<api::RepositoryChangedNotification>()
        {
            for (
                i,
                api::TaskChange {
                    title,
                    task_id,
                    fields,
                },
            ) in notification.changes.iter().enumerate()
            {
                println!("Task ({task_id}): '{title}':");
                for api::TaskFieldChange {
                    r#type: typ,
                    previous,
                    current,
                } in fields
                {
                    println!(
                        "  -- {typ}: {} => {}",
                        previous.as_deref().unwrap_or("none"),
                        current.as_deref().unwrap_or("none")
                    );
                }

                if i + 1 != notification.changes.len() {
                    println!();
                }
            }
        } else {
            log::info!(
                "unknown notification: {};{}",
                notification.name(),
                notification.to_value()
            );
        }
    }
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct TaskModifications {
    title_parts: Vec<String>,
    due: Option<Option<chrono::DateTime<Utc>>>,
    priority: Option<Option<TaskPriority>>,
    project: Option<Option<String>>,
    tag_operations: Vec<(bool, String)>,
}

fn parse_task_modifications(modifiers: &[cli::TaskModifier]) -> anyhow::Result<TaskModifications> {
    let mut modifications = TaskModifications::default();

    for modifier in modifiers {
        match modifier {
            cli::TaskModifier::Text(text) => {
                modifications.title_parts.push(text.clone());
            }
            cli::TaskModifier::Due(value) => {
                modifications.due = if value.trim().is_empty() {
                    Some(None)
                } else if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                    Some(Some(
                        date.and_time(NaiveTime::default())
                            .and_local_timezone(chrono::Local)
                            .latest()
                            .context("")?
                            .to_utc(),
                    ))
                } else {
                    bail!("unable to parse due date: {value}");
                };
            }
            cli::TaskModifier::TagAdd(tag) => {
                modifications.tag_operations.push((false, tag.clone()));
            }
            cli::TaskModifier::TagRemove(tag) => {
                modifications.tag_operations.push((true, tag.clone()));
            }
            cli::TaskModifier::Priority(value) => {
                modifications.priority = match value.as_str() {
                    "" => Some(None),
                    "L" => Some(Some(TaskPriority::Low)),
                    "M" => Some(Some(TaskPriority::Medium)),
                    "H" => Some(Some(TaskPriority::High)),
                    _ => bail!("unknown priority: {value}"),
                };
            }
            cli::TaskModifier::Project(value) => {
                modifications.project = if value.trim().is_empty() {
                    Some(None)
                } else {
                    Some(Some(value.clone()))
                };
            }
        }
    }

    Ok(modifications)
}

fn task_title_from_modifications(modifications: &TaskModifications) -> Option<String> {
    let title = modifications.title_parts.join(" ");
    if title.trim().is_empty() {
        None
    } else {
        Some(title.trim().to_string())
    }
}

fn apply_modifications(task: &mut Task, modifications: &TaskModifications) -> bool {
    let mut modified = false;

    if let Some(new_due) = modifications.due {
        task.due = new_due;
        modified = true;
    }

    if let Some(new_priority) = modifications.priority {
        task.priority = new_priority;
        modified = true;
    }

    if let Some(new_project) = &modifications.project {
        task.project = new_project.as_ref().map(|project| project.clone().into());
        modified = true;
    }

    for (remove, tag) in &modifications.tag_operations {
        if *remove {
            if let Some(index) = task.tags.iter().position(|value| value.as_ref() == tag) {
                task.tags.remove(index);
                modified = true;
            }
        } else if !task.tags.iter().any(|value| value.as_ref() == tag) {
            task.tags.push(tag.clone().into());
            modified = true;
        }
    }

    modified
}

#[allow(clippy::too_many_lines)]
fn main() -> anyhow::Result<ExitCode> {
    let args = CliArgs::parse();
    let mode = args.mode.unwrap_or_else(|| Mode::Search {
        filter: Vec::default(),
    });

    stride_logging::print_logs(args.print_logs);

    let (support_dir, cache_dir) = match std::env::var("STRIDE_HOME") {
        Ok(path) => (PathBuf::from(&path), Path::new(&path).join("cache")),
        Err(std::env::VarError::NotPresent) => (
            choose_path_suffix(&dirs::data_dir().context("could not get data directory")?),
            choose_path_suffix(&dirs::cache_dir().context("could not get cache directory")?),
        ),
        Err(err) => return Err(err.into()),
    };

    std::fs::create_dir_all(support_dir.join(".ssh"))?;
    let known_paths = KnownPaths::new(support_dir.clone(), cache_dir.clone());
    stride_logging::init(&known_paths.logs);

    let notifier = Box::new(CliNotifier);
    let engine = Engine::builder(known_paths)?
        .notifier(notifier)
        .insert_default_methods()
        .backend(GitBackend::handler())
        .command("ssh.host.add", SshHostAddHandler)
        .command("ssh.host.list", SshHostListHandler)
        .command("ssh.host.remove", SshHostRemoveHandler)
        .command("ssh.key.list", SshKeyListHandler)
        .command("ssh.key.generate", SshKeyGenerateHandler)
        .command("ssh.key.add", SshKeyAddHandler)
        .command("ssh.key.update", SshKeyUpdateHandler)
        .command("ssh.key.remove", SshKeyRemoveHandler)
        .build();

    let mut settings = engine.execute(SettingGetMethod {})?.setting;

    let current_repository = match settings.current_repository {
        Some(uuid) => Some(uuid),
        None => engine
            .execute(RepositoryListMethod {})?
            .repositories
            .first()
            .map(|repository| repository.id),
    };

    let current_repository = if let Some(uuid) = current_repository {
        uuid
    } else {
        let result = engine.execute(RepositoryAddMethod { name: None })?;
        settings.current_repository = Some(result.repository_id);

        engine.execute(SettingSetMethod {
            setting: settings.clone(),
        })?;

        result.repository_id
    };

    let time_provider = Arc::new(SystemTimeProvider::default());
    let clock = Clock::new(time_provider);

    let database_filepath = engine.known_paths().database_filepath(current_repository);

    let actor_id = ActorId::new(current_repository);
    let mut database = Database::open(&database_filepath, actor_id, clock)?;
    database.apply_migrations()?;

    match mode {
        Mode::Search { filter } => {
            let search = filter.join(" ").to_lowercase();
            let status = [TaskStatus::Pending].into();

            let mut tasks = database
                .tasks_by_status(&status)?
                .into_iter()
                .enumerate()
                .map(|(index, task)| TaskItem {
                    index: index + 1,
                    task,
                })
                .collect::<Vec<_>>();

            #[allow(clippy::cast_possible_truncation)]
            tasks.sort_by_cached_key(|item| -(item.task.urgency() * 100.0) as i64);

            tasks.retain(|item| {
                item.task
                    .title
                    .as_ref()
                    .is_some_and(|title| title.to_lowercase().contains(&search))
            });

            let table = TaskTable::new()
                .include("ID", TaskField::Index)
                .include("Age", TaskField::Age)
                .include("Project", TaskField::Project)
                .include("Tags", TaskField::Tags)
                .include("Due", TaskField::Due)
                .include("P", TaskField::Priority)
                .include("Title", TaskField::Title)
                .include("Urgency", TaskField::Urgency)
                .build(&tasks);

            println!("{table}");
        }
        Mode::Add { modifiers } => {
            if modifiers.is_empty() {
                bail!("Missing arguments");
            }

            let modifications = parse_task_modifications(&modifiers)?;
            let title = task_title_from_modifications(&modifications);
            let title = if let Some(title) = title {
                title
            } else {
                let mut content = String::new();
                std::io::stdin().read_line(&mut content)?;
                content.split_whitespace().collect::<Vec<_>>().join(" ")
            };

            let mut task = Task::new(title.into());
            apply_modifications(&mut task, &modifications);

            engine.execute(api::RepositoryTaskAddMethod {
                repository_id: current_repository,
                task,
            })?;
        }
        Mode::Done { id } => {
            let id = if let Ok(id) = id.parse::<Uuid>() {
                id
            } else if let Ok(index) = id.parse::<usize>() {
                if index == 0 {
                    bail!("zero is not a valid task index");
                }
                let status = [TaskStatus::Pending].into();
                let tasks = database.tasks_by_status(&status)?;

                let task = tasks
                    .get(index - 1)
                    .with_context(|| format!("unable to find task with index: {index}"))?;

                task.id
            } else {
                bail!("invalid task identifier expected index or UUID");
            };

            let mut transaction = database.transaction()?;
            transaction.update_task_with(id, |mut task| {
                task.status = Some(TaskStatus::Done);
                task.modified = Some(Utc::now());
                Ok(task)
            })?;
            transaction.commit()?;
        }
        Mode::Modify { modifiers } => {
            let id = args
                .task_id
                .context("expected task id when calling modify")?;
            let id = if let Ok(id) = id.parse::<Uuid>() {
                id
            } else if let Ok(index) = id.parse::<usize>() {
                if index == 0 {
                    bail!("zero is not a valid task index");
                }
                let status = [TaskStatus::Pending].into();
                let tasks = database.tasks_by_status(&status)?;

                let task = tasks
                    .get(index - 1)
                    .with_context(|| format!("unable to find task with index: {index}"))?;

                task.id
            } else {
                bail!("invalid task identifier expected index or UUID");
            };

            let modifications = parse_task_modifications(&modifiers)?;
            let new_title = task_title_from_modifications(&modifications);

            let mut transaction = database.transaction()?;
            transaction.update_task_with(id, |mut task| {
                let mut modified = apply_modifications(&mut task, &modifications);
                if let Some(new_title) = new_title {
                    task.title = Some(new_title.into());
                    modified = true;
                }
                if modified {
                    task.modified = Some(Utc::now());
                }
                Ok(task)
            })?;
            transaction.commit()?;
        }
        Mode::Undo { .. } => {
            todo!("undo")
        }
        Mode::Sync { backend: _name } => {
            let _result = engine.clone().execute(api::RepositorySyncMethod {
                id: current_repository,
                backends: None,
            })?;
        }
        Mode::Log { .. } => {
            /// This is to prevent going though the git history in one go which allocates uses a of memory.
            // TODO: Maybe figure out what is the best value.
            const _CHUNK_COUNT: u32 = 10000;

            todo!();
            // let mut last_oid = None;
            // let mut count: u32 = 0;
            // if let Some(skip) = skip {
            //     let Some(commits) = repository.log(last_oid, Some(skip))? else {
            //         return Ok(());
            //     };
            //     for commit in commits {
            //         last_oid = commit.parent;
            //         count += 1;
            //     }
            //
            //     // If we skipped though all the commits, when we can just stop here.
            //     if last_oid.is_none() {
            //         return Ok(());
            //     }
            // }
            //
            // let limit = count.saturating_add(limit);
            //
            // 'outer: loop {
            //     let Some(commits) = repository.log(last_oid, Some(CHUNK_COUNT))? else {
            //         return Ok(());
            //     };
            //     for commit in commits {
            //         if count >= limit {
            //             break 'outer;
            //         }
            //
            //         // TODO: Make history formatting configurable.
            //         println!(
            //             "{:4}. {} {} {} {}",
            //             count + 1,
            //             commit.oid,
            //             commit.author,
            //             commit.email,
            //             commit.message.trim()
            //         );
            //
            //         last_oid = commit.parent;
            //         count += 1;
            //     }
            //
            //     if last_oid.is_none() {
            //         break;
            //     }
            // }
        }
        Mode::Export { filepath: _ } => {
            // let contents = repository.borrow_mut().export()?;
            // if let Some(filepath) = filepath {
            //     fs::write(filepath, contents)?;
            // } else {
            //     println!("{contents}");
            // }

            todo!()
        }
        Mode::Import { filepath: _ } => {
            // let contents = if let Some(filepath) = filepath {
            //     fs::read_to_string(filepath)?
            // } else {
            //     let mut contents = String::new();
            //     std::io::stdin().read_to_string(&mut contents)?;
            //     contents
            // };
            // repository.borrow_mut().import(&contents)?;
            todo!()
        }
        Mode::Repository { uuid } => {
            let repositories = engine.execute(RepositoryListMethod {})?.repositories;

            let mut repository = None;
            for candidate in repositories {
                if candidate.id == uuid {
                    repository = Some(candidate);
                    break;
                }
            }
            let repository = repository.context("Repository with specified uuid was not found")?;
            let name = repository
                .name
                .unwrap_or_else(|| "unnamed".to_string().into_boxed_str());
            println!("Switching to repository: {name}");

            settings.current_repository = Some(uuid);
            engine.execute(SettingSetMethod {
                setting: settings.clone(),
            })?;
        }
        Mode::Backend { command } => {
            backend::handle_command(command.as_ref(), engine.backends(), &mut database)?;
        }
        Mode::Plugin { command } => match command {
            None => {
                for plugin in engine.lock_plugin_manager().list() {
                    println!("{}", plugin.instance.manifest.name);
                }
            }
            Some(command) => match command {
                cli::PluginCommand::Import { filepath } => {
                    engine.lock_plugin_manager().import(&filepath)?;
                }
                cli::PluginCommand::Toggle { plugin_name } => {
                    engine.lock_plugin_manager().toggle(&plugin_name)?;
                }
            },
        },
        Mode::Ssh { command } => match command {
            SshCommand::Key { command: None } => {
                for key in SshKey::load_keys(&engine.known_paths().ssh_keys)? {
                    println!("{} {}", key.id, key.public_key);
                }
            }
            SshCommand::Key {
                command: Some(SshKeyCommand::Generate),
            } => {
                let key = SshKey::generate(&engine.known_paths().ssh_keys)?;
                println!("{} {}", key.id, key.public_key);
            }
            SshCommand::Key {
                command: Some(SshKeyCommand::Remove { id }),
            } => {
                SshKey::remove_key(&engine.known_paths().ssh_keys, id)?;
            }
            SshCommand::KnownHosts { command: None } => {
                let known_hosts_filepath = &engine.known_paths().ssh_known_hosts_filepath();
                let hosts = KnownHosts::read_file(known_hosts_filepath)?;
                for host in hosts.hosts() {
                    println!("{} {}", host.hostname, host.key_type.name());
                }
            }
            SshCommand::KnownHosts {
                command: Some(SshKnownHostsCommand::Remove { hostname }),
            } => {
                let known_hosts_filepath = &engine.known_paths().ssh_known_hosts_filepath();
                let mut hosts = KnownHosts::read_file(known_hosts_filepath)?;
                hosts.remove_by_hostname(&hostname);
                hosts.write_file(known_hosts_filepath)?;
            }
        },
        Mode::Api { method, params } => {
            let Some(method) = method else {
                let descriptions = engine.clone().method_descriptions();
                for description in descriptions {
                    println!("- {}", description.name);
                }
                return Ok(ExitCode::SUCCESS);
            };
            let params = if let Some(params) = params {
                serde_json::from_str::<api::Value>(&params)?
            } else {
                api::Value::Map(api::Map::new())
            };
            let result = engine.clone().execute_erased(&method, params)?;
            println!("{result}");
        }
    }

    engine.wait_for_tasks();

    Ok(ExitCode::SUCCESS)
}
