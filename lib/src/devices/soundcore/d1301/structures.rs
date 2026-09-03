use std::iter;

use bitflags::bitflags;
use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::{le_i16, le_i32, le_u8},
};

use crate::devices::soundcore::common::{
    macros::sound_mode_enum, packet::parsing::take_bool, structures::flag,
};

flag!(AutoPowerOffPrompt);
flag!(ListeningModePrompt);
flag!(NoiseCanceling);
flag!(IncomingCallsDuringBluetoothMode);
flag!(TapControlsDisabled);
flag!(NoiseCancelingPrompt);
flag!(AutoSwitchOnceAsleep);

/// What the earbuds do with audio once they detect you are asleep. Only
/// meaningful while [`AutoSwitchOnceAsleep`] is enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::FromRepr)]
#[repr(u8)]
pub enum PostSleepAudio {
    Pause = 0,
    #[default]
    PlayLocal = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DefaultListeningMode(pub ListeningMode);

impl DefaultListeningMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(ListeningMode::take, Self).parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        self.0.bytes()
    }
}

sound_mode_enum!(
    pub enum ListeningMode {
        Bluetooth = 0,
        Local = 1,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AutoStopTimer {
    pub is_enabled: bool,
    pub duration_in_minutes: i16,
    /// The "Auto-Switch Once Asleep" action. Shares this packet with the timer
    /// but is independent of it. Raw so an unknown value cannot fail the parse.
    pub post_sleep_audio: u8,
    pub time_left_in_seconds: i32,
}

impl AutoStopTimer {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "auto stop timer",
            map(
                (take_bool, le_i16, le_u8, le_i32),
                |(is_enabled, duration_in_minutes, post_sleep_audio, time_left_in_seconds)| Self {
                    is_enabled,
                    duration_in_minutes,
                    post_sleep_audio,
                    time_left_in_seconds,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        iter::once(u8::from(self.is_enabled))
            .chain(self.duration_in_minutes.to_le_bytes())
            .chain(iter::once(self.post_sleep_audio))
            .chain(self.time_left_in_seconds.to_le_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alarm {
    pub id: u8,
    pub is_enabled: bool,
    pub time: i16,
    pub repeat: AlarmRepeat,
    pub wake_up_tune: AlarmWakeUpTune,
    pub volume: AlarmVolume,
    pub snooze_duration_in_minutes: u8,
}

impl Alarm {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "alarm",
            map(
                (
                    le_u8,
                    take_bool,
                    le_i16,
                    AlarmRepeat::take,
                    AlarmWakeUpTune::take,
                    AlarmVolume::take,
                    le_u8,
                ),
                |(
                    id,
                    is_enabled,
                    time,
                    repeat,
                    wake_up_tune,
                    volume,
                    snooze_duration_in_minutes,
                )| Self {
                    id,
                    is_enabled,
                    time,
                    repeat,
                    wake_up_tune,
                    volume,
                    snooze_duration_in_minutes,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        iter::once(self.id)
            .chain(iter::once(u8::from(self.is_enabled)))
            .chain(self.time.to_le_bytes())
            .chain(iter::once(self.repeat.bits()))
            .chain(self.wake_up_tune.bytes())
            .chain(iter::once(self.volume.inner()))
            .chain(iter::once(self.snooze_duration_in_minutes))
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AlarmRepeat: u8 {
        const SUNDAY = 1 << 7;
        const MONDAY = 1 << 6;
        const TUESDAY = 1 << 5;
        const WEDNESDAY = 1 << 4;
        const THURSDAY = 1 << 3;
        const FRIDAY = 1 << 2;
        const SATURDAY =  1 << 1;
    }
}

impl AlarmRepeat {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, Self::from_bits_truncate).parse_complete(input)
    }
}

sound_mode_enum!(
    pub enum AlarmWakeUpTune {
        Nature = 0,
        Glow = 1,
        Rhythm = 2,
        Bloom = 3,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlarmVolume(u8);

impl AlarmVolume {
    pub fn new(volume: u8) -> Self {
        Self(volume.clamp(0, 100))
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
