use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        packet::{self, PacketIOController, outbound::ToPacket},
        state_modifier::StateModifier,
        structures::{AgeRange, BasicHearId, CustomHearId, EqualizerConfiguration, Gender},
    },
};

pub struct EqualizerStateModifier<
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
    options: EqualizerStateModifierOptions,
}

pub struct EqualizerStateModifierOptions {
    pub has_drc: bool,
}

impl<
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> EqualizerStateModifier<ConnectionType, CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
{
    pub fn new(
        packet_io: Arc<PacketIOController<ConnectionType>>,
        options: EqualizerStateModifierOptions,
    ) -> Self {
        Self { packet_io, options }
    }
}

#[async_trait]
impl<
    ConnectionType,
    T,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> StateModifier<T>
    for EqualizerStateModifier<
        ConnectionType,
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
where
    T: Has<EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>
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
                packet::outbound::set_equalizer_with_drc(target_equalizer_configuration)
            } else {
                packet::outbound::set_equalizer(target_equalizer_configuration)
            })
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target_equalizer_configuration);
        Ok(())
    }
}

pub struct EqualizerWithBasicHearIdStateModifier<
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>
    EqualizerWithBasicHearIdStateModifier<
        ConnectionType,
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
{
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<
    ConnectionType,
    T,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> StateModifier<T>
    for EqualizerWithBasicHearIdStateModifier<
        ConnectionType,
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
where
    T: Has<EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>
        + Has<BasicHearId<CHANNELS, BANDS>>
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
        let target_equalizer_configuration: &EqualizerConfiguration<_, _, _, _, _> =
            target_state.get();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration<_, _, _, _, _> = state.get();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        let mut target_hear_id: BasicHearId<CHANNELS, BANDS> = *target_state.get();
        // We don't expose hear id in any way, so it should be disabled to ensure the equalizer
        // configuration that we're applying is in effect
        target_hear_id.is_enabled = false;

        self.packet_io
            .send_with_response(
                &packet::outbound::SetEqualizerAndCustomHearId {
                    equalizer_configuration: target_equalizer_configuration,
                    gender: *target_state.get(),
                    age_range: *target_state.get(),
                    custom_hear_id: &{
                        // TODO have SetEqualizerAndCustomHearIdPacket take only the wanted fields rather than an entire CustomHearId struct
                        CustomHearId {
                            is_enabled: target_hear_id.is_enabled,
                            volume_adjustments: target_hear_id.volume_adjustments,
                            time: target_hear_id.time,
                            hear_id_type: Default::default(),
                            favorite_music_genre: Default::default(),
                            custom_volume_adjustments: [None; CHANNELS],
                        }
                    },
                }
                .to_packet(),
            )
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_equalizer_configuration;
            *state.get_mut() = target_hear_id;
        });
        Ok(())
    }
}

pub struct EqualizerWithCustomHearIdStateModifier<
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>
    EqualizerWithCustomHearIdStateModifier<
        ConnectionType,
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
{
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<
    ConnectionType,
    T,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> StateModifier<T>
    for EqualizerWithCustomHearIdStateModifier<
        ConnectionType,
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
where
    T: Has<EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>
        + Has<CustomHearId<CHANNELS, BANDS>>
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
        let target_equalizer_configuration: &EqualizerConfiguration<
            CHANNELS,
            BANDS,
            MIN_VOLUME,
            MAX_VOLUME,
            FRACTION_DIGITS,
        > = target_state.get();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration<
                CHANNELS,
                BANDS,
                MIN_VOLUME,
                MAX_VOLUME,
                FRACTION_DIGITS,
            > = state.get();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        let mut target_hear_id: CustomHearId<CHANNELS, BANDS> = *target_state.get();
        // We don't expose hear id in any way, so it should be disabled to ensure the equalizer
        // configuration that we're applying is in effect
        target_hear_id.is_enabled = false;

        self.packet_io
            .send_with_response(
                &packet::outbound::SetEqualizerAndCustomHearId {
                    equalizer_configuration: target_equalizer_configuration,
                    gender: *target_state.get(),
                    age_range: *target_state.get(),
                    custom_hear_id: &target_hear_id,
                }
                .to_packet(),
            )
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_equalizer_configuration;
            *state.get_mut() = target_hear_id;
        });
        Ok(())
    }
}
