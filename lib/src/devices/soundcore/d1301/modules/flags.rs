use std::sync::Arc;

use crate::{
    devices::soundcore::{
        common::{
            modules::{ModuleCollection, flag::FlagConfiguration},
            packet::{self, PacketIOController},
        },
        d1301::{self, state::D1301State},
    },
    settings::{CategoryId, SettingId},
};

impl ModuleCollection<D1301State> {
    pub fn add_d1301_auto_power_off_prompt(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag::<d1301::structures::AutoPowerOffPrompt>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::AutoPowerOffPrompt,
                set_command: packet::Command([16, 158]),
                update_command: None,
                is_inverted: false,
            },
        );
    }

    pub fn add_d1301_listening_mode_prompt(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag::<d1301::structures::ListeningModePrompt>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::ListeningModePrompt,
                set_command: packet::Command([16, 159]),
                update_command: None,
                is_inverted: false,
            },
        );
    }

    pub fn add_d1301_noise_canceling(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag::<d1301::structures::NoiseCanceling>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::NoiseCanceling,
                set_command: packet::Command([6, 135]),
                update_command: None,
                is_inverted: false,
            },
        );
    }

    pub fn add_d1301_incoming_calls_during_bluetooth_mode(
        &mut self,
        packet_io: Arc<PacketIOController>,
    ) {
        self.add_flag::<d1301::structures::IncomingCallsDuringBluetoothMode>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::IncomingCallsDuringBluetoothMode,
                set_command: packet::Command([16, 117]),
                update_command: None,
                is_inverted: false,
            },
        );
    }

    pub fn add_d1301_tap_controls_disabled(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag::<d1301::structures::TapControlsDisabled>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::ButtonConfiguration,
                setting_id: SettingId::ButtonsEnabled,
                set_command: packet::Command([16, 148]),
                update_command: None,
                is_inverted: true,
            },
        );
    }

    pub fn add_d1301_noise_canceling_prompt(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag::<d1301::structures::NoiseCancelingPrompt>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::NoiseCancelingPrompt,
                set_command: packet::Command([16, 119]),
                update_command: None,
                is_inverted: false,
            },
        );
    }
}
