use openscq30_lib::packets::inbound::InboundPacket as LibInboundPacket;
use rifgen::rifgen_attr::generate_interface;

use crate::{
    type_conversion, BatteryChargingUpdatePacket, BatteryLevelUpdatePacket,
    ChineseVoicePromptStateUpdatePacket, FirmwareVersionUpdatePacket, LdacStateUpdatePacket,
    SetEqualizerOkPacket, SetEqualizerWithDrcOkPacket, SetSoundModeOkPacket, SoundModeUpdatePacket,
    StateUpdatePacket, TwsStatusUpdatePacket,
};

pub struct InboundPacket(LibInboundPacket);

impl InboundPacket {
    #[generate_interface(constructor)]
    pub fn new(input: &[i8]) -> Result<InboundPacket, String> {
        let input = type_conversion::i8_slice_to_u8_slice(input);
        LibInboundPacket::new(input)
            .map(Self)
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn sound_mode_update(&self) -> Option<SoundModeUpdatePacket> {
        if let LibInboundPacket::SoundModeUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn set_sound_mode_ok(&self) -> Option<SetSoundModeOkPacket> {
        if let LibInboundPacket::SetSoundModeOk(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn set_equalizer_ok(&self) -> Option<SetEqualizerOkPacket> {
        if let LibInboundPacket::SetEqualizerOk(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn state_update(&self) -> Option<StateUpdatePacket> {
        if let LibInboundPacket::StateUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn set_equalizer_with_drc_ok(&self) -> Option<SetEqualizerWithDrcOkPacket> {
        if let LibInboundPacket::SetEqualizerWithDrcOk(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn battery_charging_update(&self) -> Option<BatteryChargingUpdatePacket> {
        if let LibInboundPacket::BatteryChargingUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn battery_level_update(&self) -> Option<BatteryLevelUpdatePacket> {
        if let LibInboundPacket::BatteryLevelUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn chinese_voice_prompt_state_update(&self) -> Option<ChineseVoicePromptStateUpdatePacket> {
        if let LibInboundPacket::ChineseVoicePromptStateUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn firmware_version_update(&self) -> Option<FirmwareVersionUpdatePacket> {
        if let LibInboundPacket::FirmwareVersionUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn ldac_state_update(&self) -> Option<LdacStateUpdatePacket> {
        if let LibInboundPacket::LdacStateUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn tws_status_update(&self) -> Option<TwsStatusUpdatePacket> {
        if let LibInboundPacket::TwsStatusUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }
}
