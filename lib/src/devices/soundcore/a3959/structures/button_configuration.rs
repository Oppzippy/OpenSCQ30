use nom::{
    IResult, Parser,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::pair,
};

use crate::devices::soundcore::standard::structures::ButtonAction;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct A3959MultiButtonConfiguration {
    pub left_single_click: A3959TwsButtonAction,
    pub right_single_click: A3959TwsButtonAction,
    pub left_double_click: A3959TwsButtonAction,
    pub right_double_click: A3959TwsButtonAction,
    pub left_triple_click: A3959TwsButtonAction,
    pub right_triple_click: A3959TwsButtonAction,
    pub left_long_press: A3959TwsButtonAction,
    pub right_long_press: A3959TwsButtonAction,
}

impl A3959MultiButtonConfiguration {
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        self.left_single_click
            .bytes()
            .into_iter()
            .chain(self.right_single_click.bytes())
            .chain(self.left_double_click.bytes())
            .chain(self.right_double_click.bytes())
            .chain(self.left_triple_click.bytes())
            .chain(self.right_triple_click.bytes())
            .chain(self.left_long_press.bytes())
            .chain(self.right_long_press.bytes())
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "custom button configuration",
            map(
                (
                    context("left single click", A3959TwsButtonAction::take),
                    context("right single click", A3959TwsButtonAction::take),
                    context("left double click", A3959TwsButtonAction::take),
                    context("right double click", A3959TwsButtonAction::take),
                    context("left triple click", A3959TwsButtonAction::take),
                    context("right triple click", A3959TwsButtonAction::take),
                    context("left long press", A3959TwsButtonAction::take),
                    context("right long press", A3959TwsButtonAction::take),
                ),
                |(
                    left_single_click,
                    right_single_click,
                    left_double_click,
                    right_double_click,
                    left_triple_click,
                    right_triple_click,
                    left_long_press,
                    right_long_press,
                )| {
                    Self {
                        left_single_click,
                        right_single_click,
                        left_double_click,
                        right_double_click,
                        left_triple_click,
                        right_triple_click,
                        left_long_press,
                        right_long_press,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct A3959TwsButtonAction {
    pub tws_connected_action: ButtonAction,
    pub tws_disconnected_action: ButtonAction,
    pub tws_connected_is_enabled: bool,
    pub tws_disconnected_is_enabled: bool,
}

impl A3959TwsButtonAction {
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
                    "A3959: unknown button action {num:#X}, falling back to default value",
                );
                ButtonAction::default()
            };
            Some(A3959TwsButtonAction {
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
