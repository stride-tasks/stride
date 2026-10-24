use futures_lite::future;
use smol::{Executor, Task};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub(crate) struct Runtime {
    executor: Arc<Executor<'static>>,

    #[allow(unused)]
    shutdown_tx: async_channel::Sender<()>,
    #[allow(unused)]
    workers: Mutex<Vec<JoinHandle<()>>>,
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
            workers: Mutex::new(workers),
        }
    }

    pub(crate) fn spawn<T: Send + 'static>(
        &self,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Task<T> {
        self.executor.spawn(future)
    }

    pub(crate) fn wait_for_tasks(&self) {
        // Close the shutdown channel to signal all workers to stop.
        self.shutdown_tx.close();

        future::block_on(async {
            if !self.executor.is_empty() {
                self.executor.tick().await;
            }
        });

        let mut lock = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for worker in lock.drain(..) {
            let _result = worker.join();
        }
    }
}
