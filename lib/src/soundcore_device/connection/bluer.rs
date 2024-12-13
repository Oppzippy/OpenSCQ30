use std::future::Future;

use connection_registry::BluerConnectionRegistry;
use tokio::{
    runtime::{Handle, Runtime},
    task::JoinHandle,
};

mod bluer_error;
mod connection;
mod connection_registry;

pub(crate) async fn new_connection_registry(
    handle: Option<Handle>,
) -> crate::Result<BluerConnectionRegistry> {
    let runtime_or_handle = handle.map(RuntimeOrHandle::Handle).unwrap_or_else(|| {
        RuntimeOrHandle::Runtime(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .worker_threads(1)
                .build()
                .unwrap(),
        )
    });
    runtime_or_handle
        .handle()
        .spawn(async move { BluerConnectionRegistry::new(runtime_or_handle).await })
        .await
        .unwrap()
}

pub enum RuntimeOrHandle {
    Runtime(Runtime),
    Handle(Handle),
}

impl RuntimeOrHandle {
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        match self {
            RuntimeOrHandle::Runtime(runtime) => runtime.spawn(future),
            RuntimeOrHandle::Handle(handle) => handle.spawn(future),
        }
    }

    pub fn handle(&self) -> Handle {
        match self {
            RuntimeOrHandle::Runtime(runtime) => runtime.handle().to_owned(),
            RuntimeOrHandle::Handle(handle) => handle.to_owned(),
        }
    }
}
