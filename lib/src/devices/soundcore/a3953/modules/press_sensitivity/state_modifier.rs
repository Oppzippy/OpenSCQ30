use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3953::structures::PressSensitivity,
        common::{
            packet::{self, PacketIOController},
            state_modifier::StateModifier,
        },
    },
};

const COMMAND: packet::Command = packet::Command([0x04, 0x85]);

pub struct PressSensitivityStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl PressSensitivityStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for PressSensitivityStateModifier
where
    T: Has<PressSensitivity> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = *target_state.get();
        {
            let state = state_sender.borrow();
            if *state.get() == target {
                return Ok(());
            }
        }
        self.packet_io
            .send_with_response(&packet::Outbound::new(COMMAND, target.bytes().to_vec()))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = target;
        });
        Ok(())
    }
}
