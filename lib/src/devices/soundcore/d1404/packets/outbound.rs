use std::iter;

use crate::devices::soundcore::{
    common::packet,
    d1404::structures::{
        ButtonConfiguration, D1404EqualizerConfiguration, MfbButtonAction, adjustment_to_decibels,
    },
};

pub const CUSTOM_PRESET_ID: u16 = 0xfefe;

const CUSTOM_BAND_FREQUENCIES: [f32; 8] =
    [100.0, 200.0, 400.0, 800.0, 1600.0, 3200.0, 6400.0, 12800.0];

/// An equalizer filter definition as sent in the set equalizer command and the filter block
/// of the state update packet: gain in dB, frequency in Hz, Q factor, and filter type.
type Filter = (f32, f32, f32, u8);

/// Writes the configuration's byte as it is, so an unknown double press value is kept.
pub fn set_button_double_press_action(
    button_configuration: &ButtonConfiguration,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([0x04, 0x81]),
        vec![0, 0, button_configuration.bytes()[0]],
    )
}

pub fn set_mfb_long_press_action(action: Option<MfbButtonAction>) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([0x04, 0x61]),
        vec![0, 1, action.map_or(15, |action| action as u8)],
    )
}

/// The filter set and unknown byte (byte right after the filter count) that the official app
/// sends for each preset. The last filter's type byte sits where packet captures had the
/// checksum stripped, so 6 (the type the state update uses for the last filter) is assumed.
fn preset_filters(preset_id: u16) -> (u8, &'static [Filter]) {
    match preset_id {
        1 => (
            0,
            &[
                (-0.9, 374.0, 0.7, 1),
                (2.4, 607.0, 0.3, 2),
                (3.0, 7024.0, 0.7, 1),
                (5.0, 3200.0, 0.7, 6),
            ],
        ),
        2 => (
            1,
            &[
                (6.0, 80.0, 0.5, 1),
                (2.0, 200.0, 2.5, 1),
                (-1.5, 522.0, 1.3, 1),
                (1.5, 3900.0, 0.5, 6),
            ],
        ),
        3 => (
            1,
            &[
                (3.0, 75.0, 0.7, 1),
                (4.0, 197.0, 2.5, 1),
                (-5.0, 3040.0, 1.3, 2),
                (1.0, 5600.0, 0.7, 6),
            ],
        ),
        // The Soundcore signature preset (0), and a fallback for unknown preset ids.
        _ => (
            1,
            &[
                (3.0, 30.0, 0.3, 1),
                (1.0, 50.0, 1.0, 2),
                (1.5, 7500.0, 0.7, 6),
            ],
        ),
    }
}

pub fn custom_filters(band_adjustments: &[i16]) -> Vec<Filter> {
    band_adjustments
        .iter()
        .enumerate()
        .map(|(index, adjustment)| {
            (
                adjustment_to_decibels(*adjustment),
                CUSTOM_BAND_FREQUENCIES[index],
                0.7,
                if index == band_adjustments.len() - 1 {
                    6
                } else {
                    1
                },
            )
        })
        .collect()
}

pub fn filter_bytes(filters: &[Filter]) -> impl Iterator<Item = u8> {
    filters.iter().flat_map(|(gain, frequency, q, kind)| {
        gain.to_le_bytes()
            .into_iter()
            .chain(frequency.to_le_bytes())
            .chain(q.to_le_bytes())
            .chain(iter::once(*kind))
    })
}

/// The set equalizer command's filter section body: filter count, unknown byte, and the
/// filter definitions. With a custom profile the filters mirror the equalizer band values;
/// presets have a fixed filter set the app writes out in full.
pub fn filter_section(equalizer_configuration: &D1404EqualizerConfiguration) -> Vec<u8> {
    let (unknown_byte, filters): (u8, Vec<Filter>) =
        if equalizer_configuration.preset_id() == CUSTOM_PRESET_ID {
            (
                1,
                custom_filters(
                    equalizer_configuration
                        .volume_adjustments_channel_1()
                        .copied()
                        .unwrap_or_default()
                        .adjustments(),
                ),
            )
        } else {
            let (unknown_byte, preset) = preset_filters(equalizer_configuration.preset_id());
            (unknown_byte, preset.to_vec())
        };
    std::iter::once(filters.len() as u8)
        .chain(iter::once(unknown_byte))
        .chain(filter_bytes(&filters))
        .collect()
}

pub fn set_equalizer(equalizer_configuration: &D1404EqualizerConfiguration) -> packet::Outbound {
    let body = equalizer_configuration
        .preset_id()
        .to_le_bytes()
        .into_iter()
        .chain([0, 0, 0, 0, 0, 0]) // unknown
        .chain(filter_section(equalizer_configuration))
        .collect();
    packet::Outbound::new(packet::Command([0x03, 0x8A]), body)
}
