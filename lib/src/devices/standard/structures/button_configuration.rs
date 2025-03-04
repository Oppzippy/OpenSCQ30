use nom::{
    IResult,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::{pair, tuple},
};
use openscq30_i18n_macros::Translate;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::standard::packets::parsing::take_bool;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MultiButtonConfiguration {
    pub left_single_click: ButtonConfiguration,
    pub left_double_click: ButtonConfiguration,
    pub left_long_press: ButtonConfiguration,
    pub right_single_click: ButtonConfiguration,
    pub right_double_click: ButtonConfiguration,
    pub right_long_press: ButtonConfiguration,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ButtonConfiguration {
    pub action: ButtonAction,
    pub is_enabled: bool,
}

impl ButtonConfiguration {
    pub fn enabled_action(&self) -> Option<ButtonAction> {
        self.is_enabled.then_some(self.action)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub(crate) struct InternalMultiButtonConfiguration {
    pub left_single_click: NoTwsButtonConfiguration,
    pub left_double_click: TwsButtonConfiguration,
    pub left_long_press: TwsButtonConfiguration,
    pub right_single_click: NoTwsButtonConfiguration,
    pub right_double_click: TwsButtonConfiguration,
    pub right_long_press: TwsButtonConfiguration,
}

impl From<InternalMultiButtonConfiguration> for MultiButtonConfiguration {
    fn from(value: InternalMultiButtonConfiguration) -> Self {
        Self {
            left_single_click: value.left_single_click.into(),
            left_double_click: value.left_double_click.into(),
            left_long_press: value.left_long_press.into(),
            right_single_click: value.right_single_click.into(),
            right_double_click: value.right_double_click.into(),
            right_long_press: value.right_long_press.into(),
        }
    }
}

impl InternalMultiButtonConfiguration {
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
    ) -> IResult<&'a [u8], InternalMultiButtonConfiguration, E> {
        context("custom button model", |input| {
            map(
                tuple((
                    context("left double click", TwsButtonConfiguration::take), // left double click
                    context("left long press", TwsButtonConfiguration::take),   // left long press
                    context("right double click", TwsButtonConfiguration::take), // right double click
                    context("right long press", TwsButtonConfiguration::take),   // right long press
                    context("left single click", NoTwsButtonConfiguration::take), // left single click
                    context("right single click", NoTwsButtonConfiguration::take), // right single click
                )),
                |(
                    left_double_click,
                    left_long_press,
                    right_double_click,
                    right_long_press,
                    left_single_press,
                    right_single_press,
                )| {
                    InternalMultiButtonConfiguration {
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub(crate) struct TwsButtonConfiguration {
    pub tws_connected_action: ButtonAction,
    pub tws_disconnected_action: ButtonAction,
    pub disconnected_switch: bool,
}

impl From<TwsButtonConfiguration> for ButtonConfiguration {
    fn from(value: TwsButtonConfiguration) -> Self {
        Self {
            action: value.active_action(),
            is_enabled: true,
        }
    }
}

impl TwsButtonConfiguration {
    pub fn bytes(&self) -> [u8; 2] {
        [
            self.disconnected_switch.into(),
            (u8::from(self.tws_disconnected_action) << 4)
                | (u8::from(self.tws_connected_action) & 0x0f),
        ]
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], TwsButtonConfiguration, E> {
        map_opt(pair(take_bool, le_u8), |(switch, num)| {
            Some(TwsButtonConfiguration {
                tws_connected_action: ButtonAction::from_repr(num & 0x0F)?,
                tws_disconnected_action: ButtonAction::from_repr((num & 0xF0) >> 4)?,
                disconnected_switch: switch,
            })
        })(input)
    }

    pub fn set_action(&mut self, action: ButtonAction, is_tws_connected: bool) {
        self.disconnected_switch = !is_tws_connected;
        if self.disconnected_switch {
            self.tws_disconnected_action = action;
        } else {
            self.tws_connected_action = action;
        }
    }

    pub fn active_action(&self) -> ButtonAction {
        if self.disconnected_switch {
            self.tws_disconnected_action
        } else {
            self.tws_connected_action
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub(crate) struct NoTwsButtonConfiguration {
    pub action: ButtonAction,
    pub is_enabled: bool,
}

impl From<NoTwsButtonConfiguration> for ButtonConfiguration {
    fn from(value: NoTwsButtonConfiguration) -> Self {
        Self {
            action: value.action,
            is_enabled: value.is_enabled,
        }
    }
}

impl NoTwsButtonConfiguration {
    pub fn enabled_action(&self) -> Option<ButtonAction> {
        self.is_enabled.then_some(self.action)
    }

    pub fn bytes(&self) -> [u8; 2] {
        [self.is_enabled.into(), u8::from(self.action) & 0x0f]
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], NoTwsButtonConfiguration, E> {
        map_opt(pair(take_bool, le_u8), |(switch, num)| {
            Some(NoTwsButtonConfiguration {
                action: ButtonAction::from_repr(num)?,
                is_enabled: switch,
            })
        })(input)
    }
}

#[derive(
    Default,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    AsRefStr,
    IntoStaticStr,
    EnumString,
    Translate,
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
    GameMode = 9,
}

impl From<ButtonAction> for u8 {
    fn from(value: ButtonAction) -> Self {
        value as Self
    }
}
