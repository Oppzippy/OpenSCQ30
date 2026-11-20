use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{packet::PacketIOController, state_modifier::StateModifier},
        d5103::{self, structures::Lights},
    },
};

pub struct LightsStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl LightsStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for LightsStateModifier
where
    T: Has<Lights> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: &Lights = target_state.get();
        let current = state_sender.borrow().get().clone();
        let turns_lights_on =
            current.effect != target.effect || current.brightness != target.brightness;

        // Changing the effect or brightness turns the lights on, so the explicit switch change
        // goes last.
        if current.effect != target.effect {
            self.packet_io
                .send_with_response(&d5103::packets::outbound::set_lights_effect(target))
                .await?;
            state_sender.send_modify(|state| {
                let lights: &mut Lights = state.get_mut();
                lights.effect = target.effect;
                lights.is_enabled = true;
            });
        }

        if current.brightness != target.brightness {
            self.packet_io
                .send_with_response(&d5103::packets::outbound::set_lights_brightness(
                    target.brightness,
                ))
                .await?;
            state_sender.send_modify(|state| {
                let lights: &mut Lights = state.get_mut();
                lights.brightness = target.brightness;
                lights.is_enabled = true;
            });
        }

        if (current.is_enabled != target.is_enabled || (turns_lights_on && !target.is_enabled))
            && !(target.is_enabled && turns_lights_on)
        {
            self.packet_io
                .send_with_response(&d5103::packets::outbound::set_lights_enabled(
                    target.is_enabled,
                ))
                .await?;
            state_sender.send_modify(|state| state.get_mut().is_enabled = target.is_enabled);
        }

        Ok(())
    }
}
