use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3953::structures::SpatialAudio,
        common::{
            packet::{self, PacketIOController},
            state_modifier::StateModifier,
        },
    },
};

const COMMAND: packet::Command = packet::Command([0x10, 0x81]);

pub struct SpatialAudioStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl SpatialAudioStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for SpatialAudioStateModifier
where
    T: Has<SpatialAudio> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: SpatialAudio = *target_state.get();
        {
            let state = state_sender.borrow();
            let current: &SpatialAudio = state.get();
            if *current == target {
                return Ok(());
            }
        }
        self.packet_io
            .send_with_response(&packet::Outbound::new(COMMAND, target.bytes().to_vec()))
            .await?;
        state_sender.send_modify(|state| {
            let value: &mut SpatialAudio = state.get_mut();
            *value = target;
        });
        Ok(())
    }
}
