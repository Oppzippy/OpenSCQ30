use std::{borrow::Cow, iter, str::FromStr};

use async_trait::async_trait;
use openscq30_i18n::Translate;
use openscq30_i18n_macros::Translate;
use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        d1301::{
            modules::alarms::{GeneralAlarmSettingId, PerAlarmSettingId},
            structures::{Alarm, AlarmRepeat, AlarmVolume},
        },
    },
    settings,
};

const MAX_ALARMS: usize = 5;

#[derive(Default)]
pub struct AlarmsSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for AlarmsSettingHandler
where
    T: Has<Vec<Alarm>> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        PerAlarmSettingId::iter()
            .map(Into::into)
            .chain(iter::once(GeneralAlarmSettingId::CreateAlarm.into()))
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let alarms = state.get();

        let maybe_general_alarm_setting_id: Option<GeneralAlarmSettingId> =
            (*setting_id).try_into().ok();
        if let Some(general_alarm_setting_id) = maybe_general_alarm_setting_id {
            return match general_alarm_setting_id {
                GeneralAlarmSettingId::CreateAlarm => Some(Setting::Action),
            };
        }

        let alarms_setting_id: PerAlarmSettingId = (*setting_id).try_into().ok()?;
        let alarm_setting = AlarmSetting::from(alarms_setting_id);
        let alarm = alarms.get(alarm_setting.alarm_index)?;

        match alarm_setting.setting_kind {
            AlarmSettingKind::Delete => Some(Setting::Action),
            AlarmSettingKind::Enabled => Some(Setting::Toggle {
                value: alarm.is_enabled,
            }),
            AlarmSettingKind::Time => Some(Setting::TimeOfDay {
                minutes_after_midnight: alarm.time.into(),
            }),
            AlarmSettingKind::Repeat => Some(Setting::MultiSelect {
                setting: settings::Select {
                    options: AlarmRepeatDay::iter()
                        .map(|day| Cow::Borrowed(day.into()))
                        .collect(),
                    localized_options: AlarmRepeatDay::iter().map(|day| day.translate()).collect(),
                },
                values: AlarmRepeatDay::iter_from_alarm_repeat(alarm.repeat)
                    .map(|day| Cow::Borrowed(day.into()))
                    .collect(),
            }),
            AlarmSettingKind::WakeUpTune => {
                Some(Setting::select_from_enum_all_variants(alarm.wake_up_tune))
            }
            AlarmSettingKind::Volume => Some(Setting::I32Range {
                setting: settings::Range {
                    range: 0..=100,
                    step: 1,
                },
                value: alarm.volume.inner().into(),
            }),
            AlarmSettingKind::SnoozeDuration => Some(Setting::I32Range {
                setting: settings::Range {
                    range: 0..=10,
                    step: 1,
                },
                value: alarm.snooze_duration_in_minutes.into(),
            }),
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let alarms = state.get_mut();

        let maybe_general_alarm_setting_id: Option<GeneralAlarmSettingId> =
            (*setting_id).try_into().ok();
        if let Some(general_alarm_setting_id) = maybe_general_alarm_setting_id {
            match general_alarm_setting_id {
                GeneralAlarmSettingId::CreateAlarm => {
                    if alarms.len() < MAX_ALARMS {
                        alarms
                            .push(Alarm::default_with_id(u8::try_from(alarms.len()).expect(
                                "length is less than MAX_ALARMS, so this won't overflow",
                            )))
                    }
                }
            }
            return Ok(());
        }

        let per_alarm_setting_id: PerAlarmSettingId = (*setting_id)
            .try_into()
            .expect("filtered to valid values only by SettingsManager, except GeneralAlarmSettingId, which we already covered");

        let alarm_setting = AlarmSetting::from(per_alarm_setting_id);

        // We don't expose settings for nonexistent alarms, but quick presets may set them, so we
        // need to handle them in order for quick presets to be able to save alarms. The Soundcore
        // app adds alarms sequentially. I don't know how the device would handle adding alarm n+1
        // when alarm n doesn't exist, so it's safer to disallow it. Quick Presets will have
        // settings in order, and our order has alarm settings listed sequentially, so this will
        // work fine for that purpose.
        if alarms.len() == alarm_setting.alarm_index && alarms.len() < MAX_ALARMS {
            alarms
                .push(Alarm::default_with_id(u8::try_from(alarms.len()).expect(
                    "length is less than MAX_ALARMS, so this won't overflow",
                )));
        }
        let Some(alarm) = alarms.get_mut(alarm_setting.alarm_index) else {
            return Err(SettingHandlerError::DoesNotExist);
        };

        match alarm_setting.setting_kind {
            AlarmSettingKind::Delete => {
                if alarm_setting.alarm_index < alarms.len() {
                    alarms.remove(alarm_setting.alarm_index);
                }
            }
            AlarmSettingKind::Enabled => {
                alarm.is_enabled = value.try_as_bool()?;
            }
            AlarmSettingKind::Time => {
                alarm.time = i16::try_from(value.try_as_i32()?.clamp(0, 24 * 60 - 1))
                    .expect("we clamped to a range that fits within i16");
            }
            AlarmSettingKind::Repeat => {
                let day_names = value.try_into_string_vec()?;
                alarm.repeat = day_names
                    .into_iter()
                    .filter_map(|name| match AlarmRepeatDay::from_str(&name) {
                        Ok(day) => Some(day),
                        Err(err) => {
                            tracing::error!("ignoring invalid day {name}: {err:?}");
                            None
                        }
                    })
                    .fold(AlarmRepeat::empty(), |acc, curr| {
                        acc | curr.to_alarm_repeat_bit()
                    });
            }
            AlarmSettingKind::WakeUpTune => {
                alarm.wake_up_tune = value.try_as_enum_variant()?;
            }
            AlarmSettingKind::Volume => {
                alarm.volume = AlarmVolume::new(
                    value.try_as_i32()?.clamp(u8::MIN.into(), u8::MAX.into()) as u8,
                )
            }
            AlarmSettingKind::SnoozeDuration => {
                alarm.snooze_duration_in_minutes = alarm.snooze_duration_in_minutes.clamp(0, 10)
            }
        }

        Ok(())
    }
}

struct AlarmSetting {
    alarm_index: usize,
    setting_kind: AlarmSettingKind,
}

impl From<PerAlarmSettingId> for AlarmSetting {
    fn from(setting_id: PerAlarmSettingId) -> Self {
        Self {
            alarm_index: alarm_index(setting_id),
            setting_kind: AlarmSettingKind::from(setting_id),
        }
    }
}

fn alarm_index(setting: PerAlarmSettingId) -> usize {
    match setting {
        PerAlarmSettingId::DeleteAlarm1
        | PerAlarmSettingId::Alarm1Enabled
        | PerAlarmSettingId::Alarm1Time
        | PerAlarmSettingId::Alarm1Repeat
        | PerAlarmSettingId::Alarm1WakeUpTune
        | PerAlarmSettingId::Alarm1Volume
        | PerAlarmSettingId::Alarm1SnoozeDuration => 0,
        PerAlarmSettingId::DeleteAlarm2
        | PerAlarmSettingId::Alarm2Enabled
        | PerAlarmSettingId::Alarm2Time
        | PerAlarmSettingId::Alarm2Repeat
        | PerAlarmSettingId::Alarm2WakeUpTune
        | PerAlarmSettingId::Alarm2Volume
        | PerAlarmSettingId::Alarm2SnoozeDuration => 1,
        PerAlarmSettingId::DeleteAlarm3
        | PerAlarmSettingId::Alarm3Enabled
        | PerAlarmSettingId::Alarm3Time
        | PerAlarmSettingId::Alarm3Repeat
        | PerAlarmSettingId::Alarm3WakeUpTune
        | PerAlarmSettingId::Alarm3Volume
        | PerAlarmSettingId::Alarm3SnoozeDuration => 2,
        PerAlarmSettingId::DeleteAlarm4
        | PerAlarmSettingId::Alarm4Enabled
        | PerAlarmSettingId::Alarm4Time
        | PerAlarmSettingId::Alarm4Repeat
        | PerAlarmSettingId::Alarm4WakeUpTune
        | PerAlarmSettingId::Alarm4Volume
        | PerAlarmSettingId::Alarm4SnoozeDuration => 3,
        PerAlarmSettingId::DeleteAlarm5
        | PerAlarmSettingId::Alarm5Enabled
        | PerAlarmSettingId::Alarm5Time
        | PerAlarmSettingId::Alarm5Repeat
        | PerAlarmSettingId::Alarm5WakeUpTune
        | PerAlarmSettingId::Alarm5Volume
        | PerAlarmSettingId::Alarm5SnoozeDuration => 4,
    }
}

enum AlarmSettingKind {
    Delete,
    Enabled,
    Time,
    Repeat,
    WakeUpTune,
    Volume,
    SnoozeDuration,
}

impl From<PerAlarmSettingId> for AlarmSettingKind {
    fn from(setting_id: PerAlarmSettingId) -> Self {
        match setting_id {
            PerAlarmSettingId::DeleteAlarm1
            | PerAlarmSettingId::DeleteAlarm2
            | PerAlarmSettingId::DeleteAlarm3
            | PerAlarmSettingId::DeleteAlarm4
            | PerAlarmSettingId::DeleteAlarm5 => AlarmSettingKind::Delete,
            PerAlarmSettingId::Alarm1Enabled
            | PerAlarmSettingId::Alarm2Enabled
            | PerAlarmSettingId::Alarm3Enabled
            | PerAlarmSettingId::Alarm4Enabled
            | PerAlarmSettingId::Alarm5Enabled => AlarmSettingKind::Enabled,
            PerAlarmSettingId::Alarm1Time
            | PerAlarmSettingId::Alarm2Time
            | PerAlarmSettingId::Alarm3Time
            | PerAlarmSettingId::Alarm4Time
            | PerAlarmSettingId::Alarm5Time => AlarmSettingKind::Time,
            PerAlarmSettingId::Alarm1Repeat
            | PerAlarmSettingId::Alarm2Repeat
            | PerAlarmSettingId::Alarm3Repeat
            | PerAlarmSettingId::Alarm4Repeat
            | PerAlarmSettingId::Alarm5Repeat => AlarmSettingKind::Repeat,
            PerAlarmSettingId::Alarm1WakeUpTune
            | PerAlarmSettingId::Alarm2WakeUpTune
            | PerAlarmSettingId::Alarm3WakeUpTune
            | PerAlarmSettingId::Alarm4WakeUpTune
            | PerAlarmSettingId::Alarm5WakeUpTune => AlarmSettingKind::WakeUpTune,
            PerAlarmSettingId::Alarm1Volume
            | PerAlarmSettingId::Alarm2Volume
            | PerAlarmSettingId::Alarm3Volume
            | PerAlarmSettingId::Alarm4Volume
            | PerAlarmSettingId::Alarm5Volume => AlarmSettingKind::Volume,
            PerAlarmSettingId::Alarm1SnoozeDuration
            | PerAlarmSettingId::Alarm2SnoozeDuration
            | PerAlarmSettingId::Alarm3SnoozeDuration
            | PerAlarmSettingId::Alarm4SnoozeDuration
            | PerAlarmSettingId::Alarm5SnoozeDuration => AlarmSettingKind::SnoozeDuration,
        }
    }
}

#[derive(Clone, Copy, EnumIter, EnumString, IntoStaticStr, Translate)]
enum AlarmRepeatDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl AlarmRepeatDay {
    fn iter_from_alarm_repeat(repeat: AlarmRepeat) -> impl Iterator<Item = Self> {
        Self::iter().filter(move |variant| repeat.contains(variant.to_alarm_repeat_bit()))
    }

    fn to_alarm_repeat_bit(&self) -> AlarmRepeat {
        match self {
            AlarmRepeatDay::Sunday => AlarmRepeat::SUNDAY,
            AlarmRepeatDay::Monday => AlarmRepeat::MONDAY,
            AlarmRepeatDay::Tuesday => AlarmRepeat::TUESDAY,
            AlarmRepeatDay::Wednesday => AlarmRepeat::WEDNESDAY,
            AlarmRepeatDay::Thursday => AlarmRepeat::THURSDAY,
            AlarmRepeatDay::Friday => AlarmRepeat::FRIDAY,
            AlarmRepeatDay::Saturday => AlarmRepeat::SATURDAY,
        }
    }
}
