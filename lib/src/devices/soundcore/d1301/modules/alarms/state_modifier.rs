use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{
            packet::{PacketIOController, inbound::TryToPacket},
            state_modifier::StateModifier,
        },
        d1301::{self, packets::inbound::AlarmsPacket, structures::Alarm},
    },
};

pub struct AlarmsStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl AlarmsStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<StateT> StateModifier<StateT> for AlarmsStateModifier
where
    StateT: Has<Vec<Alarm>> + Send + Sync,
{
    #[tracing::instrument(skip(self, state_sender, target_state))]
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        let (remove_alarms, set_alarms) = {
            let state = state_sender.borrow();
            let current_alarms = state.get();
            let target_alarms = target_state.get();

            if current_alarms == target_alarms {
                return Ok(());
            }

            // This should always be sorted, since that property is made use of later in this function
            let remove_alarms = current_alarms
                .iter()
                .filter(|current_alarm| {
                    target_alarms
                        .iter()
                        .all(|target_alarm| current_alarm.id != target_alarm.id)
                })
                .map(|alarm| alarm.id)
                .collect::<Vec<_>>();

            // This covers modifying existing alarms and creating new ones
            let set_alarms = target_alarms
                .iter()
                .filter(|target_alarm| {
                    current_alarms.iter().all(|current_alarm| {
                        target_alarm.id != current_alarm.id || *target_alarm != current_alarm
                    })
                })
                .collect::<Vec<_>>();

            (remove_alarms, set_alarms)
        };

        // set alarms first, since that won't change indices
        for alarm in set_alarms {
            self.packet_io
                .send_with_response(&d1301::packets::outbound::set_alarm(alarm))
                .await?;
        }

        debug_assert!(
            remove_alarms.is_sorted(),
            "remove_alarms should always be sorted"
        );
        for (index, alarm_id) in remove_alarms.into_iter().enumerate().take(256) {
            // Alarms that come after the deleted alarm will shift over to take its place
            // Since remove_alarms is sorted,
            self.packet_io
                .send_with_response(&d1301::packets::outbound::delete_alarm(
                    alarm_id
                        - u8::try_from(index).expect(
                            "we capped the iterator at 256 elements, so this will always be <=255",
                        ),
                ))
                .await?;
        }

        // First update with what we assume the state is, and then double check with the device to confirm
        // This is done since I don't have the device to test with, so it will help catch unexpected behavior
        state_sender.send_modify(|state| {
            *state.get_mut() = target_state.get().clone();
        });

        // Verify that we got things right. In particular, what happens with alarm 1 when removing alarm 0
        // is what I'm unsure about. We don't want to have to send a request_alarms every time we change a
        // setting though, so this isn't enabled for release builds.
        #[cfg(debug_assertions)]
        {
            let alarms_packet: AlarmsPacket = self
                .packet_io
                .send_with_response(&d1301::packets::outbound::request_alarms())
                .await?
                .try_to_packet()?;
            let state = state_sender.borrow();
            if *state.get() != alarms_packet.0 {
                tracing::warn!(
                    expected = ?state.get(),
                    actual = ?alarms_packet.0,
                    "alarms didn't match what we predicted. please open an issue.",
                );
            }
        }

        Ok(())
    }
}
