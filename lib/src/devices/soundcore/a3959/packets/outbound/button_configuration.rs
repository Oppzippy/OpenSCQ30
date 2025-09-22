use crate::devices::soundcore::{
    a3959,
    common::packet::{Command, outbound::OutboundPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonConfiguration {
    button: a3959::structures::Button,
    action: a3959::structures::TwsButtonAction,
}

impl ButtonConfiguration {
    pub fn new(
        button: a3959::structures::Button,
        action: a3959::structures::TwsButtonAction,
    ) -> Self {
        Self { button, action }
    }
}

impl OutboundPacket for ButtonConfiguration {
    fn command(&self) -> Command {
        Command([0x04, 0x81])
    }

    fn body(&self) -> Vec<u8> {
        self.button.bytes().chain(self.action.bytes()).collect()
    }
}
