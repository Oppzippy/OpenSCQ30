use nom::{
    IResult, Parser,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::structures::ButtonAction;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MultiButtonConfiguration {
    pub left_single_click: TwsButtonAction,
    pub right_single_click: TwsButtonAction,
    pub left_double_click: TwsButtonAction,
    pub right_double_click: TwsButtonAction,
    pub left_triple_click: TwsButtonAction,
    pub right_triple_click: TwsButtonAction,
    pub left_long_press: TwsButtonAction,
    pub right_long_press: TwsButtonAction,
}

impl MultiButtonConfiguration {
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
                    context("left single click", TwsButtonAction::take),
                    context("right single click", TwsButtonAction::take),
                    context("left double click", TwsButtonAction::take),
                    context("right double click", TwsButtonAction::take),
                    context("left triple click", TwsButtonAction::take),
                    context("right triple click", TwsButtonAction::take),
                    context("left long press", TwsButtonAction::take),
                    context("right long press", TwsButtonAction::take),
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
pub struct TwsButtonAction {
    pub tws_connected_action: Option<ButtonAction>,
    pub tws_disconnected_action: Option<ButtonAction>,
}

impl TwsButtonAction {
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        let tws_disconnected_action = self.tws_disconnected_action.map(u8::from).unwrap_or(0xF);
        let tws_connected_action = self.tws_connected_action.map(u8::from).unwrap_or(0xF);
        std::iter::once((tws_disconnected_action << 4) | tws_connected_action)
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map_opt(le_u8, |num| {
            Some(Self {
                tws_connected_action: ButtonAction::from_repr(num & 0x0F),
                tws_disconnected_action: ButtonAction::from_repr((num & 0xF0) >> 4),
            })
        })
        .parse_complete(input)
    }

    pub fn active_action(&self, is_tws_connected: bool) -> Option<ButtonAction> {
        if is_tws_connected {
            self.tws_connected_action
        } else {
            self.tws_disconnected_action
        }
    }

    pub fn set_action(&mut self, action: Option<ButtonAction>, is_tws_connected: bool) {
        if is_tws_connected {
            self.tws_connected_action = action;
        } else {
            self.tws_disconnected_action = action;
        }
    }
}
