#[cfg(feature = "bluetooth")]
use crate::api::connection::SoundcoreDeviceConnectionRegistry;
#[cfg(feature = "bluetooth")]
pub(crate) mod btleplug;

#[cfg(feature = "bluetooth")]
pub async fn new_connection_registry() -> crate::Result<impl SoundcoreDeviceConnectionRegistry> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        btleplug::new_connection_registry().await
    }
}
