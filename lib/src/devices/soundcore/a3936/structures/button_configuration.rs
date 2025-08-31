use nom::{
    IResult, Parser,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::pair,
};

use crate::devices::soundcore::common::structures::ButtonAction;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct A3936InternalMultiButtonConfiguration {
    pub left_single_click: A3936TwsButtonAction,
    pub right_single_click: A3936TwsButtonAction,
    pub left_double_click: A3936TwsButtonAction,
    pub right_double_click: A3936TwsButtonAction,
    pub left_long_press: A3936TwsButtonAction,
    pub right_long_press: A3936TwsButtonAction,
}

impl A3936InternalMultiButtonConfiguration {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12);
        bytes.extend(self.left_single_click.bytes());
        bytes.extend(self.right_single_click.bytes());
        bytes.extend(self.left_double_click.bytes());
        bytes.extend(self.right_double_click.bytes());
        bytes.extend(self.left_long_press.bytes());
        bytes.extend(self.right_long_press.bytes());
        bytes
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "custom button model",
            map(
                (
                    context("left single click", A3936TwsButtonAction::take),
                    context("right single click", A3936TwsButtonAction::take),
                    context("left double click", A3936TwsButtonAction::take),
                    context("right double click", A3936TwsButtonAction::take),
                    context("left long press", A3936TwsButtonAction::take),
                    context("right long press", A3936TwsButtonAction::take),
                ),
                |(
                    left_single_click,
                    right_single_click,
                    left_double_click,
                    right_double_click,
                    left_long_press,
                    right_long_press,
                )| {
                    Self {
                        left_double_click,
                        left_long_press,
                        right_double_click,
                        right_long_press,
                        left_single_click,
                        right_single_click,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct A3936TwsButtonAction {
    pub tws_connected_action: ButtonAction,
    pub tws_disconnected_action: ButtonAction,
    pub tws_connected_is_enabled: bool,
    pub tws_disconnected_is_enabled: bool,
}

impl A3936TwsButtonAction {
    pub fn bytes(&self) -> [u8; 2] {
        [
            (u8::from(self.tws_disconnected_is_enabled) << 4) | u8::from(self.tws_connected_action),
            (u8::from(self.tws_disconnected_action) << 4) | (u8::from(self.tws_connected_action)),
        ]
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map_opt(pair(le_u8, le_u8), |(switch, num)| {
            let log_and_return_default = || {
                tracing::warn!(
                    "A3936: unknown button action {num:#X}, falling back to default value",
                );
                ButtonAction::default()
            };
            Some(Self {
                tws_connected_is_enabled: switch & 0x01 == 0x01,
                tws_disconnected_is_enabled: switch & 0x10 == 0x10,
                tws_connected_action: ButtonAction::from_repr(num & 0x0F)
                    .unwrap_or_else(log_and_return_default),
                tws_disconnected_action: ButtonAction::from_repr((num & 0xF0) >> 4)
                    .unwrap_or_else(log_and_return_default),
            })
        })
        .parse_complete(input)
    }

    pub fn action_if_enabled(&self, is_tws_connected: bool) -> Option<ButtonAction> {
        if self.is_enabled(is_tws_connected) {
            Some(self.active_action(is_tws_connected))
        } else {
            None
        }
    }

    pub fn active_action(&self, is_tws_connected: bool) -> ButtonAction {
        if is_tws_connected {
            self.tws_connected_action
        } else {
            self.tws_disconnected_action
        }
    }

    pub fn set_action(&mut self, action: ButtonAction, is_tws_connected: bool) {
        if is_tws_connected {
            self.tws_connected_action = action;
        } else {
            self.tws_disconnected_action = action;
        }
    }

    pub fn is_enabled(&self, is_tws_connected: bool) -> bool {
        if is_tws_connected {
            self.tws_connected_is_enabled
        } else {
            self.tws_disconnected_is_enabled
        }
    }

    pub fn set_enabled(&mut self, is_enabled: bool, is_tws_connected: bool) {
        if is_tws_connected {
            self.tws_connected_is_enabled = is_enabled;
        } else {
            self.tws_disconnected_is_enabled = is_enabled;
        }
    }
}
