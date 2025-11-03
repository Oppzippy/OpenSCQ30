use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
        structures::LimitHighVolume,
    },
};

pub struct LimitHighVolumeStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> LimitHighVolumeStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for LimitHighVolumeStateModifier<ConnectionType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    T: Has<LimitHighVolume> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = target_state.get();
        let current = *state_sender.borrow().get();

        if current.enabled != target.enabled || current.db_limit != target.db_limit {
            self.packet_io
                .send_with_response(&packet::outbound::set_limit_high_volume(
                    target.enabled,
                    target.db_limit,
                ))
                .await?;
            state_sender.send_modify(|state| {
                state.get_mut().enabled = target.enabled;
                state.get_mut().db_limit = target.db_limit;
            });
        }

        if current.refresh_rate != target.refresh_rate {
            self.packet_io
                .send_with_response(&packet::outbound::set_limit_high_volume_refresh_rate(
                    target.refresh_rate,
                ))
                .await?;
            state_sender.send_modify(|state| {
                state.get_mut().refresh_rate = target.refresh_rate;
            });
        }

        Ok(())
    }
}
