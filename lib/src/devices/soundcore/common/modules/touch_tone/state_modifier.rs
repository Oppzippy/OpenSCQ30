use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        packet::{self, packet_io_controller::PacketIOController},
        state_modifier::StateModifier,
        structures::TouchTone,
    },
};

pub struct TouchToneStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> TouchToneStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for TouchToneStateModifier<ConnectionType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    T: Has<TouchTone> + Clone + Send + Sync,
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
            .send_with_response(&packet::outbound::SetTouchTone(*target).into())
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target);
        Ok(())
    }
}
