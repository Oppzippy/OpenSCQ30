use crate::devices::soundcore::common::{
    packet::{self, outbound::IntoPacket},
    structures::button_configuration::{ButtonParseSettings, ButtonSide, ButtonStatusCollection},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetButtonConfiguration {
    pub button_id: u8,
    pub side: ButtonSide,
    pub action_id: u8,
}

impl IntoPacket for SetButtonConfiguration {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        packet::Command([0x04, 0x81])
    }

    fn body(&self) -> Vec<u8> {
        vec![self.side.into(), self.button_id, self.action_id]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetButtonConfigurationsToDefault;

impl IntoPacket for ResetButtonConfigurationsToDefault {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        packet::Command([0x04, 0x82])
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

pub struct SetButtonConfigurationEnabled {
    pub button_id: u8,
    pub side: ButtonSide,
    pub enabled: u8,
}

impl IntoPacket for SetButtonConfigurationEnabled {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        // 0: unknown, maybe side?
        // 1: button id
        // 2: 0 for disabled, 1 for enabled
        // 00 02 01
        packet::Command([0x04, 0x83])
    }

    fn body(&self) -> Vec<u8> {
        vec![self.side.into(), self.button_id, self.enabled]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetAllButtonConfigurations<'a, const N: usize> {
    pub buttons: &'a ButtonStatusCollection<N>,
    pub parse_settings: &'a [ButtonParseSettings; N],
}

impl<const N: usize> IntoPacket for SetAllButtonConfigurations<'_, N> {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        packet::Command([0x04, 0x84])
    }

    fn body(&self) -> Vec<u8> {
        self.buttons
            .0
            .iter()
            .zip(self.parse_settings)
            .flat_map(|(status, parse_settings)| status.bytes(*parse_settings))
            .collect()
    }
}
