use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::standard::{
        modules::touch_tone::TouchToneSetting,
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::TouchTone,
    },
};

pub struct TouchToneSettingHandler {}

impl TouchToneSettingHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<T> SettingHandler<T> for TouchToneSettingHandler
where
    T: Has<TouchTone> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        TouchToneSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let touch_tone = state.get();
        let setting: TouchToneSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            TouchToneSetting::TouchTone => Setting::Toggle {
                value: (*touch_tone).into(),
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let touch_tone = state.get_mut();
        let setting: TouchToneSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            TouchToneSetting::TouchTone => {
                let is_enabled = value.try_as_bool()?;
                *touch_tone = is_enabled.into();
            }
        }
        Ok(())
    }
}
