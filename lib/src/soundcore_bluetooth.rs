pub mod stub;
pub mod traits;

#[cfg(all(
    feature = "bluetooth",
    any(target_os = "windows", target_os = "macos", target_os = "linux")
))]
use self::traits::{SoundcoreDeviceConnectionError, SoundcoreDeviceConnectionRegistry};

#[cfg(all(
    feature = "bluetooth",
    any(target_os = "windows", target_os = "macos", target_os = "linux")
))]
pub(crate) mod btleplug;
#[cfg(all(
    feature = "bluetooth",
    any(target_os = "windows", target_os = "macos", target_os = "linux")
))]
pub async fn new_connection_registry(
) -> Result<impl SoundcoreDeviceConnectionRegistry, SoundcoreDeviceConnectionError> {
    btleplug::new_connection_registry().await
}
