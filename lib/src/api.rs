#[cfg(any(feature = "bluetooth", feature = "demo"))]
use crate::{
    api::traits::SoundcoreDeviceRegistry,
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
};

#[cfg(feature = "demo")]
pub(crate) mod demo;
#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub(crate) mod real;
pub mod traits;

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry(
) -> Result<impl SoundcoreDeviceRegistry, SoundcoreDeviceConnectionError> {
    use self::real::RealSoundcoreDeviceRegistry;
    use crate::soundcore_bluetooth;

    let connection_registry = soundcore_bluetooth::new_connection_registry().await?;
    RealSoundcoreDeviceRegistry::new(connection_registry).await
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry(
) -> Result<impl SoundcoreDeviceRegistry, SoundcoreDeviceConnectionError> {
    use self::demo::DemoSoundcoreDeviceRegistry;
    Ok(DemoSoundcoreDeviceRegistry::new())
}
