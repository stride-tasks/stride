use smol::{Executor, Task};

use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub(crate) struct Runtime {
    executor: Arc<Executor<'static>>,

    #[allow(unused)]
    shutdown_tx: async_channel::Sender<()>,
    #[allow(unused)]
    workers: Vec<JoinHandle<()>>,
}

impl Runtime {
    pub(crate) fn new(num_threads: Option<NonZeroUsize>) -> Self {
        let num_threads = num_threads.map_or_else(
            || thread::available_parallelism().map_or(4, std::num::NonZero::get),
            std::num::NonZero::get,
        );

        let executor = Arc::new(Executor::new());
        let (shutdown_tx, shutdown_rx) = async_channel::unbounded::<()>();

        let workers = (0..num_threads)
            .map(|i| {
                let ex = Arc::clone(&executor);
                let rx = shutdown_rx.clone(); // Keep the receiver for each worker

                thread::Builder::new()
                    .name(format!("stride-runtime-worker-{i}"))
                    .spawn(move || {
                        smol::block_on(ex.run(async move {
                            let _ = rx.recv().await;
                        }));
                    })
                    .expect("unable to create thread pool")
            })
            .collect();

        Self {
            executor,
            shutdown_tx,
            workers,
        }
    }

    pub(crate) fn spawn<T: Send + 'static>(
        &self,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Task<T> {
        self.executor.spawn(future)
    }
}
