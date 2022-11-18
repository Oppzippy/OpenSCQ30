use strum::FromRepr;

#[repr(u8)]
#[derive(FromRepr, Clone, Copy, Debug)]
pub enum AmbientSoundMode {
    NoiseCanceling = 0,
    Transparency = 1,
    Normal = 2,
}

impl AmbientSoundMode {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}
