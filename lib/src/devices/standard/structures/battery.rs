use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
    IResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", tag = "type"))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DualBattery {
    pub left: SingleBattery,
    pub right: SingleBattery,
}

impl DualBattery {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], DualBattery, E> {
        context(
            "dual battery",
            map(
                tuple((
                    BatteryLevel::take,
                    BatteryLevel::take,
                    IsBatteryCharging::take,
                    IsBatteryCharging::take,
                )),
                |(left_level, right_level, is_left_charging, is_right_charging)| DualBattery {
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
        )(input)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SingleBattery {
    pub is_charging: IsBatteryCharging,
    pub level: BatteryLevel,
}

impl SingleBattery {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], SingleBattery, E> {
        context(
            "battery",
            map(
                tuple((BatteryLevel::take, IsBatteryCharging::take)),
                |(level, is_charging)| SingleBattery { level, is_charging },
            ),
        )(input)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all = "camelCase", from = "bool", into = "bool")
)]
pub enum IsBatteryCharging {
    #[default]
    No,
    Yes,
}

impl IsBatteryCharging {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], IsBatteryCharging, E> {
        context(
            "is battery charging",
            map(le_u8, |value| -> IsBatteryCharging {
                if value == 1 {
                    IsBatteryCharging::Yes
                } else {
                    IsBatteryCharging::No
                }
            }),
        )(input)
    }
}

impl From<bool> for IsBatteryCharging {
    fn from(value: bool) -> Self {
        match value {
            true => IsBatteryCharging::Yes,
            false => IsBatteryCharging::No,
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BatteryLevel(pub u8);

impl BatteryLevel {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], BatteryLevel, E> {
        context("battery level", map(le_u8, BatteryLevel))(input)
    }
}
