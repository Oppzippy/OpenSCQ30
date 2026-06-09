use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::{AmbientSoundModeCycleTws, TwsStatus},
    },
};

use super::SoundModeCycleSetting;

pub struct AmbientSoundModeCycleTwsSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for AmbientSoundModeCycleTwsSettingHandler
where
    T: Has<TwsStatus> + Has<AmbientSoundModeCycleTws> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SoundModeCycleSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let cycle = state.get();
        let tws_status: &TwsStatus = state.get();
        get_inner(tws_status.is_connected, cycle, setting_id)
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let tws_status: &TwsStatus = state.get();
        let is_tws_connected = tws_status.is_connected;
        let cycle = state.get_mut();
        set_inner(is_tws_connected, cycle, setting_id, value)
    }
}

#[inline(never)]
fn get_inner(
    is_tws_connected: bool,
    tws_cycle: &AmbientSoundModeCycleTws,
    setting_id: &SettingId,
) -> Option<Setting> {
    let setting: SoundModeCycleSetting = (*setting_id).try_into().ok()?;
    let cycle = if is_tws_connected {
        tws_cycle.tws_enabled
    } else {
        tws_cycle.tws_disabled
    };
    Some(match setting {
        SoundModeCycleSetting::NormalModeInCycle => Setting::Toggle {
            value: cycle.normal_mode,
        },
        SoundModeCycleSetting::TransparencyModeInCycle => Setting::Toggle {
            value: cycle.transparency_mode,
        },
        SoundModeCycleSetting::NoiseCancelingModeInCycle => Setting::Toggle {
            value: cycle.noise_canceling_mode,
        },
    })
}

#[inline(never)]
fn set_inner(
    is_tws_connected: bool,
    tws_cycle: &mut AmbientSoundModeCycleTws,
    setting_id: &SettingId,
    value: Value,
) -> SettingHandlerResult<()> {
    let setting: SoundModeCycleSetting = (*setting_id)
        .try_into()
        .expect("already filtered to valid values only by SettingsManager");
    let cycle = if is_tws_connected {
        &mut tws_cycle.tws_enabled
    } else {
        &mut tws_cycle.tws_disabled
    };
    match setting {
        SoundModeCycleSetting::NormalModeInCycle => {
            cycle.normal_mode = value.try_as_bool()?;
        }
        SoundModeCycleSetting::TransparencyModeInCycle => {
            cycle.transparency_mode = value.try_as_bool()?;
        }
        SoundModeCycleSetting::NoiseCancelingModeInCycle => {
            cycle.noise_canceling_mode = value.try_as_bool()?;
        }
    }
    Ok(())
}
