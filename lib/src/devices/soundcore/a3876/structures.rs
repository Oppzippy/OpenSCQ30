use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::{macros::sound_mode_enum, packet::parsing::take_bool};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct VolumeBalance(i8);

impl VolumeBalance {
    pub fn new(balance: i8) -> Self {
        Self(balance.clamp(-6, 6))
    }

    pub fn inner(self) -> i8 {
        self.0
    }

    pub fn bytes(self) -> impl Iterator<Item = u8> {
        std::iter::once(self.byte())
    }

    pub fn byte(self) -> u8 {
        u8::try_from(self.0 + 100).expect("range is 94-106, which fits in u8")
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::new(i8::try_from(b).unwrap_or(100) - 100)).parse_complete(input)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct ColorfulLights {
    pub is_enabled: bool,
    pub brightness: ColorfulLightsBrightness,
    pub auto_lights_off_duration: AutoLightsOffDurationInMinutes,
    pub mode: ColorfulLightsMode,
    pub color: RgbColor,
}

impl ColorfulLights {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "colorful lights",
            map(
                (
                    take_bool,
                    ColorfulLightsBrightness::take,
                    AutoLightsOffDurationInMinutes::take,
                    ColorfulLightsMode::take,
                    RgbColor::take,
                ),
                |(is_enabled, brightness, auto_lights_off_duration, mode, color)| Self {
                    is_enabled,
                    brightness,
                    auto_lights_off_duration,
                    color,
                    mode,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [
            u8::from(self.is_enabled),
            self.brightness.inner(),
            self.auto_lights_off_duration.inner(),
            self.mode.byte(),
        ]
        .into_iter()
        .chain(self.color.bytes())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ColorfulLightsBrightness(u8);

impl Default for ColorfulLightsBrightness {
    fn default() -> Self {
        Self::new(1)
    }
}

impl ColorfulLightsBrightness {
    pub fn new(brightness: u8) -> Self {
        Self(brightness.clamp(1, 10))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AutoLightsOffDurationInMinutes(u8);

impl Default for AutoLightsOffDurationInMinutes {
    fn default() -> Self {
        Self::new(1)
    }
}

impl AutoLightsOffDurationInMinutes {
    pub fn new(brightness: u8) -> Self {
        Self(brightness.clamp(1, 120))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColor {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map((le_u8, le_u8, le_u8), |(r, g, b)| Self {
            red: r,
            green: g,
            blue: b,
        })
        .parse_complete(input)
    }

    pub fn bytes(self) -> impl Iterator<Item = u8> {
        [self.red, self.green, self.blue].into_iter()
    }
}

sound_mode_enum!(
    pub enum ColorfulLightsMode {
        LightOn = 0,
        Breathing = 1,
        Flash = 2,
        MusicFollows = 3,
    }
);
