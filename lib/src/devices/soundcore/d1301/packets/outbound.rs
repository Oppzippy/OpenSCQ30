use std::iter;

use crate::devices::soundcore::{
    common::packet,
    d1301::structures::{Alarm, AutoStopTimer, DefaultListeningMode, ListeningMode},
};

pub fn request_auto_stop_timer() -> packet::Outbound {
    packet::Outbound::new(packet::Command([21, 3]), Vec::new())
}

pub fn request_alarms() -> packet::Outbound {
    packet::Outbound::new(packet::Command([20, 1]), Vec::new())
}

pub fn set_auto_stop_timer(auto_stop_timer: &AutoStopTimer) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([21, 133]),
        // does not include time left
        iter::once(u8::from(auto_stop_timer.is_enabled))
            .chain(auto_stop_timer.duration_in_minutes.to_le_bytes())
            .chain(iter::once(auto_stop_timer.unknown))
            .collect(),
    )
}

/// Create or update an alarm.
///
/// The body is [`Alarm::bytes`] unchanged: id, enabled, time as little endian
/// minutes past midnight, repeat bitflags, tune, volume, snooze minutes.
/// Updating an existing alarm sends the whole record with the same id.
///
/// Not yet reachable from a setting: alarms are a list of records, and there is
/// no `Setting` variant that can edit one. Kept here, verified, so that whoever
/// adds that primitive does not have to rediscover the wire format.
#[allow(dead_code)]
pub fn set_alarm(alarm: &Alarm) -> packet::Outbound {
    packet::Outbound::new(packet::Command([20, 129]), alarm.bytes().collect())
}

/// Delete the alarm with this id.
#[allow(dead_code)]
pub fn delete_alarm(alarm_id: u8) -> packet::Outbound {
    packet::Outbound::new(packet::Command([20, 130]), vec![alarm_id])
}

pub fn set_listening_mode(listening_mode: ListeningMode) -> packet::Outbound {
    packet::Outbound::new(packet::Command([1, 169]), listening_mode.bytes().collect())
}

pub fn set_default_listening_mode(listening_mode: DefaultListeningMode) -> packet::Outbound {
    packet::Outbound::new(packet::Command([16, 149]), listening_mode.bytes().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::soundcore::d1301::structures::{AlarmRepeat, AlarmVolume, AlarmWakeUpTune};

    /// Captured from a Sleep A30 while adding an alarm in the official app:
    /// 09:20, weekends, Nature, volume 50, snooze 10 minutes, in slot 1.
    #[test]
    fn set_alarm_matches_a_captured_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x14, 0x81, 0x12, 0x00, 0x01, 0x01, 0x30, 0x02, 0x82,
            0x00, 0x32, 0x0a, 0x8f,
        ];
        let alarm = Alarm {
            id: 1,
            is_enabled: true,
            time: 560, // 09:20
            repeat: AlarmRepeat::SATURDAY | AlarmRepeat::SUNDAY,
            wake_up_tune: AlarmWakeUpTune::Nature,
            volume: AlarmVolume::new(50),
            snooze_duration_in_minutes: 10,
        };
        assert_eq!(EXPECTED, set_alarm(&alarm).bytes_with_checksum());
    }

    /// Captured while editing the alarm already in slot 0. Updating sends the
    /// whole record with the same id rather than a partial change.
    #[test]
    fn set_alarm_updates_an_existing_alarm() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x14, 0x81, 0x12, 0x00, 0x00, 0x01, 0xd1, 0x01, 0x00,
            0x01, 0x32, 0x05, 0xa8,
        ];
        let alarm = Alarm {
            id: 0,
            is_enabled: true,
            time: 465, // 07:45
            repeat: AlarmRepeat::empty(),
            wake_up_tune: AlarmWakeUpTune::Glow,
            volume: AlarmVolume::new(50),
            snooze_duration_in_minutes: 5,
        };
        assert_eq!(EXPECTED, set_alarm(&alarm).bytes_with_checksum());
    }

    /// Captured from the same session while deleting the alarm in slot 1.
    #[test]
    fn delete_alarm_matches_a_captured_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x14, 0x82, 0x0b, 0x00, 0x01, 0x98,
        ];
        assert_eq!(EXPECTED, delete_alarm(1).bytes_with_checksum());
    }
}
