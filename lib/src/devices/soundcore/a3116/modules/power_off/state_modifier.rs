use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3116,
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct PowerOffStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> PowerOffStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for PowerOffStateModifier<ConnectionType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    T: Has<a3116::structures::PowerOffPending> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        if *target_state.get() == a3116::structures::PowerOffPending(true) {
            self.packet_io
                .send_with_response(&a3116::packets::outbound::power_off())
                .await?;
            state_sender.send_modify(|state| {
                *state.get_mut() = a3116::structures::PowerOffPending::default();
            });
        }

        Ok(())
    }
}
