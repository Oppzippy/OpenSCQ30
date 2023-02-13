#[cfg(feature = "btleplug")]
pub(crate) mod btleplug;
pub mod stub;
pub mod traits;

#[cfg(feature = "btleplug")]
use self::traits::{SoundcoreDeviceConnectionError, SoundcoreDeviceConnectionRegistry};

#[cfg(feature = "btleplug")]
pub async fn new_connection_registry(
) -> Result<impl SoundcoreDeviceConnectionRegistry, SoundcoreDeviceConnectionError> {
    btleplug::new_connection_registry().await
}
