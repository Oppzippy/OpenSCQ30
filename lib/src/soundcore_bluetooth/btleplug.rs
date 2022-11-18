use std::error::Error;

use btleplug::platform::Manager;

use self::soundcore_device_connection_registry::BtlePlugSoundcoreDeviceConnectionRegistry;

pub mod soundcore_device_connection;
pub mod soundcore_device_connection_registry;

pub async fn new_handler() -> Result<BtlePlugSoundcoreDeviceConnectionRegistry, Box<dyn Error>> {
    let manager = Manager::new().await?;
    return Ok(BtlePlugSoundcoreDeviceConnectionRegistry::new(manager));
}
