use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3954,
        common::{
            self,
            packet::{PacketIOController, outbound::ToPacket},
            state_modifier::StateModifier,
        },
    },
};

pub struct EqualizerStateModifier<
    T,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    _state: PhantomData<T>,
    packet_io: Arc<PacketIOController>,
}

impl<
    T,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> EqualizerStateModifier<T, CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
{
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self {
            packet_io,
            _state: PhantomData,
        }
    }
}

#[async_trait]
impl<
    T,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> StateModifier<T>
    for EqualizerStateModifier<T, CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
where
    T: Has<
            common::structures::EqualizerConfiguration<
                CHANNELS,
                BANDS,
                MIN_VOLUME,
                MAX_VOLUME,
                FRACTION_DIGITS,
            >,
        > + Has<common::structures::CustomHearId<CHANNELS, BANDS>>
        + Has<a3954::structures::SpatialAudio>
        + Clone
        + Send
        + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let equalizer_changed = self.handle_equalizer(state_sender, target_state).await?;
        if !equalizer_changed {
            self.handle_spatial_audio(state_sender, target_state)
                .await?;
        }
        Ok(())
    }
}

impl<
    T,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> EqualizerStateModifier<T, CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
where
    T: Has<
            common::structures::EqualizerConfiguration<
                CHANNELS,
                BANDS,
                MIN_VOLUME,
                MAX_VOLUME,
                FRACTION_DIGITS,
            >,
        > + Has<common::structures::CustomHearId<CHANNELS, BANDS>>
        + Has<a3954::structures::SpatialAudio>
        + Clone
        + Send
        + Sync,
{
    /// Returns true if the equalizer changed
    async fn handle_equalizer(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<bool> {
        let target_eq: &common::structures::EqualizerConfiguration<_, _, _, _, _> =
            target_state.get();
        {
            let state = state_sender.borrow();
            let current_eq: &common::structures::EqualizerConfiguration<_, _, _, _, _> =
                state.get();
            if current_eq == target_eq {
                return Ok(false);
            }
        }

        let mut target_hear_id: common::structures::CustomHearId<_, _> = *target_state.get();
        target_hear_id.is_enabled = false;

        self.packet_io
            .send_with_response(&a3954::packets::outbound::set_equalizer_configuration(
                target_eq,
                &target_hear_id,
            ))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_eq;
            *state.get_mut() = target_hear_id;
            let spatial_audio: &mut a3954::structures::SpatialAudio = state.get_mut();
            spatial_audio.is_enabled = false;
        });
        Ok(true)
    }

    async fn handle_spatial_audio(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = target_state.get();
        let current: a3954::structures::SpatialAudio = {
            let state = state_sender.borrow();
            *state.get()
        };
        if current == *target {
            return Ok(());
        }

        if target.is_enabled {
            self.packet_io
                .send_with_response(&a3954::packets::outbound::set_spatial_audio(target))
                .await?;
        } else {
            // The soundcore app sends a set eq packet to disable spatial audio rather than sending spatial audio with
            // is_enabled set to false, so we will do the same. First, we need to apply changes to any spatial audio fields
            // other than is_enabled.
            if current.mode != target.mode || current.music_mode != target.music_mode {
                self.packet_io
                    .send_with_response(&a3954::packets::outbound::set_spatial_audio(
                        &a3954::structures::SpatialAudio {
                            is_enabled: true,
                            mode: target.mode,
                            music_mode: target.music_mode,
                        },
                    ))
                    .await?;
            }
            self.packet_io
                .send_with_response(
                    &common::packet::outbound::SetEqualizerAndCustomHearId {
                        equalizer_configuration: target_state.get(),
                        gender: common::structures::Gender::default(),
                        age_range: common::structures::AgeRange::default(),
                        custom_hear_id: target_state.get(),
                        force_supports_hear_id: true,
                    }
                    .to_packet(),
                )
                .await?;
        }
        state_sender.send_modify(|state| *state.get_mut() = *target);
        Ok(())
    }
}
