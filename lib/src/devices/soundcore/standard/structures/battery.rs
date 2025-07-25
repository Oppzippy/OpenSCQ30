use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use strum::Display;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Battery {
    SingleBattery(SingleBattery),
    DualBattery(DualBattery),
}

impl Default for Battery {
    fn default() -> Self {
        Self::SingleBattery(Default::default())
    }
}

impl From<SingleBattery> for Battery {
    fn from(single_battery: SingleBattery) -> Self {
        Self::SingleBattery(single_battery)
    }
}

impl From<DualBattery> for Battery {
    fn from(dual_battery: DualBattery) -> Self {
        Self::DualBattery(dual_battery)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct DualBattery {
    pub left: SingleBattery,
    pub right: SingleBattery,
}

impl DualBattery {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.left.is_charging as u8,
            self.right.is_charging as u8,
            self.left.level.0,
            self.right.level.0,
        ]
    }
}

impl DualBattery {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "dual battery",
            map(
                (
                    BatteryLevel::take,
                    BatteryLevel::take,
                    IsBatteryCharging::take,
                    IsBatteryCharging::take,
                ),
                |(left_level, right_level, is_left_charging, is_right_charging)| Self {
                    left: SingleBattery {
                        level: left_level,
                        is_charging: is_left_charging,
                    },
                    right: SingleBattery {
                        level: right_level,
                        is_charging: is_right_charging,
                    },
                },
            ),
        )
        .parse_complete(input)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SingleBattery {
    pub is_charging: IsBatteryCharging,
    pub level: BatteryLevel,
}

impl SingleBattery {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "battery",
            map(
                (BatteryLevel::take, IsBatteryCharging::take),
                |(level, is_charging)| Self { level, is_charging },
            ),
        )
        .parse_complete(input)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Display)]
#[repr(u8)]
pub enum IsBatteryCharging {
    #[default]
    No = 0,
    Yes = 1,
}

impl IsBatteryCharging {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "is battery charging",
            map(le_u8, |value| -> Self {
                if value == 1 { Self::Yes } else { Self::No }
            }),
        )
        .parse_complete(input)
    }
}

impl From<bool> for IsBatteryCharging {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Yes,
            false => Self::No,
        }
    }
}

impl From<IsBatteryCharging> for bool {
    fn from(value: IsBatteryCharging) -> Self {
        match value {
            IsBatteryCharging::No => false,
            IsBatteryCharging::Yes => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BatteryLevel(pub u8);

impl BatteryLevel {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("battery level", map(le_u8, BatteryLevel)).parse_complete(input)
    }
}
