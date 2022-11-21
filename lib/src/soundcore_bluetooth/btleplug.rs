use btleplug::platform::Manager;

use self::soundcore_device_connection_registry::BtlePlugSoundcoreDeviceConnectionRegistry;

use super::traits::soundcore_device_connection_error::SoundcoreDeviceConnectionError;

pub mod soundcore_device_connection;
pub mod soundcore_device_connection_error;
pub mod soundcore_device_connection_registry;

pub async fn new_handler(
) -> Result<BtlePlugSoundcoreDeviceConnectionRegistry, SoundcoreDeviceConnectionError> {
    let manager = Manager::new()
        .await
        .map_err(SoundcoreDeviceConnectionError::from)?;
    Ok(BtlePlugSoundcoreDeviceConnectionRegistry::new(manager))
}
