use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3116,
        common::{
            packet::PacketIOController, state_modifier::StateModifier,
            structures::EqualizerConfiguration,
        },
    },
};

pub struct EqualizerStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> EqualizerStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for EqualizerStateModifier<ConnectionType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    T: Has<EqualizerConfiguration<1, 9, -6, 6, 0>> + Clone + Send + Sync + 'static,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = *target_state.get();
        let state = *state_sender.borrow().get();

        if target == state {
            return Ok(());
        }

        if target.preset_id() != state.preset_id() {
            self.packet_io
                .send_with_response(&a3116::packets::outbound::set_equalizer_preset(
                    target.preset_id() as u8,
                ))
                .await?;
        }
        if target.volume_adjustments() != state.volume_adjustments() {
            self.packet_io
                .send_with_response(&a3116::packets::outbound::set_equalizer_volume_adjustments(
                    target.volume_adjustments()[0],
                ))
                .await?;
        }

        state_sender.send_modify(|state| *state.get_mut() = target);
        Ok(())
    }
}
