use async_trait::async_trait;
use openscq30_lib_has::MaybeHas;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::{
        a3116::{self, modules::volume::VolumeSetting},
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
};

pub struct VolumeSettingHandler {
    max_volume: u8,
}

impl VolumeSettingHandler {
    pub fn new(max_volume: u8) -> Self {
        Self { max_volume }
    }
}

#[async_trait]
impl<T> SettingHandler<T> for VolumeSettingHandler
where
    T: MaybeHas<a3116::structures::Volume> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        VolumeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let volume = state.maybe_get()?;
        let setting: VolumeSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            VolumeSetting::Volume => Setting::I32Range {
                setting: settings::Range {
                    range: 0..=self.max_volume.into(),
                    step: 1,
                },
                value: volume.0.into(),
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let volume = state
            .maybe_get_mut()
            .ok_or(SettingHandlerError::MissingData)?;
        let setting: VolumeSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            VolumeSetting::Volume => {
                *volume = a3116::structures::Volume(
                    value.try_as_i32()?.clamp(0, self.max_volume.into()) as u8,
                );
            }
        }
        Ok(())
    }
}
