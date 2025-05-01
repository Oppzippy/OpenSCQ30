use nom::{
    IResult, Parser,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::pair,
};
use openscq30_i18n_macros::Translate;
use strum::{AsRefStr, EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::standard::packets::parsing::take_bool;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MultiButtonConfiguration {
    pub left_single_click: NoTwsButtonConfiguration,
    pub left_double_click: TwsButtonConfiguration,
    pub left_long_press: TwsButtonConfiguration,
    pub right_single_click: NoTwsButtonConfiguration,
    pub right_double_click: TwsButtonConfiguration,
    pub right_long_press: TwsButtonConfiguration,
}

impl MultiButtonConfiguration {
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

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], MultiButtonConfiguration, E> {
        context(
            "custom button model",
            map(
                (
                    context("left double click", TwsButtonConfiguration::take), // left double click
                    context("left long press", TwsButtonConfiguration::take),   // left long press
                    context("right double click", TwsButtonConfiguration::take), // right double click
                    context("right long press", TwsButtonConfiguration::take),   // right long press
                    context("left single click", NoTwsButtonConfiguration::take), // left single click
                    context("right single click", NoTwsButtonConfiguration::take), // right single click
                ),
                |(
                    left_double_click,
                    left_long_press,
                    right_double_click,
                    right_long_press,
                    left_single_press,
                    right_single_press,
                )| {
                    MultiButtonConfiguration {
                        left_double_click,
                        left_long_press,
                        right_double_click,
                        right_long_press,
                        left_single_click: left_single_press,
                        right_single_click: right_single_press,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TwsButtonConfiguration {
    pub tws_connected_action: ButtonAction,
    pub tws_disconnected_action: ButtonAction,
    pub disconnected_switch: bool,
}

impl TwsButtonConfiguration {
    pub fn bytes(&self) -> [u8; 2] {
        [
            self.disconnected_switch.into(),
            (u8::from(self.tws_disconnected_action) << 4)
                | (u8::from(self.tws_connected_action) & 0x0f),
        ]
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], TwsButtonConfiguration, E> {
        map_opt(pair(take_bool, le_u8), |(switch, num)| {
            Some(TwsButtonConfiguration {
                tws_connected_action: ButtonAction::from_repr(num & 0x0F)?,
                tws_disconnected_action: ButtonAction::from_repr((num & 0xF0) >> 4)?,
                disconnected_switch: switch,
            })
        })
        .parse_complete(input)
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
pub struct NoTwsButtonConfiguration {
    pub action: ButtonAction,
    pub is_enabled: bool,
}

impl NoTwsButtonConfiguration {
    pub fn enabled_action(&self) -> Option<ButtonAction> {
        self.is_enabled.then_some(self.action)
    }

    pub fn bytes(&self) -> [u8; 2] {
        [self.is_enabled.into(), u8::from(self.action) & 0x0f]
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], NoTwsButtonConfiguration, E> {
        map_opt(pair(take_bool, le_u8), |(switch, num)| {
            Some(NoTwsButtonConfiguration {
                action: ButtonAction::from_repr(num)?,
                is_enabled: switch,
            })
        })
        .parse_complete(input)
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
