use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        packet::{self, PacketIOController, outbound::ToPacket},
        state_modifier::StateModifier,
    },
};

pub struct ResetButtonConfigurationStateModifier<ConnectionType: RfcommConnection, RefreshStateFn> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
    refresh_button_state: RefreshStateFn,
}

impl<ConnectionType, RefreshStateFn>
    ResetButtonConfigurationStateModifier<ConnectionType, RefreshStateFn>
where
    ConnectionType: RfcommConnection,
{
    pub fn new(
        packet_io: Arc<PacketIOController<ConnectionType>>,
        refresh_button_state: RefreshStateFn,
    ) -> Self {
        Self {
            packet_io,
            refresh_button_state,
        }
    }
}

#[async_trait]
impl<ConnectionType, StateType, RefreshStateFn, RefreshStateFut> StateModifier<StateType>
    for ResetButtonConfigurationStateModifier<ConnectionType, RefreshStateFn>
where
    ConnectionType: RfcommConnection + Send + Sync,
    StateType: Has<ResetButtonConfigurationPending> + Clone + Send + Sync,
    RefreshStateFn: Fn(watch::Sender<StateType>) -> RefreshStateFut + Sync,
    RefreshStateFut: Future<Output = device::Result<()>> + Send,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateType>,
        target_state: &StateType,
    ) -> device::Result<()> {
        if target_state.get().0 {
            self.packet_io
                .send_with_response(
                    &packet::outbound::ResetButtonConfigurationsToDefault.to_packet(),
                )
                .await?;
            (self.refresh_button_state)(state_sender.clone()).await?;
        }
        Ok(())
    }
}
