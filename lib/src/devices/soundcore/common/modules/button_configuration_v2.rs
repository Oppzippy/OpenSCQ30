use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::ButtonConfigurationSettingHandler;
use state_modifier::ButtonConfigurationStateModifier;

use crate::{
    api::{connection::RfcommConnection, settings::CategoryId},
    devices::soundcore::common::{
        packet::PacketIOController,
        structures::{TwsStatus, button_configuration_v2::*},
    },
    i18n::fl,
};

use super::ModuleCollection;

mod setting_handler;
mod state_modifier;

#[derive(Copy, Clone, Debug)]
pub struct ButtonConfigurationSettings<const NUM_BUTTONS: usize, const NUM_PRESS_KINDS: usize> {
    pub supports_set_all_packet: bool,
    pub order: [Button; NUM_BUTTONS],
    pub settings: [ButtonSettings; NUM_PRESS_KINDS],
}

impl<const NUM_BUTTONS: usize, const NUM_PRESS_KINDS: usize>
    ButtonConfigurationSettings<NUM_BUTTONS, NUM_PRESS_KINDS>
{
    pub fn button_settings(&self, button: Button) -> Option<ButtonSettings> {
        self.settings
            .iter()
            .find(|s| s.press_kind == button.press_kind())
            .copied()
    }

    pub fn position(&self, button: Button) -> Option<usize> {
        self.order.iter().position(|b| *b == button)
    }

    pub fn default_status_collection(&self) -> ButtonStatusCollection<NUM_BUTTONS> {
        let statuses = self.order.map(|button| {
            let settings = self
                .button_settings(button)
                .expect("if it is found in order, it should also be found in settings");

            let enabled = match settings.parse_settings.enabled_flag_kind {
                EnabledFlagKind::None => None,
                EnabledFlagKind::Single => Some(EnabledStatus::Single(true)),
                EnabledFlagKind::TwsLowBits => Some(EnabledStatus::Tws {
                    connected: true,
                    disconnected: true,
                }),
            };

            let default_action_id = settings.available_actions.first().unwrap().id;
            let action = match settings.parse_settings.action_kind {
                ActionKind::Single => ActionStatus::Single(default_action_id),
                ActionKind::TwsLowBits => ActionStatus::Tws {
                    connected: default_action_id,
                    disconnected: default_action_id,
                },
            };
            ButtonStatus { enabled, action }
        });
        ButtonStatusCollection::new(statuses)
    }

    pub fn parse_settings(&self) -> [ButtonParseSettings; NUM_BUTTONS] {
        self.order
            .map(|button| self.button_settings(button).unwrap().parse_settings)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ButtonSettings {
    pub button_id: u8,
    pub press_kind: ButtonPressKind,
    pub parse_settings: ButtonParseSettings,
    pub disable_mode: ButtonDisableMode,
    pub available_actions: &'static [ButtonAction],
}

#[derive(Clone, Debug)]
pub struct ButtonAction {
    pub id: u8,
    pub name: &'static str,
    pub localized_name: fn() -> String,
}

#[derive(Copy, Clone, Debug)]
pub enum ButtonDisableMode {
    NotDisablable,
    DisablingOneSideDisablesOther,
    IndividualDisable,
}

pub const COMMON_SETTINGS: ButtonConfigurationSettings<6, 3> = ButtonConfigurationSettings {
    supports_set_all_packet: true,
    order: [
        Button::LeftDoublePress,
        Button::LeftLongPress,
        Button::RightDoublePress,
        Button::RightLongPress,
        Button::LeftSinglePress,
        Button::RightSinglePress,
    ],
    settings: [
        ButtonSettings {
            parse_settings: ButtonParseSettings {
                enabled_flag_kind: EnabledFlagKind::Single,
                action_kind: ActionKind::TwsLowBits,
            },
            button_id: 0,
            press_kind: ButtonPressKind::Double,
            available_actions: COMMON_TWS_ACTIONS,
            disable_mode: ButtonDisableMode::NotDisablable,
        },
        ButtonSettings {
            parse_settings: ButtonParseSettings {
                enabled_flag_kind: EnabledFlagKind::Single,
                action_kind: ActionKind::TwsLowBits,
            },
            button_id: 1,
            press_kind: ButtonPressKind::Long,
            available_actions: COMMON_TWS_ACTIONS,
            disable_mode: ButtonDisableMode::NotDisablable,
        },
        ButtonSettings {
            parse_settings: ButtonParseSettings {
                enabled_flag_kind: EnabledFlagKind::Single,
                action_kind: ActionKind::Single,
            },
            button_id: 2,
            press_kind: ButtonPressKind::Single,
            available_actions: COMMON_TWS_ACTIONS,
            disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
        },
    ],
};

pub const COMMON_TWS_ACTIONS: &'static [ButtonAction] = &[
    ButtonAction {
        id: 0,
        name: "VolumeUp",
        localized_name: || fl!("volume-up"),
    },
    ButtonAction {
        id: 1,
        name: "VolumeDown",
        localized_name: || fl!("volume-down"),
    },
    ButtonAction {
        id: 2,
        name: "PreviousSong",
        localized_name: || fl!("previous-song"),
    },
    ButtonAction {
        id: 3,
        name: "NextSong",
        localized_name: || fl!("next-song"),
    },
    ButtonAction {
        id: 4,
        name: "AmbientSoundMode",
        localized_name: || fl!("ambient-sound-mode"),
    },
    ButtonAction {
        id: 5,
        name: "VoiceAssistant",
        localized_name: || fl!("voice-assistant"),
    },
    ButtonAction {
        id: 6,
        name: "PlayPause",
        localized_name: || fl!("play-pause"),
    },
    ButtonAction {
        id: 9,
        name: "GameMode",
        localized_name: || fl!("game-mode"),
    },
];

impl<T> ModuleCollection<T>
where
    T: Has<TwsStatus> + Clone + Send + Sync,
{
    pub fn add_button_configuration_v2<
        ConnectionType,
        const NUM_BUTTONS: usize,
        const NUM_PRESS_KINDS: usize,
    >(
        &mut self,
        packet_io: Arc<PacketIOController<ConnectionType>>,
        settings: &'static ButtonConfigurationSettings<NUM_BUTTONS, NUM_PRESS_KINDS>,
    ) where
        T: Has<ButtonStatusCollection<NUM_BUTTONS>>,
        ConnectionType: RfcommConnection + 'static + Send + Sync,
    {
        const {
            assert!(
                NUM_BUTTONS == NUM_PRESS_KINDS * 2,
                "NUM_BUTTONS should contain two entries for each press kind: one for the left side and one for the right",
            );
        }

        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            ButtonConfigurationSettingHandler::new(settings),
        );
        self.state_modifiers
            .push(Box::new(ButtonConfigurationStateModifier::new(
                packet_io, settings,
            )));
    }
}
