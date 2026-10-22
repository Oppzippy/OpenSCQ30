use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    d1203::{
        modules::D1203CustomEqualizer,
        packets::inbound::{D1203PresetEqualizer, D1203StateUpdatePacket},
    },
    common::structures::{DualBattery, DualFirmwareVersion, SerialNumber},
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1203State {
    dual_battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    #[has(skip)]
    preset_equalizer: D1203PresetEqualizer,
    /// User-requested custom 8-band EQ gains (whole dB). Written by the
    /// interactive `customEqualizerProfile` setting; see `modules`.
    custom_equalizer: D1203CustomEqualizer,
}

impl D1203State {
    /// The EQ preset reported by the device (read-only/informational).
    #[allow(dead_code)]
    pub fn preset_equalizer(&self) -> D1203PresetEqualizer {
        self.preset_equalizer
    }
}

impl From<D1203StateUpdatePacket> for D1203State {
    fn from(value: D1203StateUpdatePacket) -> Self {
        Self {
            dual_battery: value.dual_battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            preset_equalizer: value.preset_equalizer,
            // The device does not report the current custom parametric EQ in
            // its state packet, so start flat; the value tracks what we write.
            custom_equalizer: D1203CustomEqualizer::default(),
        }
    }
}
