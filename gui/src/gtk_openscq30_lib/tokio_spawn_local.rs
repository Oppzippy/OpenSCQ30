use futures::Future;
use tokio::{runtime::Runtime, task::JoinHandle};

pub trait TokioSpawnLocal {
    fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static;
}

impl TokioSpawnLocal for Runtime {
    fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let _enter = self.enter();
        tokio::task::spawn_local(future)
    }
}
