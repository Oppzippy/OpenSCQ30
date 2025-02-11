pub mod connection;
pub mod device;
mod session;
pub mod settings;
pub use session::*;

#[cfg(any(feature = "bluetooth", feature = "demo"))]
use crate::futures::Futures;

#[cfg(any(feature = "bluetooth", feature = "demo"))]
use self::device::DeviceRegistry;

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry(
    handle: tokio::runtime::Handle,
) -> crate::Result<impl DeviceRegistry> {
    use crate::{futures::TokioFutures, soundcore_device::device::SoundcoreDeviceRegistry};
    let connection_registry =
        crate::soundcore_device::connection::new_connection_registry(Some(handle)).await?;
    SoundcoreDeviceRegistry::<_, TokioFutures>::new(connection_registry).await
}

#[cfg(all(feature = "bluetooth", not(feature = "demo")))]
pub async fn new_soundcore_device_registry_with_custom_runtime<FuturesType>(
) -> crate::Result<impl DeviceRegistry>
where
    FuturesType: Futures,
{
    use crate::soundcore_device::device::SoundcoreDeviceRegistry;
    let connection_registry =
        crate::soundcore_device::connection::new_connection_registry(None).await?;
    SoundcoreDeviceRegistry::<_, FuturesType>::new(connection_registry).await
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry(
    _handle: tokio::runtime::Handle,
) -> crate::Result<impl DeviceRegistry> {
    use crate::{demo::device::DemoDeviceRegistry, futures::TokioFutures};
    Ok(DemoDeviceRegistry::<TokioFutures>::new())
}

#[cfg(feature = "demo")]
pub async fn new_soundcore_device_registry_with_custom_runtime<FuturesType>(
) -> crate::Result<impl DeviceRegistry>
where
    FuturesType: Futures,
{
    use crate::demo::device::DemoDeviceRegistry;
    Ok(DemoDeviceRegistry::<FuturesType>::new())
}
