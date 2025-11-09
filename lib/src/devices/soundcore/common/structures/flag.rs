use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError},
};

use crate::devices::soundcore::common::packet::parsing::take_bool;

pub trait Flag {
    fn get_bool(&self) -> bool;
    fn set_bool(&mut self, value: bool);
}

macro_rules! flag {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name(pub bool);

        #[allow(unused)]
        impl $name {
            pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                input: &'a [u8],
            ) -> IResult<&'a [u8], Self, E> {
                map(take_bool, Self).parse_complete(input)
            }

            pub fn bytes(&self) -> [u8; 1] {
                [self.0.into()]
            }
        }

        impl Flag for $name {
            fn get_bool(&self) -> bool {
                self.0
            }

            fn set_bool(&mut self, value: bool) {
                self.0 = value;
            }
        }
    };
}

flag!(TouchTone);
flag!(GamingMode);
flag!(SoundLeakCompensation);
flag!(SurroundSound);
flag!(AutoPlayPause);
flag!(WearingTone);
flag!(TouchLock);
flag!(LowBatteryPrompt);
