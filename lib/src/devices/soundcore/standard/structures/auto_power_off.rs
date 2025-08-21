#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AutoPowerOff {
    pub is_enabled: bool,
    pub duration: AutoPowerOffDurationIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AutoPowerOffDurationIndex(pub u8);
