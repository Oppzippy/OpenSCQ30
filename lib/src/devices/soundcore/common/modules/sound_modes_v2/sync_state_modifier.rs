use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::{Notify, watch};

use crate::{
    api::device,
    devices::soundcore::common::{
        modules::sound_modes_v2::{Migrate, MigrationPlanner, ToPacketBody},
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
    },
};

pub struct SynchronizedSoundModesStateModifier<MigratableT, MigratableTFieldEnum, const SIZE: usize>
{
    packet_io: Arc<PacketIOController>,
    migration_planner: MigrationPlanner<MigratableTFieldEnum, SIZE>,
    notify: Arc<Notify>,
    _migratable: PhantomData<MigratableT>,
}

impl<MigratableT, MigratableTFieldEnum, const SIZE: usize>
    SynchronizedSoundModesStateModifier<MigratableT, MigratableTFieldEnum, SIZE>
where
    MigratableT: Migrate<SIZE, T = MigratableTFieldEnum>,
{
    pub fn new(packet_io: Arc<PacketIOController>, notify: Arc<Notify>) -> Self {
        Self {
            packet_io,
            migration_planner: MigratableT::migration_planner(),
            notify,
            _migratable: PhantomData,
        }
    }
}

#[async_trait]
impl<StateType, MigratableT, MigratableTFieldEnum, const SIZE: usize> StateModifier<StateType>
    for SynchronizedSoundModesStateModifier<MigratableT, MigratableTFieldEnum, SIZE>
where
    StateType: Has<MigratableT> + Clone + Send + Sync,
    MigratableT: Migrate<SIZE, T = MigratableTFieldEnum>
        + ToPacketBody
        + Send
        + Sync
        + PartialEq
        + std::fmt::Debug,
    MigratableTFieldEnum: Send + Sync,
{
    #[tracing::instrument(skip(self, state_sender, target_state))]
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateType>,
        target_state: &StateType,
    ) -> device::Result<()> {
        let path = {
            let from_state = state_sender.borrow();
            let from = from_state.get();
            let to = target_state.get();
            MigratableT::migrate(&self.migration_planner, from, to)
        };
        if let Some(last) = path.last() {
            tracing::debug!("migrating sound modes using path {path:?}");
            assert_eq!(
                last,
                target_state.get(),
                "last element in path should be target state"
            );
        }
        for step in path {
            let notified = self.notify.notified();
            self.packet_io
                .send_with_response(&packet::Outbound::new(
                    packet::Command([0x06, 0x81]),
                    step.bytes(),
                ))
                .await?;
            tokio::select! {
                _ = notified => {
                    tracing::debug!("got sound mode update, so the device is ready for another set sound mode command");
                    // state was changed by the packet handler, so we don't have to do anything
                    let state = state_sender.borrow();
                    let sound_modes = state.get();
                    if sound_modes != &step {
                        tracing::warn!("unexpected step. expected {step:?}, got {sound_modes:?}");
                    }
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    tracing::warn!(
                        "timed out waiting for sound mode update. does the device not send a sound mode update after we send a set sound mode?"
                    );
                    // assume it succeeded but the device doesn't send sound mode update, so we must manually modify the state
                    state_sender.send_modify(|v| {
                        *v.get_mut() = step;
                    });
                },
            }
        }
        Ok(())
    }
}
