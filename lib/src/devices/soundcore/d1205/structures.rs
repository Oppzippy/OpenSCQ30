use std::iter;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::{le_f32, le_u8, le_u16},
};

use crate::devices::soundcore::common::{
    macros::sound_mode_enum,
    modules::sound_modes_v2::ToPacketBody,
    packet::{self, inbound::FromPacketBody, parsing::take_bool},
    structures::{ManualNoiseCanceling, WindNoise},
};

sound_mode_enum!(
    pub enum AmbientSoundMode {
        NoiseCanceling = 0,
        Transparency = 1,
        Normal = 2,
        AdaptiveNoiseCanceling = 3,
    }
);

/// Sound mode block, sent with 06 81 and pushed by the earbuds with 06 01.
///
/// Unlike the D1202, adaptive noise canceling is its own ambient sound mode (3) rather than a noise canceling mode.
/// Bytes that the Soundcore app shows no setting for are preserved as is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    /// High nibble: manual noise canceling strength 1-5
    pub manual_noise_canceling: ManualNoiseCanceling,
    /// Low nibble of the noise canceling byte, always 0xF in captures
    pub unknown_low_nibble: u8,
    /// "Hearguard" toggle shown in transparency mode in the Soundcore app. Not exposed yet, since there is no
    /// matching setting in openscq30.
    pub hearguard: u8,
    /// Always 0 in captures. On the D1202 this byte selects manual/adaptive/multi-scene.
    pub noise_canceling_mode: u8,
    pub wind_noise: WindNoise,
    pub unknown: [u8; 3],
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1205 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    le_u8,
                    le_u8,
                    le_u8,
                    WindNoise::take,
                    take(3usize),
                ),
                |(
                    ambient_sound_mode,
                    noise_canceling,
                    hearguard,
                    noise_canceling_mode,
                    wind_noise,
                    unknown,
                ): (_, u8, _, _, _, &[u8])| Self {
                    ambient_sound_mode,
                    manual_noise_canceling: ManualNoiseCanceling::new(noise_canceling >> 4),
                    unknown_low_nibble: noise_canceling & 0x0F,
                    hearguard,
                    noise_canceling_mode,
                    wind_noise,
                    unknown: unknown.try_into().expect("take(3) returns 3 bytes"),
                },
            ),
        )
        .parse_complete(input)
    }
}

impl SoundModes {
    pub fn bytes(&self) -> impl Iterator<Item = u8> + use<> {
        [
            self.ambient_sound_mode.byte(),
            (self.manual_noise_canceling.inner() << 4) | (self.unknown_low_nibble & 0x0F),
            self.hearguard,
            self.noise_canceling_mode,
            self.wind_noise.byte(),
        ]
        .into_iter()
        .chain(self.unknown)
    }
}

impl ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        Self::bytes(self).collect()
    }
}

/// Spatial audio toggle. The mode byte is preserved since the Soundcore app only offers on/off for this device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SpatialAudio {
    pub is_enabled: bool,
    pub mode: u8,
}

impl SpatialAudio {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "spatial audio",
            map((take_bool, le_u8), |(is_enabled, mode)| Self {
                is_enabled,
                mode,
            }),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 2] {
        [u8::from(self.is_enabled), self.mode]
    }
}

pub const EQUALIZER_BAND_COUNT: usize = 8;
pub const EQUALIZER_BAND_HZ: [u16; EQUALIZER_BAND_COUNT] =
    [100, 200, 400, 800, 1600, 3200, 6400, 12800];
pub const CUSTOM_EQUALIZER_PRESET_ID: u16 = 0xfefe;
pub const CUSTOM_EQUALIZER_Q: f32 = 0.7;

/// One filter of the parametric equalizer.
///
/// The filter type is most likely 0 = low shelf, 1 = peaking, 2 = high shelf, but that is not confirmed.
/// Custom equalizer profiles only use type 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EqualizerFilter {
    pub filter_type: u8,
    pub gain_db: f32,
    pub frequency_hz: f32,
    pub q: f32,
}

impl EqualizerFilter {
    pub const fn new(filter_type: u8, gain_db: f32, frequency_hz: f32, q: f32) -> Self {
        Self {
            filter_type,
            gain_db,
            frequency_hz,
            q,
        }
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> + use<> {
        iter::once(self.filter_type)
            .chain(self.gain_db.to_le_bytes())
            .chain(self.frequency_hz.to_le_bytes())
            .chain(self.q.to_le_bytes())
    }
}

/// Filter lists the Soundcore app sends for the built in presets, captured from the official app.
pub const PRESET_FILTERS: &[(u16, &[EqualizerFilter])] = &[
    (
        0x0000,
        &[
            EqualizerFilter::new(1, 3.0, 30.0, 0.3),
            EqualizerFilter::new(1, 1.0, 50.0, 1.0),
            EqualizerFilter::new(2, 1.5, 7500.0, 0.7),
        ],
    ),
    (
        0x0001,
        &[
            EqualizerFilter::new(0, -1.5, 374.0, 0.7),
            EqualizerFilter::new(1, 1.5, 607.0, 0.3),
            EqualizerFilter::new(2, 2.5, 7024.0, 0.7),
            EqualizerFilter::new(1, 2.5, 3200.0, 0.7),
        ],
    ),
    (
        0x0002,
        &[
            EqualizerFilter::new(1, 6.0, 80.0, 0.5),
            EqualizerFilter::new(1, 2.0, 200.0, 2.5),
            EqualizerFilter::new(1, -1.5, 522.0, 1.3),
            EqualizerFilter::new(1, 1.5, 3900.0, 0.5),
        ],
    ),
    (
        0x0003,
        &[
            EqualizerFilter::new(1, 3.0, 75.0, 0.7),
            EqualizerFilter::new(1, 4.0, 197.0, 2.5),
            EqualizerFilter::new(1, -5.0, 3040.0, 1.3),
            EqualizerFilter::new(2, 1.0, 5600.0, 0.7),
        ],
    ),
];

pub fn preset_filters(preset_id: u16) -> Option<&'static [EqualizerFilter]> {
    PRESET_FILTERS
        .iter()
        .find(|(id, _)| *id == preset_id)
        .map(|(_, filters)| *filters)
}

/// The filters for a custom profile: one peaking filter per band with a fixed Q, gains in tenths of a dB.
pub fn custom_filters(
    volume_adjustments: &[i16; EQUALIZER_BAND_COUNT],
) -> [EqualizerFilter; EQUALIZER_BAND_COUNT] {
    std::array::from_fn(|i| {
        EqualizerFilter::new(
            1,
            f32::from(volume_adjustments[i]) / 10.0,
            f32::from(EQUALIZER_BAND_HZ[i]),
            CUSTOM_EQUALIZER_Q,
        )
    })
}

pub const EQUALIZER_FILTER_SLOTS: usize = 8;
const EQUALIZER_FILTER_SIZE: usize = 13;

/// Equalizer block of the state update packet (145 bytes). Only the preset id and the custom gains are interpreted, the
/// rest is kept as is so the packet can be serialized again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualizerBlock {
    pub preset_id: u16,
    /// Gains of the saved custom profile in tenths of a dB. Kept even while a preset is active.
    pub custom_volume_adjustments: [i16; EQUALIZER_BAND_COUNT],
    /// Unknown 2 bytes followed by what looks like a big endian unix timestamp
    pub unknown: [u8; 6],
    pub filter_count: u8,
    /// The active filters, 8 slots of 13 bytes, unused slots are 0xFF
    pub filter_slots: Vec<u8>,
}

impl Default for EqualizerBlock {
    fn default() -> Self {
        Self {
            preset_id: 0,
            custom_volume_adjustments: [0; EQUALIZER_BAND_COUNT],
            unknown: [0; 6],
            filter_count: 0,
            filter_slots: vec![0xFF; EQUALIZER_FILTER_SLOTS * EQUALIZER_FILTER_SIZE],
        }
    }
}

impl EqualizerBlock {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1205 equalizer",
            map(
                (
                    le_u16,
                    nom::multi::count(le_f32, EQUALIZER_BAND_COUNT),
                    take(6usize),
                    le_u8,
                    take(EQUALIZER_FILTER_SLOTS * EQUALIZER_FILTER_SIZE),
                ),
                |(preset_id, gains, unknown, filter_count, filter_slots): (
                    u16,
                    Vec<f32>,
                    &[u8],
                    u8,
                    &[u8],
                )| Self {
                    preset_id,
                    custom_volume_adjustments: std::array::from_fn(|i| {
                        (gains[i] * 10.0).round() as i16
                    }),
                    unknown: unknown.try_into().expect("take(6) returns 6 bytes"),
                    filter_count,
                    filter_slots: filter_slots.to_vec(),
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> + use<> {
        self.preset_id
            .to_le_bytes()
            .into_iter()
            .chain(
                self.custom_volume_adjustments
                    .into_iter()
                    .flat_map(|v| (f32::from(v) / 10.0).to_le_bytes()),
            )
            .chain(self.unknown)
            .chain(iter::once(self.filter_count))
            .chain(self.filter_slots.clone())
    }
}

/// Gains are in tenths of a dB. The Soundcore app allows -6 dB to +6 dB in 0.1 dB steps.
pub type D1205EqualizerConfiguration =
    crate::devices::soundcore::common::structures::EqualizerConfiguration<1, 8, -60, 60, 1>;
