use crate::devices::{
    a3936::structures::A3936CustomButtonModel,
    standard::{packets::outbound::OutboundPacket, structures::Command},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3936SetCustomButtonModelPacket {
    custom_button_model: A3936CustomButtonModel,
}

impl A3936SetCustomButtonModelPacket {
    pub fn new(custom_button_model: A3936CustomButtonModel) -> Self {
        Self {
            custom_button_model,
        }
    }
}

impl OutboundPacket for A3936SetCustomButtonModelPacket {
    fn command(&self) -> Command {
        Command::new([0x08, 0xEE, 0x00, 0x00, 0x00, 0x04, 0x84])
    }

    fn body(&self) -> Vec<u8> {
        self.custom_button_model.bytes()
    }
}
