use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::connection::Connection,
    devices::standard::{
        packets::outbound::SetAmbientSoundModeCyclePacket, state_modifier::StateModifier,
        structures::AmbientSoundModeCycle,
    },
    soundcore_device::device::packet_io_controller::PacketIOController,
};

pub struct AmbientSoundModeCycleStateModifier<ConnectionType: Connection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: Connection> AmbientSoundModeCycleStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for AmbientSoundModeCycleStateModifier<ConnectionType>
where
    ConnectionType: Connection + Send + Sync,
    T: AsMut<AmbientSoundModeCycle> + AsRef<AmbientSoundModeCycle> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let target_cycle = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let cycle = state.as_ref();
            if cycle == target_cycle {
                return Ok(());
            }
        }

        self.packet_io
            .send(
                &SetAmbientSoundModeCyclePacket {
                    cycle: *target_cycle,
                }
                .into(),
            )
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = *target_cycle);
        Ok(())
    }
}
