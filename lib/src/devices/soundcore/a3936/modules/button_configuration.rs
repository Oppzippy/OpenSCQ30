use std::sync::Arc;

use setting_handler::ButtonConfigurationSettingHandler;
use state_modifier::ButtonConfigurationStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::Connection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::{
        a3936::structures::A3936InternalMultiButtonConfiguration,
        standard::{
            modules::ModuleCollection, packets::packet_io_controller::PacketIOController,
            structures::TwsStatus,
        },
    },
};

mod setting_handler;
mod state_modifier;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum ButtonConfigurationSetting {
    LeftSinglePress,
    LeftDoublePress,
    LeftLongPress,
    RightSinglePress,
    RightDoublePress,
    RightLongPress,
}

impl TryFrom<&SettingId> for ButtonConfigurationSetting {
    type Error = ();

    fn try_from(value: &SettingId) -> Result<Self, Self::Error> {
        match value {
            SettingId::LeftSinglePress => Ok(ButtonConfigurationSetting::LeftSinglePress),
            SettingId::LeftDoublePress => Ok(ButtonConfigurationSetting::LeftDoublePress),
            SettingId::LeftLongPress => Ok(ButtonConfigurationSetting::LeftLongPress),
            SettingId::RightSinglePress => Ok(ButtonConfigurationSetting::RightSinglePress),
            SettingId::RightDoublePress => Ok(ButtonConfigurationSetting::RightDoublePress),
            SettingId::RightLongPress => Ok(ButtonConfigurationSetting::RightLongPress),
            _ => Err(()),
        }
    }
}

impl From<ButtonConfigurationSetting> for SettingId {
    fn from(value: ButtonConfigurationSetting) -> Self {
        match value {
            ButtonConfigurationSetting::LeftSinglePress => SettingId::LeftSinglePress,
            ButtonConfigurationSetting::LeftDoublePress => SettingId::LeftDoublePress,
            ButtonConfigurationSetting::LeftLongPress => SettingId::LeftLongPress,
            ButtonConfigurationSetting::RightSinglePress => SettingId::RightSinglePress,
            ButtonConfigurationSetting::RightDoublePress => SettingId::RightDoublePress,
            ButtonConfigurationSetting::RightLongPress => SettingId::RightLongPress,
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsMut<A3936InternalMultiButtonConfiguration>
        + AsRef<A3936InternalMultiButtonConfiguration>
        + Clone
        + Send
        + Sync,
    T: AsRef<TwsStatus>,
{
    pub fn add_a3936_button_configuration<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: Connection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            ButtonConfigurationSettingHandler::new(),
        );
        self.state_modifiers
            .push(Box::new(ButtonConfigurationStateModifier::new(packet_io)));
    }
}
