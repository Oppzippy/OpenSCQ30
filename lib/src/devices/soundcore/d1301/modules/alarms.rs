mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        common::{modules::ModuleCollection, packet::PacketIOController},
        d1301,
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(Copy, Clone, EnumString, EnumIter, IntoStaticStr)]
    enum GeneralAlarmSettingId {
        CreateAlarm,
    }
}

enum_subset! {
    SettingId,
    #[derive(Copy, Clone, EnumString, EnumIter, IntoStaticStr)]
    enum PerAlarmSettingId {
        Alarm1Enabled,
        Alarm1Time,
        Alarm1Repeat,
        Alarm1WakeUpTune,
        Alarm1Volume,
        Alarm1SnoozeDuration,
        DeleteAlarm1,
        Alarm2Enabled,
        Alarm2Time,
        Alarm2Repeat,
        Alarm2WakeUpTune,
        Alarm2Volume,
        Alarm2SnoozeDuration,
        DeleteAlarm2,
        Alarm3Enabled,
        Alarm3Time,
        Alarm3Repeat,
        Alarm3WakeUpTune,
        Alarm3Volume,
        Alarm3SnoozeDuration,
        DeleteAlarm3,
        Alarm4Enabled,
        Alarm4Time,
        Alarm4Repeat,
        Alarm4WakeUpTune,
        Alarm4Volume,
        Alarm4SnoozeDuration,
        DeleteAlarm4,
        Alarm5Enabled,
        Alarm5Time,
        Alarm5Repeat,
        Alarm5WakeUpTune,
        Alarm5Volume,
        Alarm5SnoozeDuration,
        DeleteAlarm5,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<Vec<d1301::structures::Alarm>> + Clone + Send + Sync,
{
    pub fn add_d1301_alarms(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Alarms, setting_handler::AlarmsSettingHandler);
        self.state_modifiers
            .push(Box::new(state_modifier::AlarmsStateModifier::new(
                packet_io,
            )));
    }
}
