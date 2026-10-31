use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{packet::PacketIOController, state_modifier::StateModifier},
        d1402,
    },
};

pub struct ButtonConfigurationStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl ButtonConfigurationStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for ButtonConfigurationStateModifier
where
    T: Has<d1402::structures::ButtonConfiguration>
        + Has<d1402::structures::MfbButtonConfiguration>
        + Clone
        + Send
        + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        move_nc_button_to_state(state_sender, target_state.get(), &self.packet_io).await?;
        move_mfb_button_to_state(state_sender, target_state.get(), &self.packet_io).await?;

        Ok(())
    }
}

async fn move_nc_button_to_state<T>(
    state_sender: &watch::Sender<T>,
    target_button_configuration: &d1402::structures::ButtonConfiguration,
    packet_io: &PacketIOController,
) -> device::Result<()>
where
    T: Has<d1402::structures::ButtonConfiguration>,
{
    let button_configuration = *state_sender.borrow().get();
    if &button_configuration == target_button_configuration {
        return Ok(());
    }

    packet_io
        .send_with_response(&d1402::packets::outbound::set_button_double_press_action(
            target_button_configuration,
        ))
        .await?;
    state_sender.send_modify(|state| *state.get_mut() = *target_button_configuration);

    Ok(())
}

async fn move_mfb_button_to_state<T>(
    state_sender: &watch::Sender<T>,
    target_mfb_button_configuration: &d1402::structures::MfbButtonConfiguration,
    packet_io: &PacketIOController,
) -> device::Result<()>
where
    T: Has<d1402::structures::MfbButtonConfiguration>,
{
    let button_configuration = *state_sender.borrow().get();
    if &button_configuration == target_mfb_button_configuration {
        return Ok(());
    }

    packet_io
        .send_with_response(&d1402::packets::outbound::set_mfb_long_press_action(
            target_mfb_button_configuration.action,
        ))
        .await?;
    state_sender.send_modify(|state| *state.get_mut() = *target_mfb_button_configuration);

    Ok(())
}
