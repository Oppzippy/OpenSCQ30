use nom::{
    combinator::{map, map_opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::{pair, tuple},
    IResult,
};

use crate::devices::standard::structures::{
    ButtonAction, CustomButtonModel, NoTwsButtonAction, TwsButtonAction,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct A3936CustomButtonModel {
    pub left_single_click: A3936TwsButtonAction,
    pub right_single_click: A3936TwsButtonAction,
    pub left_double_click: A3936TwsButtonAction,
    pub right_double_click: A3936TwsButtonAction,
    pub left_long_press: A3936TwsButtonAction,
    pub right_long_press: A3936TwsButtonAction,
}

impl From<A3936CustomButtonModel> for CustomButtonModel {
    fn from(value: A3936CustomButtonModel) -> Self {
        Self {
            left_single_click: value.left_single_click.into(),
            right_single_click: value.right_single_click.into(),
            left_double_click: value.left_double_click.into(),
            right_double_click: value.right_double_click.into(),
            left_long_press: value.left_long_press.into(),
            right_long_press: value.right_long_press.into(),
        }
    }
}

impl From<CustomButtonModel> for A3936CustomButtonModel {
    fn from(value: CustomButtonModel) -> Self {
        Self {
            left_single_click: value.left_single_click.into(),
            right_single_click: value.right_single_click.into(),
            left_double_click: value.left_double_click.into(),
            right_double_click: value.right_double_click.into(),
            left_long_press: value.left_long_press.into(),
            right_long_press: value.right_long_press.into(),
        }
    }
}

impl A3936CustomButtonModel {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12);
        bytes.extend(self.left_single_click.bytes());
        bytes.extend(self.right_single_click.bytes());
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
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct A3936TwsButtonAction {
    pub tws_connected_action: ButtonAction,
    pub tws_disconnected_action: ButtonAction,
    pub tws_connected_is_enabled: bool,
    pub tws_disconnected_is_enabled: bool,
}

impl From<A3936TwsButtonAction> for TwsButtonAction {
    fn from(value: A3936TwsButtonAction) -> Self {
        Self {
            tws_connected_action: value.tws_connected_action,
            tws_disconnected_action: value.tws_disconnected_action,
            is_enabled: value.tws_disconnected_is_enabled && value.tws_connected_is_enabled,
        }
    }
}

impl From<TwsButtonAction> for A3936TwsButtonAction {
    fn from(value: TwsButtonAction) -> Self {
        Self {
            tws_connected_action: value.tws_connected_action,
            tws_disconnected_action: value.tws_disconnected_action,
            tws_connected_is_enabled: value.is_enabled,
            tws_disconnected_is_enabled: value.is_enabled,
        }
    }
}

impl From<A3936TwsButtonAction> for NoTwsButtonAction {
    fn from(value: A3936TwsButtonAction) -> Self {
        Self {
            action: value.tws_connected_action,
            is_enabled: value.tws_connected_is_enabled && value.tws_disconnected_is_enabled,
        }
    }
}

impl From<NoTwsButtonAction> for A3936TwsButtonAction {
    fn from(value: NoTwsButtonAction) -> Self {
        Self {
            tws_connected_action: value.action,
            tws_disconnected_action: value.action,
            tws_connected_is_enabled: value.is_enabled,
            tws_disconnected_is_enabled: value.is_enabled,
        }
    }
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
}
