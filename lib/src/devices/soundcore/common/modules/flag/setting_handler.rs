use async_trait::async_trait;
use openscq30_lib_has::Has;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::settings_manager::{SettingHandler, SettingHandlerResult},
};

pub struct FlagSettingHandler<Flag> {
    setting_id: SettingId,
    get_flag: fn(&Flag) -> bool,
    set_flag: fn(&mut Flag, bool),
}

impl<Flag> FlagSettingHandler<Flag> {
    pub fn new(
        setting_id: SettingId,
        get_flag: fn(&Flag) -> bool,
        set_flag: fn(&mut Flag, bool),
    ) -> Self {
        Self {
            setting_id,
            get_flag,
            set_flag,
        }
    }
}

#[async_trait]
impl<Flag, T> SettingHandler<T> for FlagSettingHandler<Flag>
where
    T: Has<Flag> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![self.setting_id]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let flag = state.get();
        Some(Setting::Toggle {
            value: (self.get_flag)(flag),
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let flag = state.get_mut();
        let is_enabled = value.try_as_bool()?;
        (self.set_flag)(flag, is_enabled);
        Ok(())
    }
}
