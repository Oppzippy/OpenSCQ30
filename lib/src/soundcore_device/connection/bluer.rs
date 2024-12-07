use connection_registry::BluerConnectionRegistry;
use tokio::runtime::Handle;

use super::btleplug::RuntimeOrHandle;

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
