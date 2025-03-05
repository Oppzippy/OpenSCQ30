use std::sync::Arc;

use tokio::sync::watch;

use crate::{
    api::connection::{RfcommBackend, RfcommConnection},
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        demo::DemoConnectionRegistry,
        device::{SoundcoreDevice, SoundcoreDeviceRegistry},
        implementation::StandardImplementation,
        macros::impl_soundcore_device,
        modules::{ModuleCollection, ModuleCollectionSpawnPacketHandlerExt},
        packets::{
            inbound::TryIntoInboundPacket,
            outbound::{OutboundPacketBytesExt, RequestStatePacket},
        },
    },
    soundcore_device::{
        device::packet_io_controller::PacketIOController, device_model::DeviceModel,
    },
    storage::OpenSCQ30Database,
};

use super::{packets::A3033StateUpdatePacket, state::A3033State};

pub(crate) const A3033_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: None,
        has_hear_id: false,
        num_equalizer_channels: 1,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: false,
        dynamic_range_compression_min_firmware_version: None,
        has_button_configuration: false,
        has_wear_detection: false,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::SoundcoreA3033],
    implementation: || StandardImplementation::new::<A3033StateUpdatePacket>(),
};

pub fn device_registry<B: RfcommBackend>(
    backend: B,
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDeviceRegistry<B, A3033Device<B::ConnectionType>> {
    SoundcoreDeviceRegistry::new(backend, database, device_model)
}

pub fn demo_device_registry(
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDeviceRegistry<
    DemoConnectionRegistry,
    A3033Device<<DemoConnectionRegistry as RfcommBackend>::ConnectionType>,
> {
    SoundcoreDeviceRegistry::new(
        DemoConnectionRegistry::new(
            device_model.to_string(),
            A3033StateUpdatePacket::default().bytes(),
        ),
        database,
        device_model,
    )
}

pub struct A3033Device<ConnectionType: RfcommConnection + Send + Sync> {
    device_model: DeviceModel,
    state_sender: watch::Sender<A3033State>,
    module_collection: Arc<ModuleCollection<A3033State>>,
    _packet_io_controller: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType> SoundcoreDevice<ConnectionType> for A3033Device<ConnectionType>
where
    ConnectionType: RfcommConnection + 'static + Send + Sync,
{
    async fn new(
        database: Arc<OpenSCQ30Database>,
        connection: ConnectionType,
        device_model: DeviceModel,
    ) -> crate::Result<Self> {
        let (packet_io_controller, packet_receiver) =
            PacketIOController::<ConnectionType>::new(Arc::new(connection)).await?;
        let packet_io_controller = Arc::new(packet_io_controller);
        let state_update_packet: A3033StateUpdatePacket = packet_io_controller
            .send(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let (state_sender, _) = watch::channel::<A3033State>(state_update_packet.into());

        let mut module_collection = ModuleCollection::<A3033State>::default();
        module_collection.add_state_update();
        module_collection
            .add_equalizer(packet_io_controller.clone(), database, device_model, false)
            .await;

        let module_collection = Arc::new(module_collection);
        module_collection.spawn_packet_handler(state_sender.clone(), packet_receiver);

        Ok(Self {
            device_model,
            state_sender,
            _packet_io_controller: packet_io_controller,
            module_collection,
        })
    }
}

impl_soundcore_device!(
    A3033Device,
    model = device_model,
    module_collection = module_collection,
    state_sender = state_sender
);
