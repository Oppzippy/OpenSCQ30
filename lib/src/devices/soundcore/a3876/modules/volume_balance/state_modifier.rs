use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3876::{self, structures::VolumeBalance},
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct VolumeBalanceStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl VolumeBalanceStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for VolumeBalanceStateModifier
where
    T: Has<VolumeBalance> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = target_state.get();
        {
            let state = state_sender.borrow();
            let current = state.get();
            if current == target {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(&a3876::packets::outbound::set_volume_balance(*target))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target);
        Ok(())
    }
}
