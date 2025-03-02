pub mod connection;
pub mod device;
pub mod quick_presets;
mod session;
pub mod settings;
pub use session::*;

#[cfg(any(feature = "bluetooth", feature = "demo"))]
use self::device::DeviceRegistry;

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry(
    handle: tokio::runtime::Handle,
) -> crate::Result<impl DeviceRegistry> {
    use crate::soundcore_device::device::SoundcoreDeviceRegistry;
    let connection_registry =
        crate::soundcore_device::connection::new_connection_registry(Some(handle)).await?;
    SoundcoreDeviceRegistry::<_>::new(connection_registry).await
}

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry_with_custom_runtime()
-> crate::Result<impl DeviceRegistry> {
    use crate::soundcore_device::device::SoundcoreDeviceRegistry;
    let connection_registry =
        crate::soundcore_device::connection::new_connection_registry(None).await?;
    SoundcoreDeviceRegistry::<_>::new(connection_registry).await
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry(
    _handle: tokio::runtime::Handle,
) -> crate::Result<impl DeviceRegistry> {
    use crate::demo::device::DemoDeviceRegistry;
    Ok(DemoDeviceRegistry::new())
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry_with_custom_runtime()
-> crate::Result<impl DeviceRegistry> {
    use crate::demo::device::DemoDeviceRegistry;
    Ok(DemoDeviceRegistry::new())
}
