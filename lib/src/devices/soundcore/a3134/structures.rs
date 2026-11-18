use nom::{
    IResult, Parser,
    combinator::{map, verify},
    error::{ContextError, ParseError, context},
    multi::{count, length_count},
    number::complete::le_u8,
};
use strum::VariantArray;

use crate::{
    devices::soundcore::common::{
        self,
        macros::sound_mode_enum,
        modules::equalizer::EqualizerPreset,
        packet::parsing::take_bool,
        structures::{EqualizerConfiguration, VolumeAdjustments},
    },
    i18n::fl,
};

common::structures::flag!(BassUp);

pub type A3134EqualizerConfiguration = EqualizerConfiguration<1, 9, -60, 60, 1>;
pub type A3134EqualizerPreset = EqualizerPreset<9, -60, 60, 1>;

pub const CUSTOM_EQUALIZER_PRESET_ID: u8 = 0xfe;

const GAIN_OFFSET: i16 = 120;
/// The official app sends these for custom profiles.
const BAND_FREQUENCY_STEPS: [u8; 9] = [7, 7, 7, 7, 7, 7, 7, 6, 1];
const BAND_UNKNOWN: u8 = 10;

/// Curves as the official app sends them.
pub fn equalizer_presets() -> [A3134EqualizerPreset; 4] {
    [
        EqualizerPreset {
            id: 0,
            name: "SoundcoreSignature",
            localized_name: || fl!("soundcore-signature"),
            volume_adjustments: VolumeAdjustments::new([10, 0, 10, 0, 10, -10, 0, 15, 15]),
        },
        EqualizerPreset {
            id: 1,
            name: "Voice",
            localized_name: || fl!("voice"),
            volume_adjustments: VolumeAdjustments::new([-20, 0, 20, 10, 20, 20, 20, 0, 0]),
        },
        EqualizerPreset {
            id: 2,
            name: "TrebleBooster",
            localized_name: || fl!("treble-booster"),
            volume_adjustments: VolumeAdjustments::new([-10, -10, 10, 0, 20, 0, 10, 20, 40]),
        },
        EqualizerPreset {
            id: 3,
            name: "Balanced",
            localized_name: || fl!("balanced"),
            volume_adjustments: VolumeAdjustments::new([-10, -10, 0, -10, 0, -10, 0, 10, -10]),
        },
    ]
}

/// The state update packet carries the custom profile even while a preset is active.
pub fn effective_equalizer_configuration(
    equalizer_configuration: A3134EqualizerConfiguration,
) -> A3134EqualizerConfiguration {
    equalizer_presets()
        .into_iter()
        .find(|preset| preset.id == equalizer_configuration.preset_id())
        .map_or(equalizer_configuration, |preset| {
            EqualizerConfiguration::new_all_bands_present(preset.id, [preset.volume_adjustments])
        })
}

pub fn take_equalizer_configuration<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], A3134EqualizerConfiguration, E> {
    context(
        "equalizer configuration",
        map((le_u8, count(take_band_gain, 9)), |(preset_id, gains)| {
            EqualizerConfiguration::new_all_bands_present(
                preset_id.into(),
                [VolumeAdjustments::new(
                    gains.try_into().expect("count guarantees 9 bands"),
                )],
            )
        }),
    )
    .parse_complete(input)
}

fn take_band_gain<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], i16, E> {
    map(
        (le_u8, le_u8, le_u8),
        |(gain, _frequency_step, _unknown)| i16::from(gain) - GAIN_OFFSET,
    )
    .parse_complete(input)
}

pub fn equalizer_band_bytes(
    equalizer_configuration: &A3134EqualizerConfiguration,
) -> impl Iterator<Item = u8> {
    let gains = *equalizer_configuration
        .volume_adjustments_channel_1()
        .copied()
        .unwrap_or_default()
        .adjustments();
    gains
        .into_iter()
        .zip(BAND_FREQUENCY_STEPS)
        .flat_map(|(gain, frequency_step)| {
            [
                u8::try_from(gain + GAIN_OFFSET).expect("gain is clamped to -60..=60"),
                frequency_step,
                BAND_UNKNOWN,
            ]
        })
}

sound_mode_enum!(
    pub enum LightsEffect {
        Rainbow = 0,
        Flash = 1,
        Flame = 2,
        Lightning = 3,
        Energy = 4,
        Scan = 5,
        Cyclone = 6,
        Bounce = 7,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightsBrightness(u8);

impl Default for LightsBrightness {
    fn default() -> Self {
        Self(Self::MAX)
    }
}

impl LightsBrightness {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 100;

    pub fn new(brightness: u8) -> Self {
        Self(brightness.clamp(Self::MIN, Self::MAX))
    }

    pub fn inner(self) -> u8 {
        self.0
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, Self::new).parse_complete(input)
    }
}

type Rgb = [u8; 3];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Lights {
    pub effect: LightsEffect,
    pub brightness: LightsBrightness,
    pub is_enabled: bool,
    effect_colors: [Vec<Rgb>; LightsEffect::VARIANTS.len()],
}

impl Lights {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "lights",
            map(
                (
                    LightsEffect::take,
                    LightsBrightness::take,
                    take_bool,
                    take_effect_colors,
                ),
                |(effect, brightness, is_enabled, effect_colors)| Self {
                    effect,
                    brightness,
                    is_enabled,
                    effect_colors,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [
            self.effect.byte(),
            self.brightness.inner(),
            self.is_enabled.into(),
        ]
        .into_iter()
        .chain(
            LightsEffect::VARIANTS
                .iter()
                .flat_map(|effect| self.effect_body(*effect)),
        )
    }

    /// The speaker keeps colors per effect, and selecting an effect sends them along.
    pub fn effect_body(&self, effect: LightsEffect) -> Vec<u8> {
        let colors = &self.effect_colors[usize::from(effect.byte())];
        [
            effect.byte(),
            u8::try_from(colors.len()).expect("color count is parsed from a u8"),
        ]
        .into_iter()
        .chain(colors.iter().flatten().copied())
        .collect()
    }
}

fn take_effect_colors<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    mut input: &'a [u8],
) -> IResult<&'a [u8], [Vec<Rgb>; LightsEffect::VARIANTS.len()], E> {
    let mut effect_colors: [Vec<Rgb>; LightsEffect::VARIANTS.len()] = Default::default();
    for (effect, colors) in LightsEffect::VARIANTS.iter().zip(&mut effect_colors) {
        let (remaining, (_id, parsed)) = context(
            "lights effect colors",
            (
                verify(le_u8, |id| *id == effect.byte()),
                length_count(le_u8, map((le_u8, le_u8, le_u8), |(r, g, b)| [r, g, b])),
            ),
        )
        .parse_complete(input)?;
        *colors = parsed;
        input = remaining;
    }
    Ok((input, effect_colors))
}
