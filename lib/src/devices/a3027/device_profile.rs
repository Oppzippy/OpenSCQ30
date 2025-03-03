use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    api::{
        connection::{RfcommBackend, RfcommConnection},
        device::OpenSCQ30Device,
        settings::{CategoryId, Setting, SettingId, Value},
    },
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        device::{SoundcoreDevice, SoundcoreDeviceRegistry},
        implementation::StandardImplementation,
        modules::{
            ModuleCollection, ModuleCollectionSpawnPacketHandlerExt,
            equalizer::AddEqualizerExt,
            sound_modes::{AddSoundModesExt, AvailableSoundModes},
        },
        packets::{
            inbound::TryIntoInboundPacket,
            outbound::{OutboundPacketBytesExt, RequestStatePacket},
        },
        structures::{AmbientSoundMode, NoiseCancelingMode},
    },
    soundcore_device::{
        device::packet_io_controller::PacketIOController, device_model::DeviceModel,
    },
    storage::OpenSCQ30Database,
};

use super::{demo::DemoConnectionRegistry, packets::A3027StateUpdatePacket, state::A3027State};
pub(crate) const A3027_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: Some(crate::device_profile::AvailableSoundModes {
            ambient_sound_modes: &[
                AmbientSoundMode::Normal,
                AmbientSoundMode::Transparency,
                AmbientSoundMode::NoiseCanceling,
            ],
            transparency_modes: &[],
            noise_canceling_modes: &[
                NoiseCancelingMode::Transport,
                NoiseCancelingMode::Indoor,
                NoiseCancelingMode::Outdoor,
            ],
            custom_noise_canceling: false,
        }),
        has_hear_id: false,
        num_equalizer_channels: 1,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: false,
        dynamic_range_compression_min_firmware_version: None,
        has_button_configuration: false,
        has_wear_detection: true,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::SoundcoreA3027, DeviceModel::SoundcoreA3030],
    implementation: || StandardImplementation::new::<A3027StateUpdatePacket>(),
};

pub fn device_registry<B: RfcommBackend>(
    backend: B,
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDeviceRegistry<B, A3027Device<B::ConnectionType>> {
    SoundcoreDeviceRegistry::new(backend, database, device_model)
}

pub fn demo_device_registry(
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDeviceRegistry<
    DemoConnectionRegistry,
    A3027Device<<DemoConnectionRegistry as RfcommBackend>::ConnectionType>,
> {
    SoundcoreDeviceRegistry::new(
        DemoConnectionRegistry::new(
            device_model.to_string(),
            A3027StateUpdatePacket::default().bytes(),
        ),
        database,
        device_model,
    )
}

pub struct A3027Device<ConnectionType: RfcommConnection + Send + Sync> {
    device_model: DeviceModel,
    state_sender: watch::Sender<A3027State>,
    module_collection: Arc<ModuleCollection<A3027State>>,
    _packet_io_controller: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType> SoundcoreDevice<ConnectionType> for A3027Device<ConnectionType>
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
        let state_update_packet: A3027StateUpdatePacket = packet_io_controller
            .send(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let (state_sender, _) = watch::channel::<A3027State>(state_update_packet.into());

        let mut module_collection = ModuleCollection::default();
        module_collection.add_sound_modes(
            packet_io_controller.clone(),
            AvailableSoundModes {
                ambient_sound_modes: vec![
                    AmbientSoundMode::Normal,
                    AmbientSoundMode::Transparency,
                    AmbientSoundMode::NoiseCanceling,
                ],
                transparency_modes: vec![],
                noise_canceling_modes: vec![
                    NoiseCancelingMode::Transport,
                    NoiseCancelingMode::Indoor,
                    NoiseCancelingMode::Outdoor,
                ],
            },
        );
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

#[async_trait]
impl<ConnectionType> OpenSCQ30Device for A3027Device<ConnectionType>
where
    ConnectionType: RfcommConnection + 'static + Send + Sync,
{
    fn model(&self) -> DeviceModel {
        self.device_model
    }

    fn categories(&self) -> Vec<CategoryId> {
        self.module_collection.setting_manager.categories().to_vec()
    }

    fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId> {
        self.module_collection.setting_manager.category(category_id)
    }

    fn setting(&self, setting_id: &SettingId) -> Option<Setting> {
        let state = self.state_sender.borrow().to_owned();
        self.module_collection
            .setting_manager
            .get(&state, setting_id)
    }

    async fn set_setting_values(
        &self,
        setting_values: Vec<(SettingId, Value)>,
    ) -> crate::Result<()> {
        let mut target_state = self.state_sender.borrow().clone();
        for (setting_id, value) in setting_values {
            self.module_collection
                .setting_manager
                .set(&mut target_state, &setting_id, value)
                .await
                .unwrap()?;
        }
        for modifier in &self.module_collection.state_modifiers {
            modifier
                .move_to_state(&self.state_sender, &target_state)
                .await?;
        }
        Ok(())
    }
}
