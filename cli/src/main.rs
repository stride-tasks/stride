use anyhow::{Context, bail};
use chrono::Utc;
use clap::Parser;
use cli::{CliArgs, Mode};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
};
use stride_api as api;
use stride_backend::{Backend, registry::Registry};
use stride_backend_git::{
    GitBackend, known_hosts::KnownHosts, method::SshHostAddHandler, ssh_key::SshKey,
};
use stride_backend_taskchampion::TaskchampionBackend;
use stride_core::{
    event::{HostEvent, PluginEvent},
    state::KnownPaths,
    task::{Task, TaskStatus},
};
use stride_crdt::{
    actor::ActorId,
    hlc::{Clock, SystemTimeProvider},
};
use stride_database::Database;
use stride_engine::EngineBuilder;
use stride_flutter_bridge::{
    api::settings::{ApplicationPaths, RepositorySpecification, Settings},
    method::RepositorySyncHandler,
};
use stride_logging::LogLevelGuard;
use stride_plugin_manager::{PluginManager, manifest::PluginAction};

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

impl api::Notifier for CliNotifier {
    fn notify(
        &self,
        context: Arc<dyn api::Context>,
        notification: api::Notification,
    ) -> api::Result<()> {
        match notification {
            api::Notification::Prompt(prompt) => {
                let summary = prompt.summary();
                let description = prompt.description();
                let mut confirm = inquire::Confirm::new(&summary).with_default(true);
                if let Some(description) = &description {
                    confirm = confirm.with_help_message(description);
                }

                let input = {
                    let _guard = LogLevelGuard::error();
                    confirm.prompt_skippable().map_err(Box::new)?
                };
                if input == Some(true) {
                    context.execute(&prompt.target(), prompt.inputs())?;
                }
            }
            api::Notification::RepositoryChanged(notification) => {
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
                    for api::FieldChange {
                        typ,
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
            }
        }
        Ok(())
    }
}

#[allow(clippy::too_many_lines)]
fn main() -> anyhow::Result<ExitCode> {
    let args = CliArgs::parse();
    let mode = args.mode.unwrap_or_else(|| Mode::Search {
        filter: Vec::default(),
    });

    let (support_dir, cache_dir) = match std::env::var("STRIDE_HOME") {
        Ok(path) => (PathBuf::from(&path), Path::new(&path).join("cache")),
        Err(std::env::VarError::NotPresent) => (
            choose_path_suffix(&dirs::data_dir().context("could not get data directory")?),
            choose_path_suffix(&dirs::cache_dir().context("could not get cache directory")?),
        ),
        Err(err) => return Err(err.into()),
    };

    let mut settings = Settings::load(ApplicationPaths {
        support_path: support_dir.to_string_lossy().to_string(),
        cache_path: cache_dir.to_string_lossy().to_string(),
        log_path: cache_dir
            .join("logs")
            .join("log.txt")
            .to_string_lossy()
            .to_string(),
    })?;

    let known_paths = KnownPaths::new(support_dir.clone(), cache_dir.clone());

    let current_repository = settings.current_repository.or_else(|| {
        settings
            .repositories
            .first()
            .map(|repository| repository.uuid)
    });

    let current_repository = if let Some(uuid) = current_repository {
        uuid
    } else {
        let repository = RepositorySpecification::default();
        let uuid = repository.uuid;
        std::fs::create_dir_all(support_dir.join("repository").join(uuid.to_string()))?;

        settings.repositories.push(repository);
        settings.current_repository = Some(uuid);
        Settings::save(settings.clone())?;
        uuid
    };

    let time_provider = Arc::new(SystemTimeProvider::default());
    let clock = Clock::new(time_provider);

    let plugins_path = support_dir.join("plugins");
    let mut plugin_manager = PluginManager::new(&plugins_path)?;
    plugin_manager.load()?;

    // let repository_path = known_paths.repository_path(current_repository);
    let database_filepath = known_paths.database_filepath(current_repository);

    let actor_id = ActorId::new(current_repository);
    let mut database = Database::open(&database_filepath, actor_id, clock)?;
    database.apply_migrations()?;

    let mut backend_registry = Registry::new();
    backend_registry.insert(GitBackend::handler());
    backend_registry.insert(TaskchampionBackend::handler());

    let notifier = Box::new(CliNotifier);
    let engine: Arc<dyn api::Context> = EngineBuilder::new()
        .notifier(notifier)
        .command("stride.repository.sync", RepositorySyncHandler)
        .command("stride.ssh.host.add", SshHostAddHandler)
        .build();

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
                .include("UUID", TaskField::Id)
                .include("Age", TaskField::Age)
                .include("Tags", TaskField::Tags)
                .include("Due", TaskField::Due)
                .include("P", TaskField::Priority)
                .include("Title", TaskField::Title)
                .include("Urgency", TaskField::Urgency)
                .build(&tasks);

            println!("{table}");
        }
        Mode::Add { content } => {
            let mut content = content.join(" ");

            if content == "-" {
                content = String::new();
                std::io::stdin().read_line(&mut content)?;
            }

            if content.trim().is_empty() {
                bail!("Missing arguments");
            }

            let task = Task::new(content.trim().to_string());

            let event = HostEvent::TaskCreate {
                task: Some(Box::new(task.clone())),
            };

            let mut transaction = database.transaction()?;
            transaction.insert_task(&task)?;
            transaction.commit()?;

            plugin_manager.emit_event(None, &event)?;
            while plugin_manager.process_host_event()? {}
            while let Some(action) = plugin_manager.process_plugin_event() {
                let (plugin_name, event) = match action {
                    PluginAction::Event { plugin_name, event } => (plugin_name, event),
                    PluginAction::Disable {
                        plugin_name,
                        reason,
                    } => {
                        log::error!("Disabling plugin {plugin_name}: {reason}");
                        plugin_manager.disable(&plugin_name, Some(reason))?;
                        continue;
                    }
                };

                match event {
                    PluginEvent::TaskCreate { task } => {
                        let mut transaction = database.transaction()?;
                        transaction.insert_task(&task)?;
                        transaction.commit()?;
                    }
                    PluginEvent::TaskModify { mut task } => {
                        let mut transaction = database.transaction()?;
                        transaction.update_task_with(task.id, |_| {
                            task.modified = Some(Utc::now());
                            Ok(task)
                        })?;
                        transaction.commit()?;
                    }
                    PluginEvent::TaskSync => {
                        backend_registry.sync_all(
                            current_repository,
                            &mut database,
                            &known_paths,
                            &engine,
                        )?;
                    }
                    PluginEvent::TaskQuery { query } => {
                        let tasks = database.task_query(&query)?;
                        plugin_manager
                            .emit_event(Some(&plugin_name), &HostEvent::TaskQuery { tasks })?;
                    }
                    PluginEvent::NetworkRequest { ty, host } => {
                        todo!("{:?}: {}", ty, host)
                    }
                }
            }
        }
        Mode::Done { id } => {
            let mut transaction = database.transaction()?;
            transaction.update_task_with(id, |mut task| {
                task.status = Some(TaskStatus::Done);
                task.modified = Some(Utc::now());
                Ok(task)
            })?;
            transaction.commit()?;
        }
        Mode::Undo { .. } => {
            todo!("undo")
        }
        Mode::Sync { backend: _name } => {
            let params = HashMap::from([(
                "id".into(),
                api::Value::String(current_repository.to_string().into()),
            )]);

            engine
                .clone()
                .execute("stride.repository.sync", api::Value::Map(params))?;
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
            let mut settings = Settings::get();
            let repository = settings
                .repositories
                .iter()
                .find(|repository| repository.uuid == uuid)
                .context("Repository with specified uuid was not found")?;
            println!("Switching to repository: {}", repository.name);
            settings.current_repository = Some(uuid);
            Settings::save(settings)?;
        }
        Mode::Backend { command } => {
            backend::handle_command(command.as_ref(), &backend_registry, &mut database)?;
        }
        Mode::Plugin { command } => match command {
            None => {
                for plugin in plugin_manager.list() {
                    println!("{}", plugin.manifest.name);
                }
            }
            Some(command) => match command {
                cli::PluginCommand::Import { filepath } => {
                    plugin_manager.import(&filepath)?;
                }
                cli::PluginCommand::Toggle { plugin_name } => {
                    plugin_manager.toggle(&plugin_name)?;
                }
            },
        },
        Mode::Ssh { command } => match command {
            SshCommand::Key { command: None } => {
                for key in SshKey::load_keys(&known_paths.ssh_keys)? {
                    println!("{} {}", key.id, key.public_key);
                }
            }
            SshCommand::Key {
                command: Some(SshKeyCommand::Generate),
            } => {
                let key = SshKey::generate(&known_paths.ssh_keys)?;
                println!("{} {}", key.id, key.public_key);
            }
            SshCommand::Key {
                command: Some(SshKeyCommand::Remove { id }),
            } => {
                SshKey::remove_key(&known_paths.ssh_keys, id)?;
            }
            SshCommand::KnownHosts { command: None } => {
                let hosts = KnownHosts::load()?;
                for host in hosts.hosts() {
                    println!("{} {}", host.hostname, host.key_type.name());
                }
            }
            SshCommand::KnownHosts {
                command: Some(SshKnownHostsCommand::Remove { hostname }),
            } => {
                let mut hosts = KnownHosts::load()?;
                hosts.remove_by_hostname(&hostname);
                KnownHosts::save(&hosts)?;
            }
        },
    }

    Ok(ExitCode::SUCCESS)
}
