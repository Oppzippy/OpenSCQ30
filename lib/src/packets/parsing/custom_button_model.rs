use nom::{
    combinator::{map, map_opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::{pair, tuple},
};

use crate::packets::structures::{
    ButtonAction, CustomButtonModel, NoTwsButtonAction, TwsButtonAction,
};

use super::{take_bool, ParseResult};

pub fn take_custom_button_model<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<CustomButtonModel, E> {
    context("custom button model", |input| {
        map(
            tuple((
                take_tws_button_action,    // left double click
                take_tws_button_action,    // left long press
                take_tws_button_action,    // right double click
                take_tws_button_action,    // right long press
                take_no_tws_button_action, // left single click
                take_no_tws_button_action, // right single click
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

fn take_tws_button_action<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
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

fn take_no_tws_button_action<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<NoTwsButtonAction, E> {
    map_opt(pair(take_bool, le_u8), |(switch, num)| {
        Some(NoTwsButtonAction {
            action: ButtonAction::from_repr(num)?,
            is_enabled: switch,
        })
    })(input)
}
