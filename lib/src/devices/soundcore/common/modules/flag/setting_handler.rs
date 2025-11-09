use std::marker::PhantomData;

use async_trait::async_trait;
use openscq30_lib_has::Has;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::Flag,
    },
};

pub struct FlagSettingHandler<FlagT> {
    setting_id: SettingId,
    _flag: PhantomData<FlagT>,
}

impl<FlagT> FlagSettingHandler<FlagT> {
    pub fn new(setting_id: SettingId) -> Self {
        Self {
            setting_id,
            _flag: PhantomData,
        }
    }
}

#[async_trait]
impl<FlagT, T> SettingHandler<T> for FlagSettingHandler<FlagT>
where
    T: Has<FlagT> + Send,
    FlagT: Flag + Send + Sync,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![self.setting_id]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let flag = state.get();
        Some(Setting::Toggle {
            value: flag.get_bool(),
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
        flag.set_bool(is_enabled);
        Ok(())
    }
}
