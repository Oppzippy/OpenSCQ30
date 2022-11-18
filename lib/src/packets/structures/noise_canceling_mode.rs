use strum::FromRepr;

#[repr(u8)]
#[derive(FromRepr, Clone, Copy)]
pub enum NoiseCancelingMode {
    Transport = 0,
    Outdoor = 1,
    Indoor = 2,
}

impl NoiseCancelingMode {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}
