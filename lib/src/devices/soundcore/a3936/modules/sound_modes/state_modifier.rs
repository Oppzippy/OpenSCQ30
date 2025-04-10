use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    api::connection::RfcommConnection,
    devices::soundcore::{
        a3936::{packets::A3936SetSoundModesPacket, structures::A3936SoundModes},
        standard::{
            packets::packet_io_controller::PacketIOController, state_modifier::StateModifier,
        },
    },
};

pub struct SoundModesStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> SoundModesStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for SoundModesStateModifier<ConnectionType>
where
    T: AsMut<A3936SoundModes> + AsRef<A3936SoundModes> + Clone + Send + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let sound_modes = *state_sender.borrow().as_ref();
        let target_sound_modes = target_state.as_ref();
        if &sound_modes == target_sound_modes {
            return Ok(());
        }

        self.packet_io
            .send(&A3936SetSoundModesPacket { sound_modes }.into())
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = *target_sound_modes);

        Ok(())
    }
}
