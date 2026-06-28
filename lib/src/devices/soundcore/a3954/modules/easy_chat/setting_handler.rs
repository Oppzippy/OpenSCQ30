use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3954::structures::EasyChat,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::EasyChatSetting;

#[derive(Default)]
pub struct EasyChatSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for EasyChatSettingHandler
where
    T: Has<EasyChat> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        EasyChatSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let easy_chat: &EasyChat = state.get();
        let easy_chat_setting: EasyChatSetting = (*setting_id).try_into().ok()?;
        match easy_chat_setting {
            EasyChatSetting::EasyChat => Some(Setting::Toggle {
                value: easy_chat.is_enabled,
            }),
            EasyChatSetting::EasyChatWaitTime => {
                Some(Setting::select_from_enum_all_variants(easy_chat.wait_time))
            }
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let easy_chat_setting: EasyChatSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match easy_chat_setting {
            EasyChatSetting::EasyChat => {
                let easy_chat = state.get_mut();
                easy_chat.is_enabled = value.try_as_bool()?;
            }
            EasyChatSetting::EasyChatWaitTime => {
                let easy_chat = state.get_mut();
                easy_chat.wait_time = value.try_as_enum_variant()?;
            }
        }
        Ok(())
    }
}
