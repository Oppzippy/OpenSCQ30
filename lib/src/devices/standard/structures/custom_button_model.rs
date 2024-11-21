use nom::{
    combinator::{map, map_opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::{pair, tuple},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, FromRepr};

use crate::devices::standard::packets::parsing::{take_bool, ParseResult};

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

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<CustomButtonModel, E> {
        context("custom button model", |input| {
            map(
                tuple((
                    TwsButtonAction::take,   // left double click
                    TwsButtonAction::take,   // left long press
                    TwsButtonAction::take,   // right double click
                    TwsButtonAction::take,   // right long press
                    NoTwsButtonAction::take, // left single click
                    NoTwsButtonAction::take, // right single click
                )),
                |(
                    left_double_click,
                    left_long_press,
                    right_double_click,
                    right_long_press,
                    left_single_press,
                    right_single_press,
                )| {
                    CustomButtonModel {
                        left_double_click,
                        left_long_press,
                        right_double_click,
                        right_long_press,
                        left_single_click: left_single_press,
                        right_single_click: right_single_press,
                    }
                },
            )(input)
        })(input)
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

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<TwsButtonAction, E> {
        map_opt(pair(take_bool, le_u8), |(switch, num)| {
            Some(TwsButtonAction {
                tws_connected_action: ButtonAction::from_repr(num & 0x0F)?,
                tws_disconnected_action: ButtonAction::from_repr((num & 0xF0) >> 4)?,
                is_enabled: switch,
            })
        })(input)
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

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<NoTwsButtonAction, E> {
        map_opt(pair(take_bool, le_u8), |(switch, num)| {
            Some(NoTwsButtonAction {
                action: ButtonAction::from_repr(num)?,
                is_enabled: switch,
            })
        })(input)
    }
}

#[derive(
    Default, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr, EnumIter, AsRefStr,
)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ButtonAction {
    #[default]
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
