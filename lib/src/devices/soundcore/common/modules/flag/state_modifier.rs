use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
    },
};

pub struct FlagStateModifier<ConnectionType: RfcommConnection, Flag> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
    command: packet::Command,
    get_flag: fn(&Flag) -> bool,
}

impl<ConnectionType: RfcommConnection, Flag> FlagStateModifier<ConnectionType, Flag> {
    pub fn new(
        packet_io: Arc<PacketIOController<ConnectionType>>,
        command: packet::Command,
        get_flag: fn(&Flag) -> bool,
    ) -> Self {
        Self {
            packet_io,
            command,
            get_flag,
        }
    }
}

#[async_trait]
impl<ConnectionType, Flag, T> StateModifier<T> for FlagStateModifier<ConnectionType, Flag>
where
    ConnectionType: RfcommConnection + Send + Sync,
    T: Has<Flag> + Send + Sync,
    Flag: PartialEq + Copy + Send + Sync,
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
            .send_with_response(&packet::Outbound::new(
                self.command,
                vec![(self.get_flag)(target).into()],
            ))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target);
        Ok(())
    }
}
