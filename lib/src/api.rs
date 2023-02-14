use crate::soundcore_bluetooth::traits::SoundcoreDeviceConnectionError;

use self::traits::SoundcoreDeviceRegistry;

#[cfg(feature = "demo")]
pub(crate) mod demo;
#[cfg(not(feature = "demo"))]
pub(crate) mod real;
pub mod traits;

pub async fn new_soundcore_device_registry(
) -> Result<impl SoundcoreDeviceRegistry, SoundcoreDeviceConnectionError> {
    #[cfg(not(feature = "demo"))]
    {
        use self::real::RealSoundcoreDeviceRegistry;
        use crate::soundcore_bluetooth;

        let connection_registry = soundcore_bluetooth::new_connection_registry().await?;
        RealSoundcoreDeviceRegistry::new(connection_registry).await
    }
    #[cfg(feature = "demo")]
    {
        use self::demo::DemoSoundcoreDeviceRegistry;
        Ok(DemoSoundcoreDeviceRegistry::new())
    }
}
