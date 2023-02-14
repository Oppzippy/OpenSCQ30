pub mod stub;
pub mod traits;

#[cfg(feature = "bluetooth")]
use self::traits::{SoundcoreDeviceConnectionError, SoundcoreDeviceConnectionRegistry};

#[cfg(feature = "bluetooth")]
pub(crate) mod btleplug;

#[cfg(feature = "bluetooth")]
pub async fn new_connection_registry(
) -> Result<impl SoundcoreDeviceConnectionRegistry, SoundcoreDeviceConnectionError> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        btleplug::new_connection_registry().await
    }
}
