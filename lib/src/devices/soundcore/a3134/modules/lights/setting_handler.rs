use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3134::structures::{Lights, LightsBrightness},
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
    settings,
};

use super::LightsSetting;

pub struct LightsSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for LightsSettingHandler
where
    T: Has<Lights> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        LightsSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let lights: &Lights = state.get();
        let lights_setting: LightsSetting = (*setting_id).try_into().ok()?;
        Some(match lights_setting {
            LightsSetting::LightsEnabled => Setting::Toggle {
                value: lights.is_enabled,
            },
            LightsSetting::LightsBrightness => Setting::I32Range {
                setting: settings::Range {
                    range: LightsBrightness::MIN.into()..=LightsBrightness::MAX.into(),
                    step: 1,
                },
                value: lights.brightness.inner().into(),
            },
            LightsSetting::LightsMode => Setting::select_from_enum_all_variants(lights.effect),
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let lights_setting: LightsSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        let lights: &mut Lights = state.get_mut();
        match lights_setting {
            LightsSetting::LightsEnabled => lights.is_enabled = value.try_as_bool()?,
            LightsSetting::LightsBrightness => {
                lights.brightness = LightsBrightness::new(
                    u8::try_from(
                        value
                            .try_as_i32()?
                            .clamp(LightsBrightness::MIN.into(), LightsBrightness::MAX.into()),
                    )
                    .expect("clamped to the u8 range"),
                );
            }
            LightsSetting::LightsMode => lights.effect = value.try_as_enum_variant()?,
        }
        Ok(())
    }
}
