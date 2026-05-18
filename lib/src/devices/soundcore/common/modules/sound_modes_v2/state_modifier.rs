use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        modules::sound_modes_v2::ToPacketBody,
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
    },
};

pub struct SoundModesStateModifier<SoundModesT> {
    _sound_modes: PhantomData<SoundModesT>,
    packet_io: Arc<PacketIOController>,
}

impl<SoundModesT> SoundModesStateModifier<SoundModesT> {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self {
            packet_io,
            _sound_modes: PhantomData,
        }
    }
}

#[async_trait]
impl<StateT, SoundModesT> StateModifier<StateT> for SoundModesStateModifier<SoundModesT>
where
    StateT: Has<SoundModesT> + Send + Sync,
    SoundModesT: ToPacketBody + PartialEq + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        let target_sound_modes = target_state.get();
        if state_sender.borrow().get() == target_sound_modes {
            return Ok(());
        }
        self.packet_io
            .send_with_response(&packet::Outbound::new(
                packet::Command([0x06, 0x81]),
                target_sound_modes.bytes(),
            ))
            .await?;
        state_sender.send_modify(|v| {
            *v.get_mut() = target_sound_modes.clone();
        });
        Ok(())
    }
}
