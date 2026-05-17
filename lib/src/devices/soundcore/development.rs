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

pub fn device_registry(
    backend: Arc<dyn RfcommBackend + Send + Sync>,
    _database: Arc<storage::OpenSCQ30Database>,
    _device_model: DeviceModel,
) -> SoundcoreDevelopmentDeviceRegistry {
    SoundcoreDevelopmentDeviceRegistry::new(backend)
}

pub fn demo_device_registry(
    _database: Arc<storage::OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDevelopmentDeviceRegistry {
    SoundcoreDevelopmentDeviceRegistry::new(Arc::new(DemoConnectionRegistry::new(
        device_model,
        HashMap::from([(
            packet::inbound::STATE_COMMAND,
            packet::Inbound::new(packet::inbound::STATE_COMMAND, vec![1, 2, 3]),
        )]),
        SoundcoreDeviceConfig::default(),
    )))
}
