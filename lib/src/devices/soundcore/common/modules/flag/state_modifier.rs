use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
        structures::Flag,
    },
};

pub struct FlagStateModifier<ConnectionType: RfcommConnection, FlagT> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
    command: packet::Command,
    _flag: PhantomData<FlagT>,
}

impl<ConnectionType: RfcommConnection, FlagT> FlagStateModifier<ConnectionType, FlagT> {
    pub fn new(
        packet_io: Arc<PacketIOController<ConnectionType>>,
        command: packet::Command,
    ) -> Self {
        Self {
            packet_io,
            command,
            _flag: PhantomData,
        }
    }
}

#[async_trait]
impl<ConnectionT, FlagT, StateT> StateModifier<StateT> for FlagStateModifier<ConnectionT, FlagT>
where
    ConnectionT: RfcommConnection + Send + Sync,
    StateT: Has<FlagT> + Send + Sync,
    FlagT: Flag + PartialEq + Copy + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
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
                vec![target.get_bool().into()],
            ))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target);
        Ok(())
    }
}
