use crate::devices::soundcore::standard::structures::{Command, MultiButtonConfiguration};

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetMultiButtonConfigurationPacket {
    button_configuration: MultiButtonConfiguration,
}

impl SetMultiButtonConfigurationPacket {
    pub(crate) fn new(button_configuration: MultiButtonConfiguration) -> Self {
        Self {
            button_configuration,
        }
    }
}

impl OutboundPacket for SetMultiButtonConfigurationPacket {
    fn command(&self) -> Command {
        Command::new([0x08, 0xEE, 0x00, 0x00, 0x00, 0x04, 0x84])
    }

    fn body(&self) -> Vec<u8> {
        self.button_configuration.bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::standard::{
        packets::outbound::{OutboundPacketBytesExt, SetMultiButtonConfigurationPacket},
        structures::{
            ButtonAction, MultiButtonConfiguration, NoTwsButtonConfiguration,
            TwsButtonConfiguration,
        },
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xEE, 0x00, 0x00, 0x00, 0x04, 0x84, 0x16, 0x00, 0x01, 0x63, 0x01, 0x42, 0x01,
            0x15, 0x00, 0x30, 0x01, 0x02, 0x00, 0x03, 0x87,
        ];

        let packet = SetMultiButtonConfigurationPacket::new(MultiButtonConfiguration {
            left_double_click: TwsButtonConfiguration {
                tws_connected_action: ButtonAction::NextSong,
                tws_disconnected_action: ButtonAction::PlayPause,
                disconnected_switch: true,
            },
            left_long_press: TwsButtonConfiguration {
                tws_connected_action: ButtonAction::PreviousSong,
                tws_disconnected_action: ButtonAction::AmbientSoundMode,
                disconnected_switch: true,
            },
            right_double_click: TwsButtonConfiguration {
                tws_connected_action: ButtonAction::VoiceAssistant,
                tws_disconnected_action: ButtonAction::VolumeDown,
                disconnected_switch: true,
            },
            right_long_press: TwsButtonConfiguration {
                tws_connected_action: ButtonAction::VolumeUp,
                tws_disconnected_action: ButtonAction::NextSong,
                disconnected_switch: false,
            },
            left_single_click: NoTwsButtonConfiguration {
                action: ButtonAction::PreviousSong,
                is_enabled: true,
            },
            right_single_click: NoTwsButtonConfiguration {
                action: ButtonAction::NextSong,
                is_enabled: false,
            },
        });
        assert_eq!(EXPECTED, packet.bytes());
    }
}
