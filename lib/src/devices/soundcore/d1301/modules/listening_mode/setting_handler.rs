use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        common::settings_manager::{SettingHandler, SettingHandlerResult},
        d1301::structures::{DefaultListeningMode, ListeningMode},
    },
};

use super::ListeningModeSetting;

#[derive(Default)]
pub struct ListeningModeSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for ListeningModeSettingHandler
where
    T: Has<ListeningMode> + Has<DefaultListeningMode> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ListeningModeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let listening_mode_setting: ListeningModeSetting = (*setting_id).try_into().ok()?;

        match listening_mode_setting {
            ListeningModeSetting::ListeningMode => {
                let listening_mode: &ListeningMode = state.get();
                Some(Setting::select_from_enum_all_variants(*listening_mode))
            }
            ListeningModeSetting::DefaultListeningMode => {
                let listening_mode: &DefaultListeningMode = state.get();
                Some(Setting::select_from_enum_all_variants(listening_mode.0))
            }
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let spatial_audio_setting: ListeningModeSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match spatial_audio_setting {
            ListeningModeSetting::ListeningMode => {
                let listening_mode: &mut ListeningMode = state.get_mut();
                *listening_mode = value.try_as_enum_variant()?;
            }
            ListeningModeSetting::DefaultListeningMode => {
                let listening_mode: &mut DefaultListeningMode = state.get_mut();
                *listening_mode = DefaultListeningMode(value.try_as_enum_variant()?);
            }
        }
        Ok(())
    }
}
