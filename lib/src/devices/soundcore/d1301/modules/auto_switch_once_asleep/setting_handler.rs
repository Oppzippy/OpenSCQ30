use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        common::settings_manager::{SettingHandler, SettingHandlerResult},
        d1301::structures::{AutoStopTimer, AutoSwitchOnceAsleep, PostSleepAudio},
    },
};

use super::AutoSwitchOnceAsleepSetting;

/// The three choices the official app offers. They are the three reachable
/// combinations of the enable flag and the post-sleep audio action; with the
/// feature off the action is retained but ignored.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    strum::IntoStaticStr,
    strum::EnumString,
    strum::EnumIter,
    strum::VariantArray,
    openscq30_i18n_macros::Translate,
)]
// Named after the app's labels, which keeps the translation keys specific.
#[allow(clippy::enum_variant_names)]
pub enum AutoSwitchOnceAsleepValue {
    #[default]
    KeepAudio,
    PauseAudio,
    PlayLocalAudio,
}

impl AutoSwitchOnceAsleepValue {
    fn from_state(enabled: bool, post_sleep_audio: u8) -> Self {
        match (enabled, post_sleep_audio) {
            (false, _) => Self::KeepAudio,
            (true, action) if action == PostSleepAudio::Pause as u8 => Self::PauseAudio,
            (true, _) => Self::PlayLocalAudio,
        }
    }

    /// The action to write, or `None` to leave it as it is.
    fn post_sleep_audio(self) -> Option<PostSleepAudio> {
        match self {
            Self::KeepAudio => None,
            Self::PauseAudio => Some(PostSleepAudio::Pause),
            Self::PlayLocalAudio => Some(PostSleepAudio::PlayLocal),
        }
    }
}

#[derive(Default)]
pub struct AutoSwitchOnceAsleepSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for AutoSwitchOnceAsleepSettingHandler
where
    T: Has<AutoSwitchOnceAsleep> + Has<AutoStopTimer> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        AutoSwitchOnceAsleepSetting::iter()
            .map(Into::into)
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let _: AutoSwitchOnceAsleepSetting = (*setting_id).try_into().ok()?;
        let enabled: &AutoSwitchOnceAsleep = state.get();
        let auto_stop_timer: &AutoStopTimer = state.get();
        Some(Setting::select_from_enum_all_variants(
            AutoSwitchOnceAsleepValue::from_state(enabled.0, auto_stop_timer.post_sleep_audio),
        ))
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let selected: AutoSwitchOnceAsleepValue = value.try_as_enum_variant()?;

        // Keep Audio only clears the enable flag, so re-enabling restores the
        // previous action.
        if let Some(action) = selected.post_sleep_audio() {
            let auto_stop_timer: &mut AutoStopTimer = state.get_mut();
            auto_stop_timer.post_sleep_audio = action as u8;
        }
        let enabled: &mut AutoSwitchOnceAsleep = state.get_mut();
        enabled.0 = selected != AutoSwitchOnceAsleepValue::KeepAudio;
        Ok(())
    }
}
