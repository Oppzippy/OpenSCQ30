use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::standard::{
        packet::{
            outbound::SetAmbientSoundModeCyclePacket, packet_io_controller::PacketIOController,
        },
        state_modifier::StateModifier,
        structures::AmbientSoundModeCycle,
    },
};

pub struct AmbientSoundModeCycleStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> AmbientSoundModeCycleStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for AmbientSoundModeCycleStateModifier<ConnectionType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    T: Has<AmbientSoundModeCycle> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_cycle = target_state.get();
        {
            let state = state_sender.borrow();
            let cycle = state.get();
            if cycle == target_cycle {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(
                &SetAmbientSoundModeCyclePacket {
                    cycle: *target_cycle,
                }
                .into(),
            )
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target_cycle);
        Ok(())
    }
}
