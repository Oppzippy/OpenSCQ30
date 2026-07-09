use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    device,
    devices::soundcore::{
        a3876,
        common::{
            packet::PacketIOController, state_modifier::StateModifier,
            structures::EqualizerConfiguration,
        },
    },
};

pub struct EqualizerStateModifier<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    packet_io: Arc<PacketIOController>,
}

impl<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> EqualizerStateModifier<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
{
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
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
    for EqualizerStateModifier<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
where
    T: Has<EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>
        + Clone
        + Send
        + Sync,
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
            .send_with_response(&a3876::packets::outbound::set_equalizer_with_drc(
                target_equalizer_configuration,
            ))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target_equalizer_configuration);
        Ok(())
    }
}
