#[cfg(feature = "bluetooth")]
use crate::api::connection::ConnectionRegistry;
#[cfg(all(feature = "bluetooth", any(target_os = "macos", target_os = "linux")))]
pub(crate) mod btleplug;
#[cfg(all(feature = "bluetooth", target_os = "windows"))]
pub(crate) mod windows;

#[cfg(feature = "bluetooth")]
pub async fn new_connection_registry(
    handle: Option<tokio::runtime::Handle>,
) -> crate::Result<impl ConnectionRegistry> {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        btleplug::new_connection_registry(handle).await
    }
    #[cfg(target_os = "windows")]
    {
        std::mem::drop(handle);
        windows::new_connection_registry().await
    }
}
