use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3876::structures::VolumeBalance,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
    settings,
};

use super::VolumeBalanceSetting;

#[derive(Default)]
pub struct VolumeBalanceSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for VolumeBalanceSettingHandler
where
    T: Has<VolumeBalance> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        VolumeBalanceSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let volume_balance: &VolumeBalance = state.get();
        let volume_balance_setting: VolumeBalanceSetting = (*setting_id).try_into().ok()?;
        match volume_balance_setting {
            VolumeBalanceSetting::VolumeBalance => Some(Setting::I32Range {
                setting: settings::Range {
                    range: -6..=6,
                    step: 1,
                },
                value: volume_balance.inner().into(),
            }),
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let volume_balance_setting: VolumeBalanceSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match volume_balance_setting {
            VolumeBalanceSetting::VolumeBalance => {
                let volume_balance = state.get_mut();
                *volume_balance = VolumeBalance::new(
                    i8::try_from(value.try_as_i32()?.clamp(-6, 6)).expect("-6 to 6 fits in i8"),
                );
            }
        }
        Ok(())
    }
}
