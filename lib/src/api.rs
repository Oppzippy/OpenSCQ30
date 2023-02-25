pub mod connection;
pub mod device;

#[cfg(any(feature = "bluetooth", feature = "demo"))]
use self::device::DeviceRegistry;

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry() -> crate::Result<impl DeviceRegistry> {
    use crate::q30::device::Q30DeviceRegistry;
    let connection_registry = crate::q30::connection::new_connection_registry().await?;
    Q30DeviceRegistry::new(connection_registry).await
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry() -> crate::Result<impl DeviceRegistry> {
    use crate::demo::device::DemoDeviceRegistry;
    Ok(DemoDeviceRegistry::new())
}
