use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3954::structures::CaseLanguage,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::CaseLanguageSetting;

pub struct CaseLanguageSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for CaseLanguageSettingHandler
where
    T: Has<CaseLanguage> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        CaseLanguageSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let language: &CaseLanguage = state.get();
        let language_setting: CaseLanguageSetting = (*setting_id).try_into().ok()?;
        match language_setting {
            CaseLanguageSetting::CaseLanguage => {
                Some(Setting::select_from_enum_all_variants(*language))
            }
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let language_setting: CaseLanguageSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match language_setting {
            CaseLanguageSetting::CaseLanguage => {
                *state.get_mut() = value.try_as_enum_variant()?;
            }
        }
        Ok(())
    }
}
