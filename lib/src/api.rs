pub mod connection;
pub mod device;

use crate::futures::Futures;

#[cfg(any(feature = "bluetooth", feature = "demo"))]
use self::device::DeviceRegistry;

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry<FuturesType>() -> crate::Result<impl DeviceRegistry>
where
    FuturesType: Futures,
{
    use crate::q30::device::Q30DeviceRegistry;
    let connection_registry = crate::q30::connection::new_connection_registry().await?;
    Q30DeviceRegistry::<_, FuturesType>::new(connection_registry).await
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry<FuturesType>() -> crate::Result<impl DeviceRegistry>
where
    FuturesType: Futures,
{
    use crate::demo::device::DemoDeviceRegistry;
    Ok(DemoDeviceRegistry::<FuturesType>::new())
}
