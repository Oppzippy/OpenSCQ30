use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
        structures::{AgeRange, BasicHearId, CustomHearId, EqualizerConfiguration, Gender},
    },
};

pub struct EqualizerStateModifier<ConnectionType: RfcommConnection, const C: usize, const B: usize>
{
    packet_io: Arc<PacketIOController<ConnectionType>>,
    options: EqualizerStateModifierOptions,
}

pub struct EqualizerStateModifierOptions {
    pub has_drc: bool,
}

impl<ConnectionType: RfcommConnection, const C: usize, const B: usize>
    EqualizerStateModifier<ConnectionType, C, B>
{
    pub fn new(
        packet_io: Arc<PacketIOController<ConnectionType>>,
        options: EqualizerStateModifierOptions,
    ) -> Self {
        Self { packet_io, options }
    }
}

#[async_trait]
impl<ConnectionType, T, const C: usize, const B: usize> StateModifier<T>
    for EqualizerStateModifier<ConnectionType, C, B>
where
    T: Has<EqualizerConfiguration<C, B>> + Clone + Send + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_equalizer_configuration = target_state.get();
        {
            let state = state_sender.borrow();
            let equalizer_configuration = state.get();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(&if self.options.has_drc {
                packet::outbound::SetEqualizerWithDrc {
                    equalizer_configuration: target_equalizer_configuration,
                }
                .into()
            } else {
                packet::outbound::SetEqualizer {
                    equalizer_configuration: target_equalizer_configuration,
                }
                .into()
            })
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = target_equalizer_configuration.clone());
        Ok(())
    }
}

pub struct EqualizerWithBasicHearIdStateModifier<
    ConnectionType: RfcommConnection,
    const C: usize,
    const B: usize,
> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection, const C: usize, const B: usize>
    EqualizerWithBasicHearIdStateModifier<ConnectionType, C, B>
{
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T, const C: usize, const B: usize> StateModifier<T>
    for EqualizerWithBasicHearIdStateModifier<ConnectionType, C, B>
where
    T: Has<EqualizerConfiguration<C, B>>
        + Has<BasicHearId<C, B>>
        + Has<Gender>
        + Has<AgeRange>
        + Clone
        + Send
        + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_equalizer_configuration: &EqualizerConfiguration<C, B> = target_state.get();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration<C, B> = state.get();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(
                &packet::outbound::SetEqualizerAndCustomHearId {
                    equalizer_configuration: target_equalizer_configuration,
                    gender: *target_state.get(),
                    age_range: *target_state.get(),
                    custom_hear_id: &{
                        let basic_hear_id: &BasicHearId<C, B> = target_state.get();
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
        state_sender.send_modify(|state| *state.get_mut() = target_equalizer_configuration.clone());
        Ok(())
    }
}

pub struct EqualizerWithCustomHearIdStateModifier<
    ConnectionType: RfcommConnection,
    const C: usize,
    const B: usize,
> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection, const C: usize, const B: usize>
    EqualizerWithCustomHearIdStateModifier<ConnectionType, C, B>
{
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T, const C: usize, const B: usize> StateModifier<T>
    for EqualizerWithCustomHearIdStateModifier<ConnectionType, C, B>
where
    T: Has<EqualizerConfiguration<C, B>>
        + Has<CustomHearId<C, B>>
        + Has<Gender>
        + Has<AgeRange>
        + Clone
        + Send
        + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_equalizer_configuration: &EqualizerConfiguration<C, B> = target_state.get();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration<C, B> = state.get();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(
                &packet::outbound::SetEqualizerAndCustomHearId {
                    equalizer_configuration: target_equalizer_configuration,
                    gender: *target_state.get(),
                    age_range: *target_state.get(),
                    custom_hear_id: target_state.get(),
                }
                .into(),
            )
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = target_equalizer_configuration.clone());
        Ok(())
    }
}
