use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        modules::sound_modes_v2::{Migrate, MigrationPlanner, ToPacketBody},
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
    },
};

pub struct SoundModesStateModifier<
    ConnectionType: RfcommConnection,
    MigratableT,
    MigratableTFieldEnum,
    const SIZE: usize,
> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
    migration_planner: MigrationPlanner<MigratableTFieldEnum, SIZE>,
    _migratable: PhantomData<MigratableT>,
}

impl<ConnectionType, MigratableT, MigratableTFieldEnum, const SIZE: usize>
    SoundModesStateModifier<ConnectionType, MigratableT, MigratableTFieldEnum, SIZE>
where
    ConnectionType: RfcommConnection,
    MigratableT: Migrate<SIZE, T = MigratableTFieldEnum>,
{
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self {
            packet_io,
            migration_planner: MigratableT::migration_planner(),
            _migratable: PhantomData,
        }
    }
}

#[async_trait]
impl<ConnectionType, StateType, MigratableT, MigratableTFieldEnum, const SIZE: usize>
    StateModifier<StateType>
    for SoundModesStateModifier<ConnectionType, MigratableT, MigratableTFieldEnum, SIZE>
where
    ConnectionType: RfcommConnection + Send + Sync,
    StateType: Has<MigratableT> + Clone + Send + Sync,
    MigratableT: Migrate<SIZE, T = MigratableTFieldEnum>
        + ToPacketBody
        + Send
        + Sync
        + PartialEq
        + std::fmt::Debug,
    MigratableTFieldEnum: Send + Sync,
{
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
            tracing::info!("migrating sound modes using path {path:?}");
            assert_eq!(
                last,
                target_state.get(),
                "last element in path should be target state"
            );
        }
        for step in path {
            self.packet_io
                .send_with_response(&packet::Outbound::new(
                    packet::Command([0x06, 0x81]),
                    step.bytes(),
                ))
                .await?;
            state_sender.send_modify(|v| {
                *v.get_mut() = step;
            });
        }
        Ok(())
    }
}
