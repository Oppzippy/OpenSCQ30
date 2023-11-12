mod btleplug_connection;
mod btleplug_connection_registry;
mod btleplug_error;
pub mod mac_address;

use btleplug::platform::Manager;
pub use btleplug_connection::*;
pub use btleplug_connection_registry::*;
pub use btleplug_error::*;

pub(crate) async fn new_connection_registry() -> crate::Result<BtlePlugConnectionRegistry> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()
        .unwrap();
    runtime
        .handle()
        .to_owned()
        .spawn(async move {
            let manager = Manager::new().await?;
            Ok(BtlePlugConnectionRegistry::new(manager, runtime))
        })
        .await
        .unwrap()
}
