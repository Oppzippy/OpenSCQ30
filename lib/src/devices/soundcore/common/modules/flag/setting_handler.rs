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
    is_inverted: bool,
    _flag: PhantomData<FlagT>,
}

impl<FlagT> FlagSettingHandler<FlagT> {
    pub fn new(setting_id: SettingId, is_inverted: bool) -> Self {
        Self {
            setting_id,
            is_inverted,
            _flag: PhantomData,
        }
    }

    pub fn invert_if_needed(&self, value: bool) -> bool {
        if self.is_inverted { !value } else { value }
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
            value: self.invert_if_needed(flag.get_bool()),
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        if let Some(flag) = state.maybe_get_mut() {
            let is_enabled = self.invert_if_needed(value.try_as_bool()?);
            flag.set_bool(is_enabled);
            Ok(())
        } else {
            Err(SettingHandlerError::MissingData)
        }
    }
}
