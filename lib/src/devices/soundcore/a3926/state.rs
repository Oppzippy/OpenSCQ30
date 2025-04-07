use crate::{
    devices::soundcore::standard::structures::{
        AgeRange, BasicHearId, DualBattery, EqualizerConfiguration, Gender,
        MultiButtonConfiguration, TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3926StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3926State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration<2, 8>,
    gender: Gender,
    age_range: AgeRange,
    hear_id: BasicHearId<2, 8>,
    button_configuration: MultiButtonConfiguration,
}

impl_as_ref_for_field!(
    struct A3926State {
        tws_status: TwsStatus,
        battery: DualBattery,
        equalizer_configuration: EqualizerConfiguration<2, 8>,
        gender: Gender,
        age_range: AgeRange,
        hear_id: BasicHearId<2, 8>,
        button_configuration: MultiButtonConfiguration,
    }
);

impl From<A3926StateUpdatePacket> for A3926State {
    fn from(value: A3926StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            equalizer_configuration: value.equalizer_configuration,
            gender: value.gender,
            age_range: value.age_range,
            hear_id: value.hear_id,
            button_configuration: value.button_configuration,
        }
    }
}
