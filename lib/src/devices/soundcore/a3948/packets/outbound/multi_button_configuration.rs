use crate::devices::soundcore::{
    a3948,
    common::packet::{Command, outbound::OutboundPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MultiButtonConfiguration {
    button_configuration: a3948::structures::MultiButtonConfiguration,
}

impl MultiButtonConfiguration {
    pub fn new(button_configuration: a3948::structures::MultiButtonConfiguration) -> Self {
        Self {
            button_configuration,
        }
    }
}

impl OutboundPacket for MultiButtonConfiguration {
    fn command(&self) -> Command {
        Command([0x04, 0x84])
    }

    fn body(&self) -> Vec<u8> {
        self.button_configuration.bytes().collect()
    }
}
