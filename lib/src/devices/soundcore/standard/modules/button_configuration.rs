use std::sync::Arc;

use setting_handler::ButtonConfigurationSettingHandler;
use state_modifier::ButtonConfigurationStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::standard::{
        packet::packet_io_controller::PacketIOController,
        structures::{MultiButtonConfiguration, TwsStatus},
    },
};

use super::ModuleCollection;

mod setting_handler;
mod state_modifier;

#[derive(EnumString, EnumIter, IntoStaticStr)]
#[allow(clippy::enum_variant_names)]
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
            SettingId::LeftSinglePress => Ok(Self::LeftSinglePress),
            SettingId::LeftDoublePress => Ok(Self::LeftDoublePress),
            SettingId::LeftLongPress => Ok(Self::LeftLongPress),
            SettingId::RightSinglePress => Ok(Self::RightSinglePress),
            SettingId::RightDoublePress => Ok(Self::RightDoublePress),
            SettingId::RightLongPress => Ok(Self::RightLongPress),
            _ => Err(()),
        }
    }
}

impl From<ButtonConfigurationSetting> for SettingId {
    fn from(value: ButtonConfigurationSetting) -> Self {
        match value {
            ButtonConfigurationSetting::LeftSinglePress => Self::LeftSinglePress,
            ButtonConfigurationSetting::LeftDoublePress => Self::LeftDoublePress,
            ButtonConfigurationSetting::LeftLongPress => Self::LeftLongPress,
            ButtonConfigurationSetting::RightSinglePress => Self::RightSinglePress,
            ButtonConfigurationSetting::RightDoublePress => Self::RightDoublePress,
            ButtonConfigurationSetting::RightLongPress => Self::RightLongPress,
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsMut<MultiButtonConfiguration> + AsRef<MultiButtonConfiguration> + Clone + Send + Sync,
    T: AsRef<TwsStatus>,
{
    pub fn add_button_configuration<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            ButtonConfigurationSettingHandler::new(),
        );
        self.state_modifiers
            .push(Box::new(ButtonConfigurationStateModifier::new(packet_io)));
    }
}
