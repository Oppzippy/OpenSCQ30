use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use nom::error::VerboseError;
use tokio::sync::watch;

use crate::{
    api::{
        connection::{RfcommBackend, RfcommConnection},
        device::OpenSCQ30Device,
        settings::{CategoryId, Setting, SettingId, Value},
    },
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        self,
        demo::DemoConnectionRegistry,
        device::{SoundcoreDevice, SoundcoreDeviceRegistry},
        implementation::ButtonConfigurationImplementation,
        modules::{
            ModuleCollection, ModuleCollectionSpawnPacketHandlerExt,
            sound_modes::AvailableSoundModes,
        },
        packets::{
            inbound::{
                InboundPacket, TryIntoInboundPacket, state_update_packet::StateUpdatePacket,
            },
            outbound::{OutboundPacketBytesExt, RequestStatePacket},
        },
        state::DeviceState,
        structures::{
            AmbientSoundMode, AmbientSoundModeCycle, Command, EqualizerConfiguration, HearId,
            MultiButtonConfiguration, NoiseCancelingMode, STATE_UPDATE, SoundModes,
            SoundModesTypeTwo,
        },
    },
    soundcore_device::{
        device::{
            device_implementation::DeviceImplementation, packet_io_controller::PacketIOController,
            soundcore_command::CommandResponse,
        },
        device_model::DeviceModel,
    },
    storage::OpenSCQ30Database,
};

use super::{packets::A3031StateUpdatePacket, state::A3031State};

pub(crate) const A3031_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
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
        has_button_configuration: true,
        has_wear_detection: false,
        has_touch_tone: true,
        has_auto_power_off: true,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::SoundcoreA3031],
    implementation: || Arc::new(A3031Implementation::default()),
};

#[derive(Debug, Default)]
struct A3031Implementation {
    buttons: Arc<ButtonConfigurationImplementation>,
}

impl DeviceImplementation for A3031Implementation {
    fn packet_handlers(
        &self,
    ) -> HashMap<Command, Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        let mut handlers = standard::implementation::packet_handlers();
        let buttons = self.buttons.to_owned();

        handlers.insert(
            STATE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let packet = match A3031StateUpdatePacket::take::<VerboseError<_>>(packet_bytes) {
                    Ok((_, packet)) => packet,
                    Err(err) => {
                        tracing::error!("failed to parse packet: {err:?}");
                        return state;
                    }
                };
                buttons.set_internal_data(packet.button_configuration);

                StateUpdatePacket::from(packet).into()
            }),
        );

        handlers
    }

    fn initialize(&self, packet: &[u8]) -> crate::Result<DeviceState> {
        let packet = A3031StateUpdatePacket::take::<VerboseError<_>>(packet)
            .map(|(_, packet)| packet)
            .map_err(|err| crate::Error::ParseError {
                message: format!("{err:?}"),
            })?;
        Ok(StateUpdatePacket::from(packet).into())
    }

    fn set_equalizer_configuration(
        &self,
        state: DeviceState,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_equalizer_configuration(state, equalizer_configuration)
    }

    fn set_sound_modes(
        &self,
        state: DeviceState,
        sound_modes: SoundModes,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_sound_modes(state, sound_modes)
    }

    fn set_sound_modes_type_two(
        &self,
        state: DeviceState,
        sound_modes: SoundModesTypeTwo,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_sound_modes_type_two(state, sound_modes)
    }

    fn set_hear_id(&self, state: DeviceState, hear_id: HearId) -> crate::Result<CommandResponse> {
        standard::implementation::set_hear_id(state, hear_id)
    }

    fn set_multi_button_configuration(
        &self,
        state: DeviceState,
        button_configuration: MultiButtonConfiguration,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_multi_button_configuration(
            state,
            &self.buttons,
            button_configuration,
        )
    }

    fn set_ambient_sound_mode_cycle(
        &self,
        state: DeviceState,
        cycle: AmbientSoundModeCycle,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_ambient_sound_mode_cycle(state, cycle)
    }
}

pub fn device_registry<B: RfcommBackend>(
    backend: B,
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDeviceRegistry<B, A3031Device<B::ConnectionType>> {
    SoundcoreDeviceRegistry::new(backend, database, device_model)
}

pub fn demo_device_registry(
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDeviceRegistry<
    DemoConnectionRegistry,
    A3031Device<<DemoConnectionRegistry as RfcommBackend>::ConnectionType>,
> {
    SoundcoreDeviceRegistry::new(
        DemoConnectionRegistry::new(
            device_model.to_string(),
            A3031StateUpdatePacket::default().bytes(),
        ),
        database,
        device_model,
    )
}

pub struct A3031Device<ConnectionType: RfcommConnection + Send + Sync> {
    device_model: DeviceModel,
    state_sender: watch::Sender<A3031State>,
    module_collection: Arc<ModuleCollection<A3031State>>,
    _packet_io_controller: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType> SoundcoreDevice<ConnectionType> for A3031Device<ConnectionType>
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
        let state_update_packet: A3031StateUpdatePacket = packet_io_controller
            .send(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let (state_sender, _) = watch::channel::<A3031State>(state_update_packet.into());

        let mut module_collection = ModuleCollection::<A3031State>::default();
        module_collection.add_state_update();
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
            .add_equalizer(packet_io_controller.clone(), database, device_model, true)
            .await;
        module_collection.add_button_configuration(packet_io_controller.clone());

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
impl<ConnectionType> OpenSCQ30Device for A3031Device<ConnectionType>
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
