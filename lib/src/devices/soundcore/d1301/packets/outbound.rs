use std::iter;

use crate::devices::soundcore::{
    common::packet,
    d1301::structures::{
        AutoStopTimer, AutoSwitchOnceAsleep, DefaultListeningMode, ListeningMode, PostSleepAudio,
    },
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

// TODO
// pub fn set_alarm(alarm: &Alarm) -> packet::Outbound {
//     packet::Outbound::new(packet::Command([20, 129]), alarm.bytes().collect())
// }

// pub fn delete_alarm(alarm_id: u8) -> packet::Outbound {
//     packet::Outbound::new(packet::Command([20, 130]), vec![alarm_id])
// }

pub fn set_auto_switch_once_asleep(enabled: AutoSwitchOnceAsleep) -> packet::Outbound {
    packet::Outbound::new(packet::Command([21, 143]), enabled.bytes().to_vec())
}

/// Set the post-sleep audio action.
///
/// Shares command 21 133 with [`set_auto_stop_timer`], but the official app
/// fills the timer fields with 0xFF sentinels so they are left alone. We send
/// the identical bytes rather than writing timer state back.
pub fn set_post_sleep_audio(post_sleep_audio: PostSleepAudio) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([21, 133]),
        vec![0xFF, 0xFF, 0xFF, post_sleep_audio.0],
    )
}

pub fn set_listening_mode(listening_mode: ListeningMode) -> packet::Outbound {
    packet::Outbound::new(packet::Command([1, 169]), listening_mode.bytes().collect())
}

pub fn set_default_listening_mode(listening_mode: DefaultListeningMode) -> packet::Outbound {
    packet::Outbound::new(packet::Command([16, 149]), listening_mode.bytes().collect())
}
