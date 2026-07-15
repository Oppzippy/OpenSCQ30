use crate::devices::soundcore::{
    a3876::structures::{
        AutoLightsOffDurationInMinutes, ColorfulLightsBrightness, ColorfulLightsMode, RgbColor,
        VolumeBalance,
    },
    common::{
        packet,
        structures::{EqualizerConfiguration, OptionalVolumeAdjustmentsExt},
    },
};

pub fn set_volume_balance(volume_balance: VolumeBalance) -> packet::Outbound {
    packet::Outbound::new(packet::Command([16, 140]), volume_balance.bytes().collect())
}

pub fn set_equalizer_with_drc<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>(
    equalizer_configuration: &EqualizerConfiguration<
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
) -> packet::Outbound {
    // Despite having two EQ channels, the set packet only sends one, which I assume is applied
    // to both sides
    packet::Outbound::new(
        packet::Command([0x02, 0x83]),
        equalizer_configuration
            .preset_id()
            .to_le_bytes()
            .into_iter()
            .chain(
                equalizer_configuration
                    .volume_adjustments_channel_1()
                    .bytes(),
            )
            .chain(
                equalizer_configuration
                    .volume_adjustments_channel_1()
                    .apply_drc()
                    .bytes(),
            )
            .collect(),
    )
}

pub fn set_colorful_lights_enabled(is_enabled: bool) -> packet::Outbound {
    packet::Outbound::new(packet::Command([14, 135]), vec![u8::from(is_enabled)])
}

pub fn set_colorful_lights_mode(mode: ColorfulLightsMode) -> packet::Outbound {
    packet::Outbound::new(packet::Command([14, 134]), mode.bytes().collect())
}

pub fn set_colorful_lights_color(color: RgbColor) -> packet::Outbound {
    packet::Outbound::new(packet::Command([14, 133]), color.bytes().collect())
}

pub fn set_colorful_lights_auto_lights_off_duration(
    duration: AutoLightsOffDurationInMinutes,
) -> packet::Outbound {
    packet::Outbound::new(packet::Command([14, 132]), vec![duration.inner()])
}

pub fn set_colorful_lights_brightness(brightness: ColorfulLightsBrightness) -> packet::Outbound {
    packet::Outbound::new(packet::Command([14, 130]), vec![brightness.inner()])
}
