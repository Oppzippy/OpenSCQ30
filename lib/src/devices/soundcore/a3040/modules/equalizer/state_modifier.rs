use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3040,
        common::{
            packet::PacketIOController,
            state_modifier::StateModifier,
            structures::{CustomHearId, EqualizerConfiguration},
        },
    },
};

pub struct EqualizerWithCustomHearIdStateModifier<
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const HEAR_ID_CHANNELS: usize,
    const HEAR_ID_BANDS: usize,
> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const HEAR_ID_CHANNELS: usize,
    const HEAR_ID_BANDS: usize,
>
    EqualizerWithCustomHearIdStateModifier<
        ConnectionType,
        CHANNELS,
        BANDS,
        HEAR_ID_CHANNELS,
        HEAR_ID_BANDS,
    >
{
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<
    T,
    ConnectionType: RfcommConnection,
    const CHANNELS: usize,
    const BANDS: usize,
    const HEAR_ID_CHANNELS: usize,
    const HEAR_ID_BANDS: usize,
> StateModifier<T>
    for EqualizerWithCustomHearIdStateModifier<
        ConnectionType,
        CHANNELS,
        BANDS,
        HEAR_ID_CHANNELS,
        HEAR_ID_BANDS,
    >
where
    T: Has<EqualizerConfiguration<CHANNELS, BANDS>>
        + Has<CustomHearId<HEAR_ID_CHANNELS, HEAR_ID_BANDS>>
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
        let target_equalizer_configuration: &EqualizerConfiguration<CHANNELS, BANDS> =
            target_state.get();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &EqualizerConfiguration<CHANNELS, BANDS> = state.get();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        let mut target_hear_id: CustomHearId<HEAR_ID_CHANNELS, HEAR_ID_BANDS> = *target_state.get();
        // We don't expose hear id in any way, so it should be disabled to ensure the equalizer
        // configuration that we're applying is in effect
        target_hear_id.is_enabled = false;

        self.packet_io
            .send_with_response(&a3040::packets::set_equalizer(
                target_equalizer_configuration,
                &target_hear_id,
            ))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_equalizer_configuration;
            *state.get_mut() = target_hear_id;
        });
        Ok(())
    }
}
