use async_trait::async_trait;
use openscq30_lib_has::MaybeHas;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3116,
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct VolumeStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl VolumeStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for VolumeStateModifier
where
    T: MaybeHas<a3116::structures::Volume> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = target_state.maybe_get();
        {
            let state = state_sender.borrow();
            let current = state.maybe_get();
            if current == target {
                return Ok(());
            }
        }

        if let Some(target) = target {
            self.packet_io
                .send_with_response(&a3116::packets::outbound::set_volume(target))
                .await?;
        }
        state_sender.send_modify(|state| state.set_maybe(target.copied()));
        Ok(())
    }
}
