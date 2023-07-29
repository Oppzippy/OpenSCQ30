use strum::FromRepr;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, FromRepr, Default)]
pub enum TransparencyMode {
    FullyTransparent = 0,
    #[default]
    VocalMode = 1,
}

impl TransparencyMode {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}
