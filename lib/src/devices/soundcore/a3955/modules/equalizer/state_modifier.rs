use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3955,
        common::{self, packet::PacketIOController, state_modifier::StateModifier},
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
        + Has<a3955::structures::IsHearIdInitialized>
        + Clone
        + Send
        + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_eq: &common::structures::EqualizerConfiguration<_, _, _, _, _> =
            target_state.get();
        {
            let state = state_sender.borrow();
            let current_eq: &common::structures::EqualizerConfiguration<_, _, _, _, _> =
                state.get();
            if current_eq == target_eq {
                return Ok(());
            }
        }

        let mut target_hear_id: common::structures::CustomHearId<_, _> = *target_state.get();
        target_hear_id.is_enabled = false;

        let is_hear_id_initialized: a3955::structures::IsHearIdInitialized = *target_state.get();

        self.packet_io
            .send_with_response(&a3955::packets::outbound::set_equalizer_configuration(
                target_eq,
                &target_hear_id,
                is_hear_id_initialized.0,
            ))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_eq;
            *state.get_mut() = target_hear_id;
        });
        Ok(())
    }
}
