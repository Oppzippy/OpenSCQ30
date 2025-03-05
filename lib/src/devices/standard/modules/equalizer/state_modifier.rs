use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::connection::Connection,
    devices::standard::{
        packets::outbound::{
            SetEqualizerAndCustomHearIdPacket, SetEqualizerPacket, SetEqualizerWithDrcPacket,
        },
        state_modifier::StateModifier,
        structures::{AgeRange, BasicHearId, CustomHearId, EqualizerConfiguration, Gender},
    },
    soundcore_device::device::packet_io_controller::PacketIOController,
};

pub struct EqualizerStateModifier<ConnectionType: Connection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
    options: EqualizerStateModifierOptions,
}

pub struct EqualizerStateModifierOptions {
    pub is_stereo: bool,
    pub has_drc: bool,
}

impl<ConnectionType: Connection> EqualizerStateModifier<ConnectionType> {
    pub fn new(
        packet_io: Arc<PacketIOController<ConnectionType>>,
        options: EqualizerStateModifierOptions,
    ) -> Self {
        Self { packet_io, options }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for EqualizerStateModifier<ConnectionType>
where
    T: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration> + Clone + Send + Sync,
    ConnectionType: Connection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let target_equalizer_configuration = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let equalizer_configuration = state.as_ref();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        let right_side = self
            .options
            .is_stereo
            .then_some(target_equalizer_configuration);
        self.packet_io
            .send(&if self.options.has_drc {
                SetEqualizerWithDrcPacket::new(target_equalizer_configuration, right_side).into()
            } else {
                SetEqualizerPacket::new(target_equalizer_configuration, right_side).into()
            })
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = target_equalizer_configuration.clone());
        Ok(())
    }
}

pub struct EqualizerWithBasicHearIdStateModifier<ConnectionType: Connection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: Connection> EqualizerWithBasicHearIdStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for EqualizerWithBasicHearIdStateModifier<ConnectionType>
where
    T: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration> + Clone + Send + Sync,
    T: AsRef<BasicHearId> + AsRef<Gender> + AsRef<AgeRange>,
    ConnectionType: Connection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let target_equalizer_configuration: &EqualizerConfiguration = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration = state.as_ref();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        self.packet_io
            .send(
                &SetEqualizerAndCustomHearIdPacket {
                    equalizer_configuration: target_equalizer_configuration,
                    gender: *target_state.as_ref(),
                    age_range: *target_state.as_ref(),
                    custom_hear_id: &{
                        let basic_hear_id: &BasicHearId = target_state.as_ref();
                        // TODO have SetEqualizerAndCustomHearIdPacket take only the wanted fields rather than an entire CustomHearId struct
                        CustomHearId {
                            is_enabled: basic_hear_id.is_enabled,
                            volume_adjustments: basic_hear_id.volume_adjustments.to_owned(),
                            time: basic_hear_id.time,
                            hear_id_type: Default::default(),
                            hear_id_music_type: Default::default(),
                            custom_volume_adjustments: None,
                        }
                    },
                }
                .into(),
            )
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = target_equalizer_configuration.clone());
        Ok(())
    }
}

pub struct EqualizerWithCustomHearIdStateModifier<ConnectionType: Connection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: Connection> EqualizerWithCustomHearIdStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for EqualizerWithCustomHearIdStateModifier<ConnectionType>
where
    T: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration> + Clone + Send + Sync,
    T: AsRef<CustomHearId> + AsRef<Gender> + AsRef<AgeRange>,
    ConnectionType: Connection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let target_equalizer_configuration: &EqualizerConfiguration = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration = state.as_ref();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        self.packet_io
            .send(
                &SetEqualizerAndCustomHearIdPacket {
                    equalizer_configuration: target_equalizer_configuration,
                    gender: *target_state.as_ref(),
                    age_range: *target_state.as_ref(),
                    custom_hear_id: target_state.as_ref(),
                }
                .into(),
            )
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = target_equalizer_configuration.clone());
        Ok(())
    }
}
