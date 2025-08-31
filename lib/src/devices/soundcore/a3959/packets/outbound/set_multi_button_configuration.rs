use crate::devices::soundcore::{
    a3959,
    common::packet::{Command, outbound::OutboundPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3959SetMultiButtonConfiguration {
    button_configuration: a3959::structures::MultiButtonConfiguration,
}

impl A3959SetMultiButtonConfiguration {
    pub fn new(button_configuration: a3959::structures::MultiButtonConfiguration) -> Self {
        Self {
            button_configuration,
        }
    }
}

impl OutboundPacket for A3959SetMultiButtonConfiguration {
    fn command(&self) -> Command {
        Command([0x04, 0x84])
    }

    fn body(&self) -> Vec<u8> {
        self.button_configuration.bytes().collect()
    }
}
