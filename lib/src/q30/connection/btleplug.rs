mod soundcore_device_connection;
mod soundcore_device_connection_descriptor;
mod soundcore_device_connection_error;
mod soundcore_device_connection_registry;

use btleplug::platform::Manager;
pub use soundcore_device_connection::*;
pub use soundcore_device_connection_descriptor::*;
pub use soundcore_device_connection_error::*;
pub use soundcore_device_connection_registry::*;

pub(crate) async fn new_connection_registry(
) -> crate::Result<BtlePlugSoundcoreDeviceConnectionRegistry> {
    let manager = Manager::new().await?;
    Ok(BtlePlugSoundcoreDeviceConnectionRegistry::new(manager))
}
