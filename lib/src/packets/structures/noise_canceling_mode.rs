use strum::{Display, FromRepr};

#[repr(u8)]
#[derive(FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
pub enum NoiseCancelingMode {
    Transport = 0,
    Outdoor = 1,
    Indoor = 2,
    Custom = 3,
}

impl NoiseCancelingMode {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}

#[cfg(test)]
mod tests {
    use super::NoiseCancelingMode;

    #[test]
    fn from_id_creates_with_valid_id() {
        let mode = NoiseCancelingMode::from_id(0);
        assert_eq!(true, mode.is_some());
    }

    #[test]
    fn from_id_returns_none_with_invalid_id() {
        let mode = NoiseCancelingMode::from_id(100);
        assert_eq!(true, mode.is_none());
    }

    #[test]
    fn getting_id_works() {
        assert_eq!(1, NoiseCancelingMode::Outdoor.id());
    }
}
