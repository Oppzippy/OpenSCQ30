use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    FromRepr,
    Default,
    Display,
    AsRefStr,
    IntoStaticStr,
    EnumIter,
    EnumString,
    Translate,
)]
pub enum TransparencyMode {
    FullyTransparent = 0,
    #[default]
    VocalMode = 1,
}

impl TransparencyMode {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], TransparencyMode, E> {
        context(
            "transparency mode",
            map(le_u8, |transparency_mode| {
                TransparencyMode::from_id(transparency_mode).unwrap_or_default()
            }),
        )(input)
    }
}
