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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SingleBattery {
    pub is_charging: IsBatteryCharging,
    pub level: BatteryLevel,
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
