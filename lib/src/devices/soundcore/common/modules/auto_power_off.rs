use std::sync::Arc;

use openscq30_i18n::Translate;
use openscq30_lib_has::MaybeHas;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::common::{
        modules::{
            ModuleCollection,
            auto_power_off::{
                setting_handler::AutoPowerOffSettingHandler,
                state_modifier::AutoPowerOffStateModifier,
            },
        },
        packet::PacketIOController,
        structures::AutoPowerOff,
    },
    i18n::fl,
    macros::enum_subset,
};

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumIter, EnumString)]
    enum AutoPowerOffSetting {
        AutoPowerOff,
    }
);

impl<T> ModuleCollection<T>
where
    T: MaybeHas<AutoPowerOff> + Clone + Send + Sync + 'static,
{
    pub fn add_auto_power_off<C, Duration>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        durations: &'static [Duration],
    ) where
        C: RfcommConnection + 'static + Send + Sync,
        Duration: Translate + Send + Sync + 'static,
        &'static str: for<'a> From<&'a Duration>,
    {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            AutoPowerOffSettingHandler::new(durations),
        );
        self.state_modifiers
            .push(Box::new(AutoPowerOffStateModifier::new(packet_io)));
    }
}

#[derive(IntoStaticStr)]
#[allow(clippy::enum_variant_names)]
pub enum AutoPowerOffDuration {
    #[strum(serialize = "10m")]
    TenMinutes,
    #[strum(serialize = "20m")]
    TwentyMinutes,
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
    #[strum(serialize = "90m")]
    NinetyMinutes,
    #[strum(serialize = "120m")]
    OneHundredTwentyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            Self::TenMinutes => fl!("x-minutes", minutes = 10),
            Self::TwentyMinutes => fl!("x-minutes", minutes = 20),
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
            Self::NinetyMinutes => fl!("x-minutes", minutes = 90),
            Self::OneHundredTwentyMinutes => fl!("x-minutes", minutes = 120),
        }
    }
}

impl AutoPowerOffDuration {
    pub fn half_hour_increments() -> &'static [Self] {
        &[
            Self::ThirtyMinutes,
            Self::SixtyMinutes,
            Self::NinetyMinutes,
            Self::OneHundredTwentyMinutes,
        ]
    }

    pub fn ten_twenty_thirty_sixty() -> &'static [Self] {
        &[
            Self::TenMinutes,
            Self::TwentyMinutes,
            Self::ThirtyMinutes,
            Self::SixtyMinutes,
        ]
    }
}
