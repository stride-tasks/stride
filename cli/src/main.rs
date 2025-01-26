use anyhow::{bail, Context};
use clap::Parser;
use cli::{CliArgs, Mode, RepositoryType};
use serde::Deserialize;
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use stride_flutter_bridge::{
    api::{
        filter::Filter,
        plugin::PluginManager,
        repository::{
            git::TaskStorage,
            taskchampion::{self, Replica},
            StrideRepository,
        },
        settings::{ApplicationPaths, Repository, Settings},
    },
    task::{Task, TaskStatus},
};
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
        println!("{active_char}{i:4}: {}", task.title);
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
        let repository = Repository::default();
        let uuid = repository.uuid;
        settings.repositories.push(repository);
        settings.current_repository = Some(uuid);
        Settings::save(settings.clone())?;
        uuid
    };

    let repository: &mut dyn StrideRepository = match args.repository {
        RepositoryType::Git => &mut TaskStorage::new(
            current_repository,
            &support_dir.join("repository").to_string_lossy(),
            &settings,
        )?,
        RepositoryType::TaskChampion => {
            let data_dir =
                choose_path_suffix(&dirs::data_dir().context("could not get data directory")?);
            let config_dir =
                choose_path_suffix(&dirs::config_dir().context("could not get config directory")?);

            let db_path = data_dir.join("taskchampion");
            let config_path = config_dir.join("config.toml");

            let (url, client_id, encryption_secret) = {
                #[derive(Deserialize)]
                struct Config {
                    sync: Sync,
                }
                #[derive(Deserialize)]
                struct Sync {
                    server: Server,
                    encryption_secret: String,
                }
                #[derive(Deserialize)]
                struct Server {
                    origin: Url,
                    client_id: Uuid,
                }

                let file = fs::read_to_string(&config_path).with_context(|| {
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

            let server_config = taskchampion::ServerConfig::Remote {
                url: url.to_string(),
                client_id,
                encryption_secret: encryption_secret.as_bytes().to_vec(),
            };
            let constraint_environment = false;

            &mut Replica::new(&db_path, server_config, constraint_environment).with_context(
                || {
                    format!(
                        "Failed to initialize taskchampion storage at: {}",
                        db_path.display()
                    )
                },
            )?
        }
    };

    match args.mode {
        Mode::Search { filter } => {
            let filter = Filter {
                search: filter.join(" "),
                status: [TaskStatus::Pending].into(),
                ..Default::default()
            };
            let tasks = repository.tasks_with_filter(&filter)?;
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
            repository.add(task)?;
        }
        Mode::Sync => {
            repository.sync()?;
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
        Mode::Export { filepath } => {
            let contents = repository.export()?;
            if let Some(filepath) = filepath {
                fs::write(filepath, contents)?;
            } else {
                println!("{contents}");
            }
        }
        Mode::Import { filepath } => {
            let contents = if let Some(filepath) = filepath {
                fs::read_to_string(filepath)?
            } else {
                let mut contents = String::new();
                std::io::stdin().read_to_string(&mut contents)?;
                contents
            };
            repository.import(&contents)?;
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
            let mut plugin_manager = PluginManager::new()?;
            plugin_manager.load()?;
            match command {
                None => {
                    let plugins = plugin_manager.list()?;
                    for plugin in plugins {
                        println!("{}", plugin.manifest.name);
                    }
                }
                Some(command) => match command {
                    cli::PluginCommand::Import { filepath } => {
                        plugin_manager.import(filepath.to_string_lossy().to_string())?;
                    }
                },
            };
        }
    }

    repository
        .commit()
        .context("Failed to commit the change to the repository")?;

    Ok(())
}
