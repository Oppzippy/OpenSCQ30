mod btleplug_connection;
mod btleplug_connection_descriptor;
mod btleplug_connection_registry;
mod btleplug_error;
pub mod mac_address;

use btleplug::platform::Manager;
pub use btleplug_connection::*;
pub use btleplug_connection_descriptor::*;
pub use btleplug_connection_registry::*;
pub use btleplug_error::*;

pub(crate) async fn new_connection_registry() -> crate::Result<BtlePlugConnectionRegistry> {
    let manager = Manager::new().await?;
    Ok(BtlePlugConnectionRegistry::new(manager))
}
