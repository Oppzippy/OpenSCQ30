use nom::{
    IResult,
    error::{ContextError, ParseError},
    number::complete::le_u8,
};
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::SettingId,
    devices::soundcore::common::{packet::parsing::take_bool, structures::TwsStatus},
    macros::enum_subset,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ButtonStatusCollection<const N: usize>(pub [ButtonStatus; N]);

impl<const N: usize> ButtonStatusCollection<N> {
    pub fn new(statuses: [ButtonStatus; N]) -> Self {
        Self(statuses)
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>, const NUM_BUTTONS: usize>(
        buttons: [ButtonParseSettings; NUM_BUTTONS],
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E> {
        move |mut input| {
            // Use std::array::try_from_fn once stabilized
            // https://doc.rust-lang.org/std/array/fn.try_from_fn.html
            let mut statuses = Vec::with_capacity(NUM_BUTTONS);
            for parse_settings in buttons {
                let status;
                (input, status) = ButtonStatus::take(parse_settings)(input)?;
                statuses.push(status);
            }

            Ok((
                input,
                Self(
                    statuses
                        .try_into()
                        .expect("should have the same size as parse_settings"),
                ),
            ))
        }
    }

    pub fn bytes(&self, parse_settings: [ButtonParseSettings; N]) -> impl Iterator<Item = u8> {
        self.0
            .iter()
            .zip(parse_settings)
            .flat_map(move |(status, settings)| status.bytes(settings))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ButtonStatus {
    pub enabled: Option<EnabledStatus>,
    pub action: ActionStatus,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ButtonParseSettings {
    pub enabled_flag_kind: EnabledFlagKind,
    pub action_kind: ActionKind,
}

impl ButtonStatus {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        parse_settings: ButtonParseSettings,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E> {
        move |input| {
            let (input, enabled) = EnabledStatus::take(parse_settings.enabled_flag_kind)(input)?;
            let (input, action) = ActionStatus::take(parse_settings.action_kind)(input)?;

            Ok((input, Self { enabled, action }))
        }
    }

    pub fn bytes(&self, parse_settings: ButtonParseSettings) -> impl Iterator<Item = u8> {
        self.enabled
            .map(|enabled| enabled.byte(parse_settings.enabled_flag_kind))
            .into_iter()
            .chain(std::iter::once(
                self.action.byte(parse_settings.action_kind),
            ))
    }

    pub fn is_enabled(&self, tws_status: TwsStatus) -> bool {
        self.enabled.map_or_else(
            || self.action.current(tws_status) != 0xF,
            |e| e.current(tws_status),
        )
    }

    pub fn current_action_id(&self, tws_status: TwsStatus) -> Option<u8> {
        if self.is_enabled(tws_status) {
            Some(self.action.current(tws_status))
        } else {
            None
        }
    }

    pub fn with_current_action_id(
        self,
        tws_status: TwsStatus,
        maybe_action_id: Option<u8>,
    ) -> Self {
        if let Some(action_id) = maybe_action_id {
            Self {
                enabled: self
                    .enabled
                    .map(|enabled| enabled.with_current(tws_status, true)),
                action: self.action.with_current_action_id(tws_status, action_id),
            }
        } else {
            Self {
                enabled: self
                    .enabled
                    .map(|enabled| enabled.with_current(tws_status, false)),
                action: if self.enabled.is_some() {
                    self.action
                } else {
                    self.action.with_current_action_id(tws_status, 0xF)
                },
            }
        }
    }
}

enum_subset!(
    SettingId,
    #[derive(Copy, Clone, Eq, PartialEq, Debug, EnumString, EnumIter, IntoStaticStr)]
    #[allow(clippy::enum_variant_names)]
    pub enum Button {
        LeftSinglePress,
        RightSinglePress,
        LeftDoublePress,
        RightDoublePress,
        LeftTriplePress,
        RightTriplePress,
        LeftLongPress,
        RightLongPress,
    }
);

impl Button {
    pub fn press_kind(&self) -> ButtonPressKind {
        match self {
            Self::LeftSinglePress | Self::RightSinglePress => ButtonPressKind::Single,
            Self::LeftDoublePress | Self::RightDoublePress => ButtonPressKind::Double,
            Self::LeftTriplePress | Self::RightTriplePress => ButtonPressKind::Triple,
            Self::LeftLongPress | Self::RightLongPress => ButtonPressKind::Long,
        }
    }

    pub fn side(&self) -> ButtonSide {
        match self {
            Self::LeftSinglePress
            | Self::LeftDoublePress
            | Self::LeftTriplePress
            | Self::LeftLongPress => ButtonSide::Left,
            Self::RightSinglePress
            | Self::RightDoublePress
            | Self::RightTriplePress
            | Self::RightLongPress => ButtonSide::Right,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ButtonSide {
    Left,
    Right,
}

impl From<ButtonSide> for u8 {
    fn from(side: ButtonSide) -> Self {
        match side {
            ButtonSide::Left => 0,
            ButtonSide::Right => 1,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonPressKind {
    Single,
    Double,
    Triple,
    Long,
}

impl ButtonPressKind {
    pub fn left_button(&self) -> Button {
        match self {
            Self::Single => Button::LeftSinglePress,
            Self::Double => Button::LeftDoublePress,
            Self::Triple => Button::LeftTriplePress,
            Self::Long => Button::LeftLongPress,
        }
    }

    pub fn right_button(&self) -> Button {
        match self {
            Self::Single => Button::RightSinglePress,
            Self::Double => Button::RightDoublePress,
            Self::Triple => Button::RightTriplePress,
            Self::Long => Button::RightLongPress,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum EnabledFlagKind {
    None,
    Single,
    TwsLowBits,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum EnabledStatus {
    Single(bool),
    Tws { connected: bool, disconnected: bool },
}

impl EnabledStatus {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        enabled_flag_kind: EnabledFlagKind,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<Self>, E> {
        move |input| {
            Ok(match enabled_flag_kind {
                EnabledFlagKind::None => (input, None),
                EnabledFlagKind::Single => {
                    let (input, is_enabled) = take_bool(input)?;
                    (input, Some(Self::Single(is_enabled)))
                }
                EnabledFlagKind::TwsLowBits => {
                    let (input, is_enabled) = le_u8(input)?;
                    (
                        input,
                        Some(Self::Tws {
                            connected: is_enabled & 0xF == 1,
                            disconnected: is_enabled >> 4 == 1,
                        }),
                    )
                }
            })
        }
    }

    pub fn byte(self, enabled_flag_kind: EnabledFlagKind) -> u8 {
        match self {
            Self::Single(is_enabled) => is_enabled.into(),
            Self::Tws {
                connected,
                disconnected,
            } => match enabled_flag_kind {
                EnabledFlagKind::None => unreachable!(),
                EnabledFlagKind::Single => unreachable!(),
                EnabledFlagKind::TwsLowBits => (u8::from(disconnected) << 4) | u8::from(connected),
            },
        }
    }

    pub fn current(self, tws_status: TwsStatus) -> bool {
        match self {
            Self::Single(is_enabled) => is_enabled,
            Self::Tws {
                connected,
                disconnected,
            } => {
                if tws_status.is_connected {
                    connected
                } else {
                    disconnected
                }
            }
        }
    }

    pub fn with_current(self, tws_status: TwsStatus, is_enabled: bool) -> Self {
        match self {
            Self::Single(_) => Self::Single(is_enabled),
            Self::Tws {
                connected,
                disconnected,
            } => {
                if tws_status.is_connected {
                    Self::Tws {
                        connected: is_enabled,
                        disconnected,
                    }
                } else {
                    Self::Tws {
                        connected,
                        disconnected: is_enabled,
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ActionKind {
    Single,
    TwsLowBits,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ActionStatus {
    Single(u8),
    Tws { connected: u8, disconnected: u8 },
}

impl ActionStatus {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        tws_action_kind: ActionKind,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E> {
        move |input| {
            Ok(match tws_action_kind {
                ActionKind::Single => {
                    let (input, action_id) = le_u8(input)?;
                    (input, Self::Single(action_id))
                }
                ActionKind::TwsLowBits => {
                    let (input, action_ids) = le_u8(input)?;
                    (
                        input,
                        Self::Tws {
                            connected: action_ids & 0xF,
                            disconnected: action_ids >> 4,
                        },
                    )
                }
            })
        }
    }

    pub fn byte(self, action_kind: ActionKind) -> u8 {
        match action_kind {
            ActionKind::Single => match self {
                Self::Single(byte) => byte,
                Self::Tws { .. } => unreachable!(),
            },
            ActionKind::TwsLowBits => match self {
                Self::Single(_) => unreachable!(),
                Self::Tws {
                    connected,
                    disconnected,
                } => (disconnected << 4) | connected,
            },
        }
    }

    pub fn current(self, tws_status: TwsStatus) -> u8 {
        match self {
            Self::Single(action) => action,
            Self::Tws {
                connected,
                disconnected,
            } => {
                if tws_status.is_connected {
                    connected
                } else {
                    disconnected
                }
            }
        }
    }

    pub fn with_current_action_id(self, tws_status: TwsStatus, action_id: u8) -> Self {
        match self {
            Self::Single(_) => Self::Single(action_id),
            Self::Tws {
                connected,
                disconnected,
            } => {
                if tws_status.is_connected {
                    Self::Tws {
                        connected: action_id,
                        disconnected,
                    }
                } else {
                    Self::Tws {
                        connected,
                        disconnected: action_id,
                    }
                }
            }
        }
    }
}
