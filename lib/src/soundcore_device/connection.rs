#[cfg(feature = "bluetooth")]
use crate::api::connection::ConnectionRegistry;
#[cfg(feature = "bluer")]
pub mod bluer;
#[cfg(all(feature = "bluetooth", any(target_os = "macos", target_os = "linux")))]
pub(crate) mod btleplug;
#[cfg(all(feature = "bluetooth", target_os = "windows"))]
pub(crate) mod windows;

#[cfg(feature = "bluetooth")]
pub async fn new_connection_registry(
    handle: Option<tokio::runtime::Handle>,
) -> crate::Result<impl ConnectionRegistry> {
    #[cfg(target_os = "linux")]
    {
        #[cfg(feature = "bluer")]
        {
            bluer::new_connection_registry(handle).await
        }
        #[cfg(not(feature = "bluer"))]
        {
            btleplug::new_connection_registry(handle).await
        }
    }
    #[cfg(target_os = "macos")]
    {
        btleplug::new_connection_registry(handle).await
    }
    #[cfg(target_os = "windows")]
    {
        std::mem::drop(handle);
        windows::new_connection_registry().await
    }
}
