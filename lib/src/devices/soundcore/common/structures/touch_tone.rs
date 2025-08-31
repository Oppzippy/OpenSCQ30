use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::common::packet::parsing::take_bool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TouchTone {
    #[default]
    Disabled,
    Enabled,
}

impl TouchTone {
    pub fn bytes(&self) -> [u8; 1] {
        [bool::from(*self).into()]
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("touch tone", map(take_bool, Into::into)).parse_complete(input)
    }
}

impl From<TouchTone> for bool {
    fn from(touch_tone: TouchTone) -> Self {
        match touch_tone {
            TouchTone::Disabled => false,
            TouchTone::Enabled => true,
        }
    }
}

impl From<bool> for TouchTone {
    fn from(is_enabled: bool) -> Self {
        if is_enabled {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}
