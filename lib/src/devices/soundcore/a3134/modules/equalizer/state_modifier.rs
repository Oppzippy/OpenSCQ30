use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3134::{self, structures::A3134EqualizerConfiguration},
        common::{packet::PacketIOController, state_modifier::StateModifier},
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
    T: Has<A3134EqualizerConfiguration> + Clone + Send + Sync + 'static,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = *target_state.get();
        if target == *state_sender.borrow().get() {
            return Ok(());
        }

        self.packet_io
            .send_with_response(&a3134::packets::outbound::set_equalizer(&target))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = target);
        Ok(())
    }
}
