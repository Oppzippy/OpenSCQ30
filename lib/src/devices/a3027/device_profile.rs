use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    api::{
        connection::{Connection, ConnectionDescriptor, ConnectionRegistry},
        device::{GenericDeviceDescriptor, OpenSCQ30Device, OpenSCQ30DeviceRegistry},
        settings::{CategoryId, Setting, SettingId, Value},
    },
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        implementation::StandardImplementation,
        modules::{
            sound_modes::{AddSoundModesExt, AvailableSoundModes},
            ModuleCollection, ModuleCollectionSpawnPacketHandlerExt,
        },
        packets::{inbound::TryIntoInboundPacket, outbound::RequestStatePacket},
        structures::{AmbientSoundMode, NoiseCancelingMode},
    },
    futures::Futures,
    soundcore_device::{
        device::packet_io_controller::PacketIOController, device_model::DeviceModel,
    },
};

use super::{packets::A3027StateUpdatePacket, state::A3027State};
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

pub struct A3027DeviceRegistry<C: ConnectionRegistry, F: Futures> {
    inner: C,
    _futures: PhantomData<F>,
}

impl<C: ConnectionRegistry, F: Futures> A3027DeviceRegistry<C, F> {
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            _futures: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<C, F> OpenSCQ30DeviceRegistry for A3027DeviceRegistry<C, F>
where
    C: ConnectionRegistry + 'static,
    F: Futures + 'static,
{
    async fn devices(&self) -> crate::Result<Vec<GenericDeviceDescriptor>> {
        self.inner
            .connection_descriptors()
            .await
            .map(|descriptors| {
                descriptors
                    .into_iter()
                    .map(|d| GenericDeviceDescriptor::new(d.name(), d.mac_address()))
                    .collect()
            })
    }

    async fn connect(
        &self,
        mac_address: macaddr::MacAddr6,
    ) -> crate::Result<Arc<dyn OpenSCQ30Device>> {
        let connection = self
            .inner
            .connection(mac_address)
            .await?
            .ok_or(crate::Error::DeviceNotFound { source: None })?;
        let device = A3027Device::<C::ConnectionType, F>::new(connection).await?;
        Ok(Arc::new(device))
    }
}

pub struct A3027Device<ConnectionType: Connection, FuturesType: Futures> {
    state_sender: watch::Sender<A3027State>,
    module_collection: Arc<ModuleCollection<A3027State>>,
    _packet_io_controller: Arc<PacketIOController<ConnectionType, FuturesType>>,
}

impl<ConnectionType, FuturesType> A3027Device<ConnectionType, FuturesType>
where
    ConnectionType: Connection + 'static,
    FuturesType: Futures + 'static,
{
    pub async fn new(connection: Arc<ConnectionType>) -> crate::Result<Self> {
        let (packet_io_controller, packet_receiver) =
            PacketIOController::<ConnectionType, FuturesType>::new(connection).await?;
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

        let module_collection = Arc::new(module_collection);
        module_collection
            .spawn_packet_handler::<FuturesType>(state_sender.clone(), packet_receiver);

        Ok(Self {
            state_sender,
            _packet_io_controller: packet_io_controller,
            module_collection,
        })
    }
}

#[async_trait(?Send)]
impl<ConnectionType, FuturesType> OpenSCQ30Device for A3027Device<ConnectionType, FuturesType>
where
    ConnectionType: Connection + 'static,
    FuturesType: Futures + 'static,
{
    async fn categories(&self) -> Vec<CategoryId> {
        self.module_collection.setting_manager.categories().to_vec()
    }

    async fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId> {
        self.module_collection.setting_manager.category(category_id)
    }

    async fn setting(&self, setting_id: &SettingId) -> crate::Result<Setting> {
        let state = self.state_sender.borrow();
        self.module_collection
            .setting_manager
            .get(&state, setting_id)
            .await
            .unwrap()
    }

    async fn set_setting_values(
        &self,
        setting_values: Vec<(SettingId<'_>, Value)>,
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
