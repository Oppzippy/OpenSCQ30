use nom::{
    IResult, Parser,
    combinator::map,
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
        packet::parsing::{take_bool, take_str},
        structures::{EqualizerConfiguration, SerialNumber, VolumeAdjustments},
    },
    i18n::fl,
};

common::structures::flag!(BassUp);

pub type D5103EqualizerConfiguration = EqualizerConfiguration<1, 9, -80, 80, 1>;
pub type D5103EqualizerPreset = EqualizerPreset<9, -80, 80, 1>;

pub const CUSTOM_EQUALIZER_PRESET_ID: u8 = 0xfe;

const GAIN_OFFSET: i16 = 120;
const BAND_UNKNOWN: u8 = 10;

/// The band filter codes the speaker uses in its state update packet.
const BAND_CODES: [u8; 9] = [3, 6, 4, 2, 3, 3, 3, 5, 2];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D5103FirmwareVersion(String);

impl Default for D5103FirmwareVersion {
    fn default() -> Self {
        Self("0.0.0".into())
    }
}

impl D5103FirmwareVersion {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d5103 firmware version",
            map(take_str(5usize), |value| Self(value.to_owned())),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 5] {
        let mut bytes = [b'0'; 5];
        for (target, source) in bytes.iter_mut().zip(self.0.as_bytes()) {
            *target = *source;
        }
        bytes
    }
}

impl std::fmt::Display for D5103FirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn take_serial_number<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], SerialNumber, E> {
    context(
        "d5103 serial number",
        map(take_str(17usize), SerialNumber::from),
    )
    .parse_complete(input)
}

pub fn take_equalizer_configuration<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], D5103EqualizerConfiguration, E> {
    context(
        "d5103 equalizer configuration",
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
    map((le_u8, le_u8, le_u8), |(gain, _band_code, _unknown)| {
        i16::from(gain) - GAIN_OFFSET
    })
    .parse_complete(input)
}

pub fn equalizer_band_bytes(
    equalizer_configuration: &D5103EqualizerConfiguration,
) -> impl Iterator<Item = u8> {
    let gains = *equalizer_configuration
        .volume_adjustments_channel_1()
        .copied()
        .unwrap_or_default()
        .adjustments();
    gains
        .into_iter()
        .zip(BAND_CODES)
        .flat_map(|(gain, band_code)| {
            [
                u8::try_from(gain + GAIN_OFFSET).expect("gain is clamped to -80..=80"),
                band_code,
                BAND_UNKNOWN,
            ]
        })
}

/// Curves from the official app's four presets. The state packet carries the custom profile
/// even while a preset is active, so the active preset's curve comes from this list.
///
/// These curves are provisional until verified against a Boom Go 3i capture.
pub fn equalizer_presets() -> [D5103EqualizerPreset; 4] {
    [
        EqualizerPreset {
            id: 3,
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
            id: 0,
            name: "TrebleBooster",
            localized_name: || fl!("treble-booster"),
            volume_adjustments: VolumeAdjustments::new([-10, -10, 10, 0, 20, 0, 10, 20, 40]),
        },
        EqualizerPreset {
            id: 2,
            name: "Balanced",
            localized_name: || fl!("balanced"),
            volume_adjustments: VolumeAdjustments::new([-10, -10, 0, -10, 0, -10, 0, 10, -10]),
        },
    ]
}

pub fn effective_equalizer_configuration(
    equalizer_configuration: D5103EqualizerConfiguration,
) -> D5103EqualizerConfiguration {
    equalizer_presets()
        .into_iter()
        .find(|preset| preset.id == equalizer_configuration.preset_id())
        .map_or(equalizer_configuration, |preset| {
            EqualizerConfiguration::new_all_bands_present(preset.id, [preset.volume_adjustments])
        })
}

// Effect IDs come from the speaker's state packet. Names for the workout-light IDs
// (10-14) are provisional until verified against the official app.
sound_mode_enum!(
    pub enum D5103LightEffect {
        NeonNight = 0,
        RomanticStroll = 1,
        EuphoricDance = 2,
        Combo = 3,
        ColorLoop = 4,
        Intimate = 5,
        StarrySky = 6,
        Energy = 7,
        LaidBack = 8,
        CampfireVibes = 9,
        Nature = 10,
        Bonfire = 11,
        Lightning = 12,
        StepEcho = 13,
        SafeLight = 14,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightsBrightness(u8);

impl Default for LightsBrightness {
    fn default() -> Self {
        Self(100)
    }
}

impl LightsBrightness {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 100;

    pub fn new(brightness: u8) -> Self {
        Self(brightness.min(Self::MAX))
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

pub type Rgb = [u8; 3];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectBody {
    pub id: u8,
    pub colors: Vec<Rgb>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lights {
    pub effect: D5103LightEffect,
    pub brightness: LightsBrightness,
    pub is_enabled: bool,
    effect_bodies: Vec<EffectBody>,
}

impl Default for Lights {
    fn default() -> Self {
        Self {
            effect: D5103LightEffect::default(),
            brightness: LightsBrightness::default(),
            is_enabled: false,
            effect_bodies: default_effect_bodies(),
        }
    }
}

impl Lights {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d5103 lights",
            map(
                (
                    D5103LightEffect::take,
                    LightsBrightness::take,
                    take_bool,
                    take_effect_bodies,
                ),
                |(effect, brightness, is_enabled, effect_bodies)| Self {
                    effect,
                    brightness,
                    is_enabled,
                    effect_bodies,
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
        .chain(self.effect_bodies.iter().flat_map(|body| {
            std::iter::once(body.id)
                .chain(std::iter::once(
                    u8::try_from(body.colors.len()).expect("color count fits in u8"),
                ))
                .chain(body.colors.iter().flatten().copied())
        }))
    }

    /// Selecting an effect sends the stored color body, preserving any colors the speaker has
    /// cached for that effect.
    pub fn effect_body(&self, effect: D5103LightEffect) -> Vec<u8> {
        let colors = self
            .effect_bodies
            .iter()
            .find(|body| body.id == effect.byte())
            .map(|body| body.colors.as_slice())
            .unwrap_or(&[]);
        [
            effect.byte(),
            u8::try_from(colors.len()).expect("color count fits in u8"),
        ]
        .into_iter()
        .chain(colors.iter().flatten().copied())
        .collect()
    }
}

fn take_effect_bodies<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    mut input: &'a [u8],
) -> IResult<&'a [u8], Vec<EffectBody>, E> {
    let mut effect_bodies = Vec::with_capacity(D5103LightEffect::VARIANTS.len());
    for _ in 0..D5103LightEffect::VARIANTS.len() {
        let (remaining, (id, colors)) = (
            le_u8,
            length_count(
                map(le_u8, usize::from),
                map((le_u8, le_u8, le_u8), |(r, g, b)| [r, g, b]),
            ),
        )
            .parse_complete(input)?;
        effect_bodies.push(EffectBody { id, colors });
        input = remaining;
    }
    Ok((input, effect_bodies))
}

fn default_effect_bodies() -> Vec<EffectBody> {
    const BODIES: &[(u8, &[Rgb])] = &[
        (0, &[[255, 255, 255], [0, 255, 247], [0, 166, 255]]),
        (
            1,
            &[
                [255, 32, 33],
                [255, 80, 26],
                [255, 156, 26],
                [255, 225, 26],
                [164, 255, 26],
                [26, 255, 122],
                [26, 228, 255],
                [26, 129, 255],
                [125, 26, 255],
                [228, 26, 255],
                [255, 32, 110],
            ],
        ),
        (2, &[[30, 199, 255], [21, 253, 154], [131, 66, 254]]),
        (
            3,
            &[
                [135, 206, 237],
                [95, 233, 235],
                [59, 186, 240],
                [37, 128, 252],
            ],
        ),
        (
            4,
            &[[255, 30, 0], [255, 102, 0], [255, 166, 0], [255, 226, 0]],
        ),
        (
            5,
            &[
                [230, 39, 255],
                [251, 0, 105],
                [0, 100, 251],
                [255, 255, 255],
            ],
        ),
        (
            6,
            &[
                [249, 73, 0],
                [255, 128, 32],
                [255, 230, 175],
                [255, 216, 61],
            ],
        ),
        (7, &[[49, 125, 0], [160, 235, 54], [255, 253, 130]]),
        (8, &[[255, 39, 39]]),
        (9, &[[255, 255, 255]]),
        (10, &[[255, 241, 155]]),
        (
            11,
            &[
                [255, 27, 27],
                [255, 152, 42],
                [57, 255, 42],
                [42, 162, 255],
                [255, 247, 42],
                [177, 42, 255],
            ],
        ),
        (12, &[[255, 85, 0], [255, 220, 45]]),
        (
            13,
            &[
                [255, 23, 132],
                [255, 84, 197],
                [198, 0, 255],
                [53, 153, 255],
                [255, 255, 255],
            ],
        ),
        (
            14,
            &[[255, 115, 0], [249, 181, 33], [234, 255, 0], [125, 228, 1]],
        ),
    ];
    BODIES
        .iter()
        .map(|(id, colors)| EffectBody {
            id: *id,
            colors: colors.to_vec(),
        })
        .collect()
}
