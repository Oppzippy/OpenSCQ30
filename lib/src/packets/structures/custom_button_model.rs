use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomButtonModel {
    pub left_double_click: TwsButtonAction,
    pub left_long_press: TwsButtonAction,
    pub right_double_click: TwsButtonAction,
    pub right_long_press: TwsButtonAction,
    pub left_single_press: NoTwsButtonAction,
    pub right_single_press: NoTwsButtonAction,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwsButtonAction {
    pub tws_connected_action: ButtonAction,
    pub tws_disconnected_action: ButtonAction,
    pub is_enabled: bool,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoTwsButtonAction {
    pub action: ButtonAction,
    pub is_enabled: bool,
}

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, FromRepr,
)]
#[repr(u8)]
#[serde(rename_all = "camelCase")]
pub enum ButtonAction {
    VolumeUp = 0,
    VolumeDown = 1,
    PreviousSong = 2,
    NextSong = 3,
    Trans = 4, // what is this?
    VoiceAssistant = 5,
    PlayPause = 6,
}
