#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, FromRepr};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CustomButtonModel {
    pub left_single_click: NoTwsButtonAction,
    pub left_double_click: TwsButtonAction,
    pub left_long_press: TwsButtonAction,
    pub right_single_click: NoTwsButtonAction,
    pub right_double_click: TwsButtonAction,
    pub right_long_press: TwsButtonAction,
}

impl CustomButtonModel {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12);
        bytes.extend(self.left_double_click.bytes());
        bytes.extend(self.left_long_press.bytes());
        bytes.extend(self.right_double_click.bytes());
        bytes.extend(self.right_long_press.bytes());
        bytes.extend(self.left_single_click.bytes());
        bytes.extend(self.right_single_click.bytes());
        bytes
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TwsButtonAction {
    pub tws_connected_action: ButtonAction,
    pub tws_disconnected_action: ButtonAction,
    pub is_enabled: bool,
}

impl TwsButtonAction {
    pub fn bytes(&self) -> [u8; 2] {
        [
            self.is_enabled.into(),
            (u8::from(self.tws_disconnected_action) << 4)
                | (u8::from(self.tws_connected_action) & 0x0f),
        ]
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct NoTwsButtonAction {
    pub action: ButtonAction,
    pub is_enabled: bool,
}

impl NoTwsButtonAction {
    pub fn bytes(&self) -> [u8; 2] {
        [self.is_enabled.into(), u8::from(self.action) & 0x0f]
    }
}

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr, EnumIter, AsRefStr,
)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ButtonAction {
    VolumeUp = 0,
    VolumeDown = 1,
    PreviousSong = 2,
    NextSong = 3,
    AmbientSoundMode = 4,
    VoiceAssistant = 5,
    PlayPause = 6,
}

impl From<ButtonAction> for u8 {
    fn from(value: ButtonAction) -> Self {
        value as Self
    }
}
