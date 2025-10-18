use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        modules::button_configuration::{ButtonConfigurationSettings, ButtonSettings},
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
        structures::button_configuration::{Button, ButtonStatusCollection},
    },
};

pub struct ButtonConfigurationStateModifier<
    ConnectionType: RfcommConnection,
    const NUM_BUTTONS: usize,
    const NUM_PRESS_KINDS: usize,
> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
    supports_set_all_packet: bool,
    button_data: [ButtonData; NUM_BUTTONS],
}

#[derive(Copy, Clone)]
struct ButtonData {
    button: Button,
    button_settings: ButtonSettings,
}

impl<ConnectionType: RfcommConnection, const NUM_BUTTONS: usize, const NUM_PRESS_KINDS: usize>
    ButtonConfigurationStateModifier<ConnectionType, NUM_BUTTONS, NUM_PRESS_KINDS>
{
    pub fn new(
        packet_io: Arc<PacketIOController<ConnectionType>>,
        settings: &ButtonConfigurationSettings<NUM_BUTTONS, NUM_PRESS_KINDS>,
    ) -> Self {
        Self {
            packet_io,
            supports_set_all_packet: settings.supports_set_all_packet,
            button_data: settings.order.map(|button| ButtonData {
                button,
                button_settings: settings.button_settings(button).unwrap(),
            }),
        }
    }
}

#[async_trait]
impl<ConnectionType: RfcommConnection, const NUM_BUTTONS: usize, const NUM_PRESS_KINDS: usize, T>
    StateModifier<T>
    for ButtonConfigurationStateModifier<ConnectionType, NUM_BUTTONS, NUM_PRESS_KINDS>
where
    T: Has<ButtonStatusCollection<NUM_BUTTONS>> + Clone + Send + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_statuses = target_state.get();

        let num_changes: usize = {
            let state = state_sender.borrow();
            let statuses = state.get();
            statuses
                .0
                .iter()
                .zip(&target_statuses.0)
                .map(|(current, target)| usize::from(current != target))
                .sum()
        };
        if num_changes == 0 {
            return Ok(());
        }

        if self.supports_set_all_packet && num_changes > 1 {
            self.packet_io
                .send_with_response(
                    &packet::outbound::SetAllButtonConfigurations {
                        buttons: target_statuses,
                        parse_settings: &self
                            .button_data
                            .map(|data| data.button_settings.parse_settings),
                    }
                    .into(),
                )
                .await?;
            state_sender.send_modify(|state| *state.get_mut() = *target_statuses);
        } else {
            for (i, target) in target_statuses.0.iter().enumerate() {
                let current = state_sender.borrow().get().0[i];
                if current != *target {
                    let ButtonData {
                        button,
                        button_settings,
                    } = self.button_data[i];

                    if current.action != target.action {
                        self.packet_io
                            .send_with_response(
                                &packet::outbound::SetButtonConfiguration {
                                    button_id: button_settings.button_id,
                                    side: button.side(),
                                    action_id: target
                                        .action
                                        .byte(button_settings.parse_settings.action_kind),
                                }
                                .into(),
                            )
                            .await?;
                    }
                    if current.enabled != target.enabled
                        && let Some(enabled) = target.enabled
                    {
                        self.packet_io
                            .send_with_response(
                                &packet::outbound::SetButtonConfigurationEnabled {
                                    button_id: button_settings.button_id,
                                    side: button.side(),
                                    enabled: enabled
                                        .byte(button_settings.parse_settings.enabled_flag_kind),
                                }
                                .into(),
                            )
                            .await?;
                    }
                    state_sender.send_modify(|state| state.get_mut().0[i] = *target);
                }
            }
        }

        Ok(())
    }
}
