pub mod connection;
pub mod device;

use self::device::SoundcoreDeviceRegistry;

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry() -> crate::Result<impl SoundcoreDeviceRegistry> {
    use crate::q30::device::RealSoundcoreDeviceRegistry;
    let connection_registry = crate::q30::connection::new_connection_registry().await?;
    RealSoundcoreDeviceRegistry::new(connection_registry).await
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry() -> crate::Result<impl SoundcoreDeviceRegistry> {
    use crate::demo::device::DemoSoundcoreDeviceRegistry;
    Ok(DemoSoundcoreDeviceRegistry::new())
}
