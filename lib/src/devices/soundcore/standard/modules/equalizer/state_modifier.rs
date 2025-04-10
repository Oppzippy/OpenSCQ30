use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::connection::RfcommConnection,
    devices::soundcore::standard::{
        packets::{
            outbound::{
                SetEqualizerAndCustomHearIdPacket, SetEqualizerWithDrcPacket, set_equalizer,
            },
            packet_io_controller::PacketIOController,
        },
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
    T: AsMut<EqualizerConfiguration<C, B>>
        + AsRef<EqualizerConfiguration<C, B>>
        + Clone
        + Send
        + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
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

        self.packet_io
            .send(&if self.options.has_drc {
                SetEqualizerWithDrcPacket {
                    equalizer_configuration: target_equalizer_configuration,
                }
                .into()
            } else {
                set_equalizer::SetEqualizerPacket {
                    equalizer_configuration: target_equalizer_configuration,
                }
                .into()
            })
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = target_equalizer_configuration.clone());
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
    T: AsMut<EqualizerConfiguration<C, B>>
        + AsRef<EqualizerConfiguration<C, B>>
        + Clone
        + Send
        + Sync,
    T: AsRef<BasicHearId<C, B>> + AsRef<Gender> + AsRef<AgeRange>,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let target_equalizer_configuration: &EqualizerConfiguration<C, B> = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration<C, B> = state.as_ref();
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
                        let basic_hear_id: &BasicHearId<C, B> = target_state.as_ref();
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
    T: AsMut<EqualizerConfiguration<C, B>>
        + AsRef<EqualizerConfiguration<C, B>>
        + Clone
        + Send
        + Sync,
    T: AsRef<CustomHearId<C, B>> + AsRef<Gender> + AsRef<AgeRange>,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let target_equalizer_configuration: &EqualizerConfiguration<C, B> = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration<C, B> = state.as_ref();
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
