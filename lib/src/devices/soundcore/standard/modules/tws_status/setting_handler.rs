use async_trait::async_trait;
use openscq30_i18n::Translate;
use strum::IntoEnumIterator;

use crate::{
    api::{
        device,
        settings::{Setting, SettingId, Value},
    },
    devices::soundcore::standard::{settings_manager::SettingHandler, structures::TwsStatus},
    i18n::fl,
};

use super::TwsStatusSetting;

#[derive(Default)]
pub struct TwsStatusSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for TwsStatusSettingHandler
where
    T: AsMut<TwsStatus> + AsRef<TwsStatus> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        TwsStatusSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let tws_status = state.as_ref();
        let setting: TwsStatusSetting = setting_id.try_into().ok()?;
        Some(match setting {
            TwsStatusSetting::HostDevice => Setting::Information {
                text: tws_status.host_device.to_string(),
                translated_text: tws_status.host_device.translate(),
            },
            TwsStatusSetting::TwsStatus => Setting::Information {
                text: if tws_status.is_connected {
                    "Connected".to_owned()
                } else {
                    "Disconnected".to_owned()
                },
                translated_text: if tws_status.is_connected {
                    fl!("connected")
                } else {
                    fl!("disconnected")
                },
            },
        })
    }

    async fn set(
        &self,
        _state: &mut T,
        _setting_id: &SettingId,
        _value: Value,
    ) -> device::Result<()> {
        unimplemented!("battery is read only")
    }
}
