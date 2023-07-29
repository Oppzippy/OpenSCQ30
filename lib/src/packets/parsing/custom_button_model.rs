use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::{pair, tuple},
};

use crate::packets::structures::CustomButtonModel;

use super::{take_bool, ParseResult};

pub fn take_custom_button_model<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<CustomButtonModel, E> {
    context("custom button model", |input| {
        map(
            tuple((
                take_tws_pair, // left double click
                take_tws_pair, // left long press
                take_tws_pair, // right double click
                take_tws_pair, // right long press
                take_bool,     // left single switch
                le_u8,         // left single click
                take_bool,     // right single switch
                le_u8,         // right single click
            )),
            |(
                _left_double_click,
                _left_long_press,
                _right_double_click,
                _right_long_press,
                _left_single_click_switch,
                _left_single_click,
                _right_single_click_switch,
                _right_single_click,
            )| {
                // TODO
                CustomButtonModel {}
            },
        )(input)
    })(input)
}

struct TwsPairWithSwitch {
    pub tws: u8,
    pub un_tws: u8,
    pub switch: bool,
}

fn take_tws_pair<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<TwsPairWithSwitch, E> {
    map(pair(take_bool, le_u8), |(switch, num)| TwsPairWithSwitch {
        tws: num & 0x0F,
        un_tws: (num & 0xF0) >> 4,
        switch,
    })(input)
}
