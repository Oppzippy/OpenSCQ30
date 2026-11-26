use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3957::structures::PressureSensitivity,
        common::{
            packet::{self, PacketIOController},
            state_modifier::StateModifier,
        },
    },
};

// [4, 0x85] verified on a Liberty 5.
const COMMAND: packet::Command = packet::Command([0x04, 0x85]);

pub struct PressureSensitivityStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl PressureSensitivityStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for PressureSensitivityStateModifier
where
    T: Has<PressureSensitivity> + Clone + Send + Sync,
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
            .send_with_response(&packet::Outbound::new(COMMAND, vec![target as u8]))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = target;
        });
        Ok(())
    }
}
