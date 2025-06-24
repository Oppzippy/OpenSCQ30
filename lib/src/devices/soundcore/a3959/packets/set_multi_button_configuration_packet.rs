use crate::devices::soundcore::{
    a3959::structures::A3959MultiButtonConfiguration,
    standard::{packets::outbound::OutboundPacket, structures::Command},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3959SetMultiButtonConfigurationPacket {
    button_configuration: A3959MultiButtonConfiguration,
}

impl A3959SetMultiButtonConfigurationPacket {
    pub fn new(button_configuration: A3959MultiButtonConfiguration) -> Self {
        Self {
            button_configuration,
        }
    }
}

impl OutboundPacket for A3959SetMultiButtonConfigurationPacket {
    fn command(&self) -> Command {
        Command::new([0x08, 0xEE, 0x00, 0x00, 0x00, 0x04, 0x84])
    }

    fn body(&self) -> Vec<u8> {
        self.button_configuration.bytes().collect()
    }
}
