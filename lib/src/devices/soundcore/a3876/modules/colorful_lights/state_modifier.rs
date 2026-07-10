use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3876::{self, structures::ColorfulLights},
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct ColorfulLightsStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl ColorfulLightsStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for ColorfulLightsStateModifier
where
    T: Has<ColorfulLights> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = *target_state.get();
        let current = *state_sender.borrow().get();

        if current.is_enabled != target.is_enabled {
            self.packet_io
                .send_with_response(&a3876::packets::outbound::set_colorful_lights_enabled(
                    target.is_enabled,
                ))
                .await?;
            state_sender.send_modify(|state| state.get_mut().is_enabled = target.is_enabled);
        }

        if current.brightness != target.brightness {
            self.packet_io
                .send_with_response(&a3876::packets::outbound::set_colorful_lights_brightness(
                    target.brightness,
                ))
                .await?;
            state_sender.send_modify(|state| state.get_mut().brightness = target.brightness);
        }

        if current.auto_lights_off_duration != target.auto_lights_off_duration {
            self.packet_io
                .send_with_response(
                    &a3876::packets::outbound::set_colorful_lights_auto_lights_off_duration(
                        target.auto_lights_off_duration,
                    ),
                )
                .await?;
            state_sender.send_modify(|state| {
                state.get_mut().auto_lights_off_duration = target.auto_lights_off_duration;
            });
        }

        if current.color != target.color {
            self.packet_io
                .send_with_response(&a3876::packets::outbound::set_colorful_lights_color(
                    target.color,
                ))
                .await?;
            state_sender.send_modify(|state| state.get_mut().color = target.color);
        }

        if current.mode != target.mode {
            self.packet_io
                .send_with_response(&a3876::packets::outbound::set_colorful_lights_mode(
                    target.mode,
                ))
                .await?;
            state_sender.send_modify(|state| state.get_mut().mode = target.mode);
        }

        Ok(())
    }
}
