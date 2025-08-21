use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3028::packets::{AutoPowerOff, SetAutoPowerOffPacket},
        standard::{
            packet::packet_io_controller::PacketIOController, state_modifier::StateModifier,
        },
    },
};

pub struct AutoPowerOffStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> AutoPowerOffStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for AutoPowerOffStateModifier<ConnectionType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    T: AsMut<Option<AutoPowerOff>> + AsRef<Option<AutoPowerOff>> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let current = state.as_ref();
            if current == target {
                return Ok(());
            }
        }

        if let Some(target) = target {
            self.packet_io
                .send_with_response(&SetAutoPowerOffPacket(*target).into())
                .await?;
        }
        state_sender.send_modify(|state| *state.as_mut() = *target);
        Ok(())
    }
}
