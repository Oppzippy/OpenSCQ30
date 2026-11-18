use crate::devices::soundcore::{
    a3134::structures::{self, A3134EqualizerConfiguration, Lights, LightsBrightness},
    common::packet::{self, Command},
};

pub const SET_BASS_UP_COMMAND: Command = Command([0x02, 0x84]);

/// Bit mask of the bands that follow, all 9 here.
const ALL_BANDS_MASK: [u8; 2] = [0x01, 0xff];

pub fn set_equalizer(equalizer_configuration: &A3134EqualizerConfiguration) -> packet::Outbound {
    packet::Outbound::new(
        Command([0x02, 0x88]),
        [equalizer_configuration.preset_id() as u8]
            .into_iter()
            .chain(ALL_BANDS_MASK)
            .chain(structures::equalizer_band_bytes(equalizer_configuration))
            .collect(),
    )
}

pub fn set_lights_enabled(is_enabled: bool) -> packet::Outbound {
    packet::Outbound::new(Command([0x0e, 0x08]), vec![is_enabled.into()])
}

pub fn set_lights_brightness(brightness: LightsBrightness) -> packet::Outbound {
    packet::Outbound::new(Command([0x0e, 0x05]), vec![brightness.inner()])
}

pub fn set_lights_effect(lights: &Lights) -> packet::Outbound {
    packet::Outbound::new(Command([0x0e, 0x07]), lights.effect_body(lights.effect))
}
