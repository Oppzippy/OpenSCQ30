use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    connection::RfcommConnection,
    device,
    devices::soundcore::{
        a3947,
        common::{
            packet::PacketIOController, state_modifier::StateModifier,
            structures::EqualizerConfiguration,
        },
    },
};

pub struct EqualizerStateModifier<ConnectionType: RfcommConnection, const C: usize, const B: usize>
{
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection, const C: usize, const B: usize>
    EqualizerStateModifier<ConnectionType, C, B>
{
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T, const C: usize, const B: usize> StateModifier<T>
    for EqualizerStateModifier<ConnectionType, C, B>
where
    T: Has<EqualizerConfiguration<C, B>>
        + Has<a3947::structures::HearId<C, B>>
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
            .send_with_response(&a3947::packets::set_equalizer_configuration(
                target_equalizer_configuration,
                target_state.get(),
            ))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = target_equalizer_configuration.clone());
        Ok(())
    }
}
