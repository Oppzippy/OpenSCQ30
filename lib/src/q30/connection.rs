#[cfg(feature = "bluetooth")]
use crate::api::connection::ConnectionRegistry;
#[cfg(feature = "bluetooth")]
pub(crate) mod btleplug;

#[cfg(feature = "bluetooth")]
pub async fn new_connection_registry() -> crate::Result<impl ConnectionRegistry> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        btleplug::new_connection_registry().await
    }
}
