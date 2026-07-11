mod packet_handler;
mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::MaybeHas;

use crate::{
    api::settings::CategoryId,
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        structures::{
            AutoPlayPause, DisableAllButtons, Flag, GamingMode, Ldac, LowBatteryPrompt,
            SoundLeakCompensation, SurroundSound, TouchLock, TouchTone, VoicePrompt,
            WearingDetection, WearingTone,
        },
    },
    settings::SettingId,
};

use paste::paste;

use super::ModuleCollection;

macro_rules! flag {
    ($flag_struct:ident, $flag_configuration:expr $(,)?) => {
        impl<T> ModuleCollection<T>
        where
            T: MaybeHas<$flag_struct> + Send + Sync,
        {
            paste! {
                pub fn [<add_ $flag_struct:snake>](&mut self, packet_io: Arc<PacketIOController>) {
                    self.add_flag(packet_io, $flag_configuration);
                }
            }
        }
    };
}

flag!(
    TouchTone,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::TouchTone,
        set_command: packet::outbound::SET_TOUCH_TONE_COMMAND,
        update_command: None,
        is_inverted: false,
    },
);

flag!(
    GamingMode,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::GamingMode,
        set_command: packet::outbound::SET_GAMING_MODE_COMMAND,
        update_command: Some(packet::inbound::GAMING_MODE_UPDATE_COMMAND),
        is_inverted: false,
    },
);

flag!(
    SoundLeakCompensation,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::SoundLeakCompensation,
        set_command: packet::outbound::SET_SOUND_LEAK_COMPENSATION_COMMAND,
        update_command: None,
        is_inverted: false,
    },
);

flag!(
    SurroundSound,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::SurroundSound,
        set_command: packet::outbound::SET_SURROUND_SOUND_COMMAND,
        update_command: None,
        is_inverted: false,
    },
);

flag!(
    AutoPlayPause,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::AutoPlayPause,
        set_command: packet::outbound::SET_AUTO_PLAY_PAUSE_COMMAND,
        update_command: None,
        is_inverted: false,
    },
);

flag!(
    WearingTone,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::WearingTone,
        set_command: packet::outbound::SET_WEARING_TONE_COMMAND,
        update_command: None,
        is_inverted: false,
    },
);

flag!(
    TouchLock,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::TouchLock,
        set_command: packet::outbound::SET_TOUCH_LOCK_COMMAND,
        update_command: None,
        is_inverted: false,
    },
);

flag!(
    LowBatteryPrompt,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::LowBatteryPrompt,
        set_command: packet::outbound::SET_LOW_BATTERY_PROMPT_COMMAND,
        update_command: None,
        is_inverted: false,
    },
);

flag!(
    WearingDetection,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::WearingDetection,
        set_command: packet::outbound::SET_WEARING_DETECTION_COMMAND,
        update_command: None,
        is_inverted: false,
    },
);

flag!(
    VoicePrompt,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::VoicePrompt,
        set_command: packet::Command([0x01, 0x90]),
        update_command: Some(packet::Command([0x01, 0x10])),
        is_inverted: false,
    },
);

flag!(
    Ldac,
    FlagConfiguration {
        category_id: CategoryId::Miscellaneous,
        setting_id: SettingId::Ldac,
        set_command: packet::Command([0x01, 0xFF]),
        update_command: Some(packet::Command([0x01, 0x7F])),
        is_inverted: false,
    },
);

flag!(
    DisableAllButtons,
    FlagConfiguration {
        category_id: CategoryId::ButtonConfiguration,
        setting_id: SettingId::ButtonsEnabled,
        set_command: packet::Command([0x10, 0x94]),
        update_command: None,
        is_inverted: true,
    },
);

impl<T> ModuleCollection<T> {
    pub fn add_flag<FlagT>(
        &mut self,
        packet_io: Arc<PacketIOController>,
        flag_configuration: FlagConfiguration,
    ) where
        T: MaybeHas<FlagT> + Send + Sync,
        FlagT: Flag + Send + Sync + PartialEq + Copy + 'static,
    {
        if let Some(update_command) = flag_configuration.update_command {
            self.packet_handlers.set_handler(
                update_command,
                Box::new(packet_handler::FlagPacketHandler::default()),
            );
        }
        self.setting_manager.add_handler(
            flag_configuration.category_id,
            setting_handler::FlagSettingHandler::new(
                flag_configuration.setting_id,
                flag_configuration.is_inverted,
            ),
        );
        self.state_modifiers
            .push(Box::new(state_modifier::FlagStateModifier::new(
                packet_io,
                flag_configuration.set_command,
            )));
    }
}

pub struct FlagConfiguration {
    pub category_id: CategoryId,
    pub setting_id: SettingId,
    pub set_command: packet::Command,
    pub update_command: Option<packet::Command>,
    /// If true, the setting will have the opposite value as the flag. For example, this is used
    /// to make the disable button actions flag instead be an enable button actions flag.
    pub is_inverted: bool,
}
