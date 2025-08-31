use crate::devices::soundcore::{
    a3936::structures::A3936InternalMultiButtonConfiguration,
    common::packet::{Command, outbound::OutboundPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3936SetMultiButtonConfigurationPacket {
    button_configuration: A3936InternalMultiButtonConfiguration,
}

impl A3936SetMultiButtonConfigurationPacket {
    pub fn new(button_configuration: A3936InternalMultiButtonConfiguration) -> Self {
        Self {
            button_configuration,
        }
    }
}

impl OutboundPacket for A3936SetMultiButtonConfigurationPacket {
    fn command(&self) -> Command {
        Command([0x04, 0x84])
    }

    fn body(&self) -> Vec<u8> {
        self.button_configuration.bytes()
    }
}
