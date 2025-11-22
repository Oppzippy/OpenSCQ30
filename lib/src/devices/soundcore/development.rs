use std::{collections::HashMap, sync::Arc};

use crate::{
    api::connection::RfcommBackend,
    devices::{
        DeviceModel,
        soundcore::{
            common::{demo::DemoConnectionRegistry, device::SoundcoreDeviceConfig, packet},
            development::device::SoundcoreDevelopmentDeviceRegistry,
        },
    },
    storage,
};

mod device;

pub fn device_registry<B>(
    backend: B,
    _database: Arc<storage::OpenSCQ30Database>,
    _device_model: DeviceModel,
) -> SoundcoreDevelopmentDeviceRegistry<B>
where
    B: RfcommBackend + Send + Sync + 'static,
{
    SoundcoreDevelopmentDeviceRegistry::new(backend)
}

pub fn demo_device_registry(
    _database: Arc<storage::OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDevelopmentDeviceRegistry<
    crate::devices::soundcore::common::demo::DemoConnectionRegistry,
> {
    SoundcoreDevelopmentDeviceRegistry::new(DemoConnectionRegistry::new(
        device_model,
        HashMap::from([(
            packet::inbound::STATE_COMMAND,
            packet::Inbound::new(packet::inbound::STATE_COMMAND, vec![1, 2, 3]),
        )]),
        SoundcoreDeviceConfig::default(),
    ))
}
