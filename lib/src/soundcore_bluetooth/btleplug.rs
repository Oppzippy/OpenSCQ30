mod soundcore_device_connection;
mod soundcore_device_connection_error;
mod soundcore_device_connection_registry;

use btleplug::platform::Manager;
pub use soundcore_device_connection::*;
pub use soundcore_device_connection_error::*;
pub use soundcore_device_connection_registry::*;

use super::traits::SoundcoreDeviceConnectionError;

pub async fn new_connection_registry(
) -> Result<BtlePlugSoundcoreDeviceConnectionRegistry, SoundcoreDeviceConnectionError> {
    let manager = Manager::new()
        .await
        .map_err(SoundcoreDeviceConnectionError::from)?;
    Ok(BtlePlugSoundcoreDeviceConnectionRegistry::new(manager))
}
