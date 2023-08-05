use serde::{Deserialize, Serialize};
use strum::{Display, FromRepr};

#[repr(u8)]
#[derive(
    FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum AmbientSoundMode {
    #[default]
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

#[cfg(test)]
mod tests {
    use super::AmbientSoundMode;

    #[test]
    fn from_id_creates_with_valid_id() {
        let mode = AmbientSoundMode::from_id(0);
        assert_eq!(true, mode.is_some());
    }

    #[test]
    fn from_id_returns_none_with_invalid_id() {
        let mode = AmbientSoundMode::from_id(100);
        assert_eq!(true, mode.is_none());
    }

    #[test]
    fn getting_id_works() {
        assert_eq!(1, AmbientSoundMode::Transparency.id());
    }
}
