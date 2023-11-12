pub mod connection;
pub mod device;

#[cfg(any(feature = "bluetooth", feature = "demo"))]
use crate::futures::Futures;

#[cfg(any(feature = "bluetooth", feature = "demo"))]
use self::device::DeviceRegistry;

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry<FuturesType>() -> crate::Result<impl DeviceRegistry>
where
    FuturesType: Futures,
{
    use crate::soundcore_device::device::SoundcoreDeviceRegistry;
    let connection_registry =
        crate::soundcore_device::connection::new_connection_registry().await?;
    SoundcoreDeviceRegistry::<_, FuturesType>::new(connection_registry).await
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry<FuturesType>() -> crate::Result<impl DeviceRegistry>
where
    FuturesType: Futures,
{
    use crate::demo::device::DemoDeviceRegistry;
    Ok(DemoDeviceRegistry::<FuturesType>::new())
}
