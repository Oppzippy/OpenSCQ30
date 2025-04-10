
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Command([u8; 7]);

impl Command {
    pub const fn new(bytes: [u8; 7]) -> Self {
        Self(bytes)
    }

    pub fn bytes(&self) -> &[u8; 7] {
        &self.0
    }

    pub fn direction(&self) -> CommandDirection {
        if self.0[4] == 1 {
            CommandDirection::Inbound
        } else {
            CommandDirection::Outbound
        }
    }

    pub fn to_inbound(&self) -> Self {
        let mut bytes = self.0;
        bytes[0] = 0x09;
        bytes[1] = 0xff;
        bytes[4] = 0x01;
        Self(bytes)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub enum CommandDirection {
    #[default]
    Outbound,
    Inbound,
}

impl From<[u8; 7]> for Command {
    fn from(value: [u8; 7]) -> Self {
        Self(value)
    }
}
