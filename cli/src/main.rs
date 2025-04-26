use anyhow::{Context, bail};
use clap::Parser;
use cli::{CliArgs, Mode};
use std::path::{Path, PathBuf};
use stride_backend::{
    Backend,
    taskchampion::{TaskchampionBackend, TaskchampionConfig},
};
use stride_core::{
    event::{HostEvent, PluginEvent},
    task::{Task, TaskStatus},
};
use stride_flutter_bridge::api::{
    filter::Filter,
    repository::Repository,
    settings::{ApplicationPaths, RepositorySpecification, Settings},
};
use stride_plugin_manager::{PluginManager, manifest::PluginAction};
use url::Url;
use uuid::Uuid;

pub mod cli;

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

fn print_tasks(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        let mut active_char = ' ';
        if task.active {
            active_char = '>';
        }
        let mut tags = String::new();
        if !task.tags.is_empty() {
            tags.push('(');
            for (i, tag) in task.tags.iter().enumerate() {
                tags.push_str(tag);

                if i + 1 != task.tags.len() {
                    tags.push_str(", ");
                }
            }
            tags.push(')');
        }
        println!("{active_char}{tags}{i:4}: {}", task.title);
    }
}

#[allow(clippy::too_many_lines)]
fn main() -> anyhow::Result<()> {
    // TODO(@bpeetz): Re-add the functionality of running `stride` without
    // args or not one of the defined subcommands to search  <2024-10-24>
    // else {
    //     let tasks = repository.tasks()?;
    //     print_tasks(&tasks);
    //     return Ok(());
    // };
    let args = CliArgs::parse();

    let cache_dir =
        choose_path_suffix(&dirs::cache_dir().context("could not get cache directory")?);
    let document_dir =
        choose_path_suffix(&dirs::document_dir().context("could not get document directory")?);
    let support_dir =
        choose_path_suffix(&dirs::data_dir().context("could not get data directory")?);

    let mut settings = Settings::load(ApplicationPaths {
        support_path: support_dir.to_string_lossy().to_string(),
        document_path: document_dir.to_string_lossy().to_string(),
        cache_path: cache_dir.to_string_lossy().to_string(),
        log_path: cache_dir
            .join("logs")
            .join("log.txt")
            .to_string_lossy()
            .to_string(),
    })?;

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
        settings.repositories.push(repository);
        settings.current_repository = Some(uuid);
        Settings::save(settings.clone())?;
        uuid
    };

    let plugins_path = support_dir.join("plugins");
    let mut plugin_manager = PluginManager::new(&plugins_path)?;
    plugin_manager.load()?;

    let mut repository = Repository::open(current_repository)?;

    // let repository: Rc<RefCell<dyn StrideRepository>> = match args.repository {
    //     RepositoryType::Git => Rc::new(RefCell::new(TaskStorage::new(
    //         current_repository,
    //         &support_dir.join("repository").to_string_lossy(),
    //         &settings,
    //     )?)),
    //     RepositoryType::TaskChampion => {
    //         let data_dir =
    //             choose_path_suffix(&dirs::data_dir().context("could not get data directory")?);
    //         let config_dir =
    //             choose_path_suffix(&dirs::config_dir().context("could not get config directory")?);

    //         let db_path = data_dir.join("taskchampion");
    //         let config_path = config_dir.join("config.toml");

    //         let (url, client_id, encryption_secret) = {
    //             #[derive(Deserialize)]
    //             struct Config {
    //                 sync: Sync,
    //             }
    //             #[derive(Deserialize)]
    //             struct Sync {
    //                 server: Server,
    //                 encryption_secret: String,
    //             }
    //             #[derive(Deserialize)]
    //             struct Server {
    //                 origin: Url,
    //                 client_id: Uuid,
    //             }

    //             let file = fs::read_to_string(&config_path).with_context(|| {
    //                 format!("Failed to read config file at: '{}'", config_path.display())
    //             })?;

    //             let config: Config = toml::from_str(&file).with_context(|| {
    //                 format!(
    //                     "Failed to parse config file at: '{}' as toml config.",
    //                     config_path.display()
    //                 )
    //             })?;

    //             (
    //                 config.sync.server.origin,
    //                 config.sync.server.client_id,
    //                 config.sync.encryption_secret,
    //             )
    //         };

    //         let server_config = taskchampion::ServerConfig::Remote {
    //             url: url.to_string(),
    //             client_id,
    //             encryption_secret: encryption_secret.as_bytes().to_vec(),
    //         };
    //         let constraint_environment = false;

    //         Rc::new(RefCell::new(
    //             Replica::new(&db_path, server_config, constraint_environment).with_context(
    //                 || {
    //                     format!(
    //                         "Failed to initialize taskchampion storage at: {}",
    //                         db_path.display()
    //                     )
    //                 },
    //             )?,
    //         ))
    //     }
    // };

    match args.mode {
        Mode::Search { filter } => {
            let filter = Filter {
                search: filter.join(" "),
                status: [TaskStatus::Pending].into(),
                ..Default::default()
            };
            let tasks = repository.all_tasks(filter)?;
            print_tasks(&tasks);
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
            repository.insert_task(&task)?;

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
                        repository.insert_task(&task)?;
                    }
                    PluginEvent::TaskModify { task } => {
                        repository.update_task(&task)?;
                    }
                    PluginEvent::TaskSync => {
                        repository.sync()?;
                    }
                    PluginEvent::TaskQuery { query } => {
                        let tasks = repository.task_query(&query)?;
                        plugin_manager
                            .emit_event(Some(&plugin_name), &HostEvent::TaskQuery { tasks })?;
                    }
                    PluginEvent::NetworkRequest { ty, host } => {
                        todo!("{:?}: {}", ty, host)
                    }
                }
            }
        }
        Mode::Sync { backend } => match backend {
            cli::Backend::Git => repository.sync()?,
            cli::Backend::TaskChampion => {
                let config_dir = choose_path_suffix(
                    &dirs::config_dir().context("could not get config directory")?,
                );
                let config_path = config_dir.join("config.toml");

                let (url, client_id, encryption_secret) = {
                    #[derive(serde::Deserialize)]
                    struct Config {
                        sync: Sync,
                    }
                    #[derive(serde::Deserialize)]
                    struct Sync {
                        server: Server,
                        encryption_secret: String,
                    }
                    #[derive(serde::Deserialize)]
                    struct Server {
                        origin: Url,
                        client_id: Uuid,
                    }

                    let file = std::fs::read_to_string(&config_path).with_context(|| {
                        format!("Failed to read config file at: '{}'", config_path.display())
                    })?;

                    let config: Config = toml::from_str(&file).with_context(|| {
                        format!(
                            "Failed to parse config file at: '{}' as toml config.",
                            config_path.display()
                        )
                    })?;

                    (
                        config.sync.server.origin,
                        config.sync.server.client_id,
                        config.sync.encryption_secret,
                    )
                };

                let constraint_environment = false;
                let config = TaskchampionConfig {
                    root_path: repository.root_path.join("backend").join("taskchampion"),
                    url: url.as_str().to_string(),
                    client_id,
                    encryption_secret: encryption_secret.as_bytes().to_vec(),
                    constraint_environment,
                };
                let mut backend = TaskchampionBackend::new(config)?;
                backend.sync(repository.db.get_mut().unwrap())?;
            }
        },
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
        Mode::Plugin { command } => {
            match command {
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
            };
        }
    }

    Ok(())
}
