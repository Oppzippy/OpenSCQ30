use crate::devices::soundcore::common::{
    packet::{self, Command},
    structures::DecibelReadingRefreshRate,
};

pub fn set_limit_high_volume(enabled: bool, db_limit: u8) -> packet::Outbound {
    debug_assert_eq!(
        db_limit / 5 * 5,
        db_limit,
        "db limit should be a multiple of 5"
    );
    debug_assert!(
        (75..=100).contains(&db_limit),
        "db limit should be between 75 and 100"
    );
    packet::Outbound::new(Command([0x20, 0x82]), vec![enabled.into(), db_limit])
}

pub fn set_limit_high_volume_refresh_rate(
    refresh_rate: DecibelReadingRefreshRate,
) -> packet::Outbound {
    packet::Outbound::new(Command([0x20, 0x81]), vec![refresh_rate as u8])
}
