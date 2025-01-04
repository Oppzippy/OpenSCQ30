use nom::{
    combinator::{map, map_opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::{pair, tuple},
    IResult,
};

use crate::devices::standard::structures::{
    ButtonAction, ButtonConfiguration, MultiButtonConfiguration,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
        context("custom button model", |input| {
            map(
                tuple((
                    context("left single click", A3936TwsButtonAction::take),
                    context("right single click", A3936TwsButtonAction::take),
                    context("left double click", A3936TwsButtonAction::take),
                    context("right double click", A3936TwsButtonAction::take),
                    context("left long press", A3936TwsButtonAction::take),
                    context("right long press", A3936TwsButtonAction::take),
                )),
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
            )(input)
        })(input)
    }

    pub fn as_multi_button_configuration(
        &self,
        is_tws_connected: bool,
    ) -> MultiButtonConfiguration {
        MultiButtonConfiguration {
            left_single_click: self
                .left_single_click
                .as_button_configuration(is_tws_connected),
            right_single_click: self
                .right_single_click
                .as_button_configuration(is_tws_connected),
            left_double_click: self
                .left_double_click
                .as_button_configuration(is_tws_connected),
            right_double_click: self
                .right_double_click
                .as_button_configuration(is_tws_connected),
            left_long_press: self
                .left_long_press
                .as_button_configuration(is_tws_connected),
            right_long_press: self
                .right_long_press
                .as_button_configuration(is_tws_connected),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
            Some(A3936TwsButtonAction {
                tws_connected_is_enabled: switch & 0x01 == 0x01,
                tws_disconnected_is_enabled: switch & 0x10 == 0x10,
                tws_connected_action: ButtonAction::from_repr(num & 0x0F)
                    .unwrap_or_else(log_and_return_default),
                tws_disconnected_action: ButtonAction::from_repr((num & 0xF0) >> 4)
                    .unwrap_or_else(log_and_return_default),
            })
        })(input)
    }

    pub fn as_button_configuration(&self, is_tws_connected: bool) -> ButtonConfiguration {
        ButtonConfiguration {
            action: self.active_action(is_tws_connected),
            is_enabled: self.is_enabled(is_tws_connected),
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
