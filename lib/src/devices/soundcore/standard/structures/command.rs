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

    pub fn to_inbound(mut self) -> Self {
        self.0[0] = 0x09;
        self.0[1] = 0xff;
        self.0[4] = 0x01;
        self
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
