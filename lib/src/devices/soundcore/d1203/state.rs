use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    d1203::{
        modules::{D1203AmbientSoundMode, D1203CustomEqualizer},
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
    /// Ambient sound mode (ANC / Transparency / Normal / Adaptive). The D1203
    /// write command (`Command([0x06, 0x81])`, 7-byte payload) is identical to
    /// the A3957, but the D1203 has a FOURTH mode (`3 = Adaptive`) the A3957
    /// enum cannot represent, so we use a D1203-specific enum instead of
    /// reusing `a3957::structures::SoundModes`. See `modules::ambient_sound_mode`.
    ///
    /// TODO(read): which TLV tag of the D1203 state packet carries the current
    /// ambient sound mode is not yet known (a full state capture is pending),
    /// so this is initialized to `Default` on connect. Writing works
    /// regardless; only reading back the device's *current* mode is a TODO.
    ambient_sound_mode: D1203AmbientSoundMode,
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
            // TODO: read the real current ambient sound mode from the state
            // packet once the TLV tag is identified. For now start at Default;
            // writes still apply correctly (the device accepts the 06 81
            // command, including value 3 = Adaptive).
            ambient_sound_mode: value.ambient_sound_mode,
        }
    }
}
