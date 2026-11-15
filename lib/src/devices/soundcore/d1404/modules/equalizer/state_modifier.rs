use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{packet::PacketIOController, state_modifier::StateModifier},
        d1404,
    },
};

pub struct EqualizerStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl EqualizerStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for EqualizerStateModifier
where
    T: Has<d1404::structures::D1404EqualizerConfiguration> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_equalizer_configuration: &d1404::structures::D1404EqualizerConfiguration =
            target_state.get();
        {
            let state = state_sender.borrow();
            let equalizer_configuration: &d1404::structures::D1404EqualizerConfiguration =
                state.get();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(&d1404::packets::outbound::set_equalizer(
                target_equalizer_configuration,
            ))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_equalizer_configuration;
        });
        Ok(())
    }
}
