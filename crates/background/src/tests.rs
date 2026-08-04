use std::{
    fmt::Display,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use async_trait::async_trait;

use crate::{
    AsyncRunnable, Background, Name, Reactor, Specification,
    error::{Result, ToBackgroundError},
};

#[derive(Debug, Default)]
struct Waitable {
    mutex: Mutex<()>,
    condvar: Condvar,
    flag: AtomicBool,
}

#[allow(unused)]
impl Waitable {
    fn new() -> Self {
        Self::default()
    }

    fn wait_for(&self, dur: Duration) -> bool {
        if self.flag.load(Ordering::SeqCst) {
            let lock = self.mutex.lock().unwrap();
            let (_lock, result) = self.condvar.wait_timeout(lock, dur).unwrap();
            result.timed_out()
        } else {
            false
        }
    }

    fn notify(&self) {
        if self
            .flag
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            == Ok(true)
        {
            self.condvar.notify_all();
        }
    }

    fn reset(&self) {
        if self
            .flag
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            == Ok(false)
        {}
    }
}

#[derive(Debug)]
struct State {
    waitable: Waitable,
    result: Mutex<Result<bool>>,
}

#[derive(Debug)]
struct TestTask {
    state: Arc<State>,
}

impl Specification for TestTask {}

#[async_trait]
impl AsyncRunnable for TestTask {
    async fn run(&mut self) -> Result<bool> {
        self.state.waitable.notify();
        self.state.result.lock().unwrap().clone()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TestError {}

impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl std::error::Error for TestError {}

impl ToBackgroundError for TestError {}

#[derive(Debug)]
struct TestHook {
    tx: std::sync::mpsc::Sender<(Name, Result<bool, crate::Error>)>,
}

impl Reactor for TestHook {
    fn on_task_result(&self, task_name: Name, result: Result<bool, crate::Error>) {
        self.tx.send((task_name, result)).unwrap();
    }
}

#[test]
fn run_task_success() {
    let name = Name {
        method: "task.sync".into(),
        unique: None,
    };

    let state = Arc::new(State {
        waitable: Waitable::new(),
        result: Mutex::new(Ok(true)),
    });
    let (tx, rx) = std::sync::mpsc::channel();
    let hook = Arc::new(TestHook { tx });
    let mut background = Background::new(hook);
    background
        .enqueue(
            name.clone(),
            Box::new(TestTask {
                state: state.clone(),
            }),
        )
        .unwrap();

    assert!(
        !state.waitable.wait_for(Duration::from_secs(3)),
        "task did not run"
    );

    let Ok((task_name, result)) = rx.recv_timeout(Duration::from_secs(5)) else {
        panic!("task should generate output");
    };

    let Ok(success) = result else {
        panic!("expected output done, got: {result:#?}");
    };

    assert_eq!(task_name, name);
    assert!(success);
}

#[test]
fn run_task_error() {
    let name = Name {
        method: "task.sync".into(),
        unique: None,
    };

    let state = Arc::new(State {
        waitable: Waitable::new(),
        result: Mutex::new(Err(TestError {}.into())),
    });
    let (tx, rx) = std::sync::mpsc::channel();
    let hook = Arc::new(TestHook { tx });
    let mut background = Background::new(hook);
    background
        .enqueue(
            name.clone(),
            Box::new(TestTask {
                state: state.clone(),
            }),
        )
        .unwrap();

    assert!(
        !state.waitable.wait_for(Duration::from_secs(3)),
        "task did not run"
    );

    let Ok((task_name, result)) = rx.recv_timeout(Duration::from_secs(5)) else {
        panic!("task should generate output");
    };

    let Err(error) = result else {
        panic!("expected output done, got: {result:#?}");
    };

    assert_eq!(task_name, name);

    assert_eq!(error.downcast_ref::<TestError>(), Some(&TestError {}));
}
