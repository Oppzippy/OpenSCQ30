use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString};
use tokio::sync::watch;

use crate::{
    connection::RfcommConnection,
    device,
    devices::soundcore::common::{
        modules::{
            ModuleCollection,
            reset_button_configuration::{
                setting_handler::ResetButtonConfigurationSettingHandler,
                state_modifier::ResetButtonConfigurationStateModifier,
            },
        },
        packet::{
            self, PacketIOController,
            inbound::{FromPacketBody, TryToPacket},
        },
        state::Update,
    },
    macros::enum_subset,
    settings::{CategoryId, SettingId},
};

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumIter, EnumString)]
    #[allow(clippy::enum_variant_names)]
    enum ResetButtonConfigurationSetting {
        ResetButtonsToDefault,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd)]
pub struct ResetButtonConfigurationPending(pub bool);

impl<T> ModuleCollection<T>
where
    T: Has<ResetButtonConfigurationPending> + Clone + Send + Sync,
{
    pub fn add_reset_button_configuration<C, ButtonConfigurationPacketType>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        request_packet: packet::Outbound,
    ) where
        C: RfcommConnection + 'static + Send + Sync,
        T: Update<ButtonConfigurationPacketType>,
        ButtonConfigurationPacketType: FromPacketBody,
    {
        let refresh_button_state = {
            let packet_io = packet_io.clone();
            move |state: watch::Sender<T>| {
                let packet_io = packet_io.clone();
                let request_packet = request_packet.clone();
                async move {
                    let packet: ButtonConfigurationPacketType = packet_io
                        .send_with_response(&request_packet)
                        .await?
                        .try_to_packet()?;
                    state.send_modify(|s| s.update(packet));
                    Ok(()) as device::Result<()>
                }
            }
        };

        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            ResetButtonConfigurationSettingHandler::new(),
        );
        self.state_modifiers
            .push(Box::new(ResetButtonConfigurationStateModifier::new(
                packet_io,
                refresh_button_state,
            )));
    }
}
