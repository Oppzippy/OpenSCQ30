use async_trait::async_trait;
use openscq30_lib_has::MaybeHas;
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
        structures::Flag,
    },
};

pub struct FlagStateModifier<FlagT> {
    packet_io: Arc<PacketIOController>,
    command: packet::Command,
    _flag: PhantomData<FlagT>,
}

impl<FlagT> FlagStateModifier<FlagT> {
    pub fn new(packet_io: Arc<PacketIOController>, command: packet::Command) -> Self {
        Self {
            packet_io,
            command,
            _flag: PhantomData,
        }
    }
}

#[async_trait]
impl<FlagT, StateT> StateModifier<StateT> for FlagStateModifier<FlagT>
where
    StateT: MaybeHas<FlagT> + Send + Sync,
    FlagT: Flag + PartialEq + Copy + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        let Some(target) = target_state.maybe_get() else {
            return Ok(());
        };
        {
            let state = state_sender.borrow();
            let current = state.maybe_get();
            if current == Some(target) {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(&packet::Outbound::new(
                self.command,
                vec![target.get_bool().into()],
            ))
            .await?;
        state_sender.send_modify(|state| {
            state.set_maybe(Some(*target));
        });
        Ok(())
    }
}
