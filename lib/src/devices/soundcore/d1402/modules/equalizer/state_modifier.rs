use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{
            packet::PacketIOController,
            state_modifier::StateModifier,
            structures::{CommonEqualizerConfiguration, CustomHearId},
        },
        d1402,
    },
};

pub struct EqualizerStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl EqualizerStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for EqualizerStateModifier
where
    T: Has<CommonEqualizerConfiguration<1, 10>>
        + Has<CustomHearId<1, 10>>
        + Has<d1402::structures::EqualizerUnknownBytes>
        + Clone
        + Send
        + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_equalizer_configuration: &CommonEqualizerConfiguration<1, 10> =
            target_state.get();
        let equalizer_unknown_bytes = {
            let state = state_sender.borrow();
            let equalizer_configuration: &CommonEqualizerConfiguration<1, 10> = state.get();
            if equalizer_configuration == target_equalizer_configuration {
                return Ok(());
            }
            // use the device's current value rather than the target's, since the target isn't
            // something that can be changed
            *Has::<d1402::structures::EqualizerUnknownBytes>::get(&*state)
        };

        let mut target_hear_id: CustomHearId<1, 10> = *target_state.get();
        // We don't expose hear id in any way, so it should be disabled to ensure the equalizer
        // configuration that we're applying is in effect
        target_hear_id.is_enabled = false;

        self.packet_io
            .send_with_response(&d1402::packets::outbound::set_equalizer(
                target_equalizer_configuration,
                &target_hear_id,
                equalizer_unknown_bytes,
            ))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_equalizer_configuration;
            *state.get_mut() = target_hear_id;
        });
        Ok(())
    }
}
