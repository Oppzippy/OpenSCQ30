use std::marker::PhantomData;

use async_trait::async_trait;
use openscq30_lib_has::MaybeHas;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
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
    T: MaybeHas<FlagT> + Send,
    FlagT: Flag + Send + Sync,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![self.setting_id]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        state.maybe_get().map(|flag| Setting::Toggle {
            value: flag.get_bool(),
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        if let Some(flag) = state.maybe_get_mut() {
            let is_enabled = value.try_as_bool()?;
            flag.set_bool(is_enabled);
            Ok(())
        } else {
            Err(SettingHandlerError::MissingData)
        }
    }
}
