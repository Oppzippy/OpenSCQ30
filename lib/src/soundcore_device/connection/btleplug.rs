mod btleplug_connection;
mod btleplug_connection_registry;
mod btleplug_error;
pub mod mac_address;

use btleplug::platform::Manager;
pub use btleplug_connection::*;
pub use btleplug_connection_registry::*;
pub use btleplug_error::*;
use futures::Future;
use tokio::{
    runtime::{Handle, Runtime},
    task::JoinHandle,
};

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

pub(crate) async fn new_connection_registry(
    handle: Option<Handle>,
) -> crate::Result<BtlePlugConnectionRegistry> {
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
        .spawn(async move {
            let manager = Manager::new().await?;
            Ok(BtlePlugConnectionRegistry::new(manager, runtime_or_handle))
        })
        .await
        .unwrap()
}
