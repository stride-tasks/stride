//! Stride's background task crate implementation.

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
    sync::Arc,
    thread::JoinHandle,
    time::Duration,
};

use async_trait::async_trait;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub use crate::error::{Error, Result, ToBackgroundError};

mod error;

#[cfg(test)]
mod tests;

pub trait Specification: Debug {
    fn initial_delay(&self) -> Option<Duration> {
        None
    }

    fn frequency(&self) -> Option<Duration> {
        None
    }

    fn tag(&self) -> Option<Arc<str>> {
        None
    }
}

#[async_trait]
pub trait AsyncRunnable: Specification + Send + 'static {
    /// Run the task.
    /// 
    /// # Errors
    /// 
    /// This function returns an error if the task fails to run.
    async fn run(&mut self) -> Result<bool>;
}

pub trait Runnable: Specification + Send + 'static {
    /// Run the task.
    /// 
    /// # Errors
    /// 
    /// This function returns an error if the task fails to run.
    fn run(&mut self) -> Result<bool>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name {
    pub method: Arc<str>,
    pub unique: Option<Arc<str>>,
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(unique) = &self.unique {
            write!(f, "{}:{unique}", self.method)
        } else {
            f.write_str(&self.method)
        }
    }
}

impl FromStr for Name {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((method, unique)) = s.split_once(':') else {
            return Ok(Name {
                method: s.into(),
                unique: None,
            });
        };
        Ok(Name {
            method: method.into(),
            unique: Some(unique.into()),
        })
    }
}

#[derive(Debug)]
enum Message {
    Task {
        name: Name,
        task: Box<dyn AsyncRunnable>,
    },
    Close,
}

async fn background_main(hook: Arc<dyn Reactor>, mut receiver: UnboundedReceiver<Message>) {
    let mut tasks: HashMap<Option<Arc<str>>, Vec<tokio::task::JoinHandle<()>>> = HashMap::new();
    while let Some(message) = receiver.recv().await {
        match message {
            Message::Task { name, mut task } => {
                let hook = hook.clone();
                let tag = task.tag();
                let join = tokio::spawn(async move {
                    if let Some(duration) = task.initial_delay() {
                        tokio::time::sleep(duration).await;
                    }

                    loop {
                        hook.on_task_start(name.clone());

                        let result = task.run().await;
                        hook.on_task_result(name.clone(), result);

                        let Some(frequency) = task.frequency() else {
                            break;
                        };

                        log::trace!("Attempt :: {name} :: {frequency:?}");
                        tokio::time::sleep(frequency).await;
                    }
                });

                tasks.entry(tag).or_default().push(join);
            }
            Message::Close => break,
        }
    }

    for (_, joins) in tasks {
        for join in joins {
            join.await.unwrap();
        }
    }
}

fn background_thread(hook: Arc<dyn Reactor>, receiver: UnboundedReceiver<Message>) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async move {
        background_main(hook, receiver).await;
    });
}

#[derive(Debug)]
pub struct Background {
    sender: UnboundedSender<Message>,
    #[allow(unused)]
    join_handle: Option<JoinHandle<()>>,
}

impl Background {
    #[must_use]
    pub fn new(hook: Arc<dyn Reactor>) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<Message>();

        let join_handle = std::thread::spawn(move || background_thread(hook, receiver));

        Self {
            sender,
            join_handle: join_handle.into(),
        }
    }

    /// Enqueue a task to be run in the background.
    /// 
    /// # Errors
    /// 
    /// This function returns an error if the background thread has been closed.
    pub fn enqueue(&mut self, name: Name, task: Box<dyn AsyncRunnable>) -> Result<()> {
        self.sender
            .send(Message::Task { name, task })
            .map_err(|_| Error::BackgroundThreadClosed)?;
        Ok(())
    }
}

impl Drop for Background {
    fn drop(&mut self) {
        self.sender.send(Message::Close).unwrap();
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().unwrap();
        }
    }
}

#[derive(Debug)]
struct AsyncWrapper {
    runnable: Box<dyn Runnable>,
}

impl Specification for AsyncWrapper {
    fn initial_delay(&self) -> Option<Duration> {
        self.runnable.initial_delay()
    }
    fn frequency(&self) -> Option<Duration> {
        self.runnable.frequency()
    }
    fn tag(&self) -> Option<Arc<str>> {
        self.runnable.tag()
    }
}

#[async_trait]
impl AsyncRunnable for AsyncWrapper {
    async fn run(&mut self) -> Result<bool> {
        self.runnable.run()
    }
}

impl From<Box<dyn Runnable>> for Box<dyn AsyncRunnable> {
    fn from(runnable: Box<dyn Runnable>) -> Self {
        Box::new(AsyncWrapper { runnable })
    }
}

pub trait Reactor: Debug + Send + Sync + 'static {
    fn on_task_start(&self, name: Name) {
        drop(name);
    }
    fn on_task_result(&self, name: Name, result: Result<bool, Error>) {
        drop(name);
        drop(result);
    }
}
