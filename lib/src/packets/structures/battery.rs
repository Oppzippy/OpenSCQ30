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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SingleBattery {
    pub is_charging: IsBatteryCharging,
    pub level: BatteryLevel,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum IsBatteryCharging {
    #[default]
    No,
    Yes,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BatteryLevel(pub u8);
