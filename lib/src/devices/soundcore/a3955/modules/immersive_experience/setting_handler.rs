use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3955::structures::ImmersiveExperience,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::ImmersiveExperienceSetting;

#[derive(Default)]
pub struct ImmersiveExperienceSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for ImmersiveExperienceSettingHandler
where
    T: Has<ImmersiveExperience> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ImmersiveExperienceSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let immersive_experience: &ImmersiveExperience = state.get();
        let setting: ImmersiveExperienceSetting = (*setting_id).try_into().ok()?;
        match setting {
            ImmersiveExperienceSetting::ImmersiveExperience => Some(
                Setting::select_from_enum_all_variants(*immersive_experience),
            ),
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let setting: ImmersiveExperienceSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            ImmersiveExperienceSetting::ImmersiveExperience => {
                *state.get_mut() = value.try_as_enum_variant()?;
            }
        }
        Ok(())
    }
}
