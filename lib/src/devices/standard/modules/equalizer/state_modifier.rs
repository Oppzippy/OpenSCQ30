use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::connection::Connection,
    devices::standard::{
        packets::outbound::SetEqualizerPacket, state_modifier::StateModifier,
        structures::EqualizerConfiguration,
    },
    soundcore_device::device::packet_io_controller::PacketIOController,
};

pub struct EqualizerStateModifier<ConnectionType: Connection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
    is_stereo: bool,
}

impl<ConnectionType: Connection> EqualizerStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>, is_stereo: bool) -> Self {
        Self {
            packet_io,
            is_stereo,
        }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for EqualizerStateModifier<ConnectionType>
where
    T: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration> + Clone + Send + Sync,
    ConnectionType: Connection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let target_equalizer_configuration = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let equalizer_configuration = state.as_ref();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
        }

        self.packet_io
            .send(
                &SetEqualizerPacket::new(
                    target_equalizer_configuration,
                    self.is_stereo.then_some(target_equalizer_configuration),
                )
                .into(),
            )
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = target_equalizer_configuration.clone());
        Ok(())
    }
}
