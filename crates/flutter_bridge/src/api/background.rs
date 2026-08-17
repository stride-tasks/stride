use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock, Mutex};
use stride_background::{Background, Name, Reactor};
use uuid::Uuid;

use crate::{RustError, frb_generated::StreamSink};

static STATE: LazyLock<Mutex<Option<State>>> = LazyLock::new(Mutex::default);

#[frb(ignore)]
struct State {
    #[allow(unused)]
    background: Background,
    stream_sink: StreamSink<BackgroundResult>,
}

#[derive(Debug)]
#[frb(ignore)]
struct BridgeHook;

impl Reactor for BridgeHook {
    fn on_task_start(&self, name: Name) {
        STATE.clear_poison();
        let mut lock = STATE.lock().unwrap();
        let Some(state) = lock.as_mut() else {
            return;
        };

        let result = state.stream_sink.add(BackgroundResult::Start {
            task: name.to_string(),
        });

        drop(result);
    }

    fn on_task_result(&self, name: Name, result: Result<bool, stride_background::Error>) {
        STATE.clear_poison();
        let mut lock = STATE.lock().unwrap();
        let Some(state) = lock.as_mut() else {
            return;
        };

        let result = match result {
            Ok(success) => BackgroundResult::Done {
                task: name.to_string(),
                success,
            },
            Err(error) => BackgroundResult::Error {
                task: name.to_string(),
                error: RustError::from(error),
            },
        };
        state.stream_sink.add(result).unwrap();
    }
}

#[frb(non_opaque)]
pub enum BackgroundResult {
    Start { task: String },
    Done { task: String, success: bool },
    Error { task: String, error: RustError },
}

pub fn init(stream_sink: StreamSink<BackgroundResult>) {
    STATE.clear_poison();
    let mut lock = STATE.lock().unwrap();

    match lock.as_mut() {
        Some(state) => {
            // TODO: clear previous tasks
            //       state.background.clear();
            state.stream_sink = stream_sink;
        }
        None => {
            *lock = Some(State {
                background: Background::new(Arc::new(BridgeHook)),
                stream_sink,
            });
        }
    }
}

#[frb(ignore)]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RepositorySpec {
    pub(crate) id: Uuid,
}

#[frb(ignore)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "kebab-case")]
enum Method {
    #[serde(rename = "task.sync")]
    TaskSync { repository: RepositorySpec },
}

#[frb(ignore)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct BgTask {
    #[serde(flatten)]
    method: Method,
}

#[frb(ignore)]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TaskSync {
    pub(crate) repository: RepositorySpec,
}

pub fn execute(_: &str) -> Result<(), RustError> {
    Ok(())
}
