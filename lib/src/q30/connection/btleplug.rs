mod btleplug_connection;
mod btleplug_connection_registry;
mod btleplug_error;
pub mod mac_address;

use btleplug::platform::Manager;
pub use btleplug_connection::*;
pub use btleplug_connection_registry::*;
pub use btleplug_error::*;
use tokio_rt_multi_thread::new_multi_thread;

pub(crate) async fn new_connection_registry() -> crate::Result<BtlePlugConnectionRegistry> {
    let runtime = new_multi_thread()
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
