use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::{le_u8, le_u16},
};
use strum::FromRepr;

// unsure what this is. values 0 to 2 are allowed. maybe switch to an enum when the meanings are determined.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, FromRepr)]
#[repr(u8)]
pub enum HearIdType {
    #[default]
    Initial = 0,
    Custom = 1,
    FavoriteMusicGenre = 2,
}

impl HearIdType {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "hear id type",
            map(le_u8, |hear_id_type_byte| {
                Self::from_repr(hear_id_type_byte).unwrap_or_default()
            }),
        )
        .parse_complete(input)
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HearIdMusicGenre(pub u16);

impl HearIdMusicGenre {
    pub fn take_one_byte<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |genre_index| Self(genre_index.into())).parse_complete(input)
    }

    pub fn take_two_bytes<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u16, HearIdMusicGenre).parse_complete(input)
    }

    pub fn single_byte(&self) -> u8 {
        self.0 as u8
    }

    pub fn bytes(&self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}
