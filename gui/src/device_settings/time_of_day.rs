use cosmic::{Element, iced::alignment, widget};
use openscq30_i18n::Translate;
use openscq30_lib::settings::SettingId;

use crate::{device_settings::labeled_setting_row, fl};

pub fn time<'a, M>(
    setting_id: SettingId,
    minutes_after_midnight: i32,
    on_change: impl Fn(i32) -> M + Clone + Send + Sync + 'static,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    let time = Time::from_minutes_after_midnight(minutes_after_midnight);
    labeled_setting_row(
        setting_id.translate(),
        widget::row![
            widget::spin_button(time.hour.to_string(), fl!("hour"), time.hour, 1, 1, 12, {
                let on_change = on_change.clone();
                move |hour| on_change(Time { hour, ..time }.to_minutes_after_midnight())
            }),
            // TODO fix horrible UX
            widget::spin_button(
                time.minute.to_string(),
                fl!("minute"),
                time.minute,
                1,
                0,
                59,
                {
                    let on_change = on_change.clone();
                    move |minute| on_change(Time { minute, ..time }.to_minutes_after_midnight())
                }
            ),
            widget::dropdown(
                // This doesn't seem to work with an array, so unfortunately an extra heap allocation is necessary
                vec![fl!("am"), fl!("pm")],
                Some(usize::from(time.am_pm == AmPm::Pm)),
                move |index| on_change(
                    Time {
                        am_pm: if index == 0 { AmPm::Am } else { AmPm::Pm },
                        ..time
                    }
                    .to_minutes_after_midnight()
                )
            ),
        ]
        .align_y(alignment::Vertical::Center),
    )
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Time {
    am_pm: AmPm,
    hour: i32,
    minute: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AmPm {
    Am,
    Pm,
}

impl Time {
    fn from_minutes_after_midnight(minutes_after_midnight: i32) -> Self {
        let hour_starting_at_zero = minutes_after_midnight / 60 % 12;
        let hour = if hour_starting_at_zero == 0 {
            12
        } else {
            hour_starting_at_zero
        };
        Self {
            am_pm: if minutes_after_midnight < 60 * 12 {
                AmPm::Am
            } else {
                AmPm::Pm
            },
            hour,
            minute: minutes_after_midnight % 60,
        }
    }

    fn to_minutes_after_midnight(&self) -> i32 {
        debug_assert_ne!(
            self.hour, 0,
            "hour should be 12 rather than 0 since we use 12 hour time"
        );
        debug_assert!(self.hour <= 12, "hour {} should be less than 12", self.hour);
        debug_assert!(
            self.minute < 60,
            "minute {} should be less than 60",
            self.minute
        );
        debug_assert!(
            !self.hour.is_negative(),
            "hour {} should be non-negative",
            self.hour
        );
        debug_assert!(
            !self.minute.is_negative(),
            "minute {} should be non-negative",
            self.minute
        );

        let hours_starting_at_0 = if self.hour == 12 { 0 } else { self.hour };
        (self.am_pm == AmPm::Pm)
            .then_some(12 * 60)
            .unwrap_or_default()
            + hours_starting_at_0 * 60
            + self.minute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_12_am() {
        let expected = Time {
            hour: 12,
            minute: 0,
            am_pm: AmPm::Am,
        };
        let actual = Time::from_minutes_after_midnight(expected.to_minutes_after_midnight());

        assert_eq!(expected, actual);
    }

    #[test]
    fn round_trips_11_59_am() {
        let expected = Time {
            hour: 11,
            minute: 59,
            am_pm: AmPm::Am,
        };
        let actual = Time::from_minutes_after_midnight(expected.to_minutes_after_midnight());

        assert_eq!(expected, actual);
    }

    #[test]
    fn round_trips_12_pm() {
        let expected = Time {
            hour: 12,
            minute: 0,
            am_pm: AmPm::Pm,
        };
        let actual = Time::from_minutes_after_midnight(expected.to_minutes_after_midnight());

        assert_eq!(expected, actual);
    }

    #[test]
    fn round_trips_12_59_pm() {
        let expected = Time {
            hour: 12,
            minute: 59,
            am_pm: AmPm::Pm,
        };
        let actual = Time::from_minutes_after_midnight(expected.to_minutes_after_midnight());

        assert_eq!(expected, actual);
    }

    #[test]
    fn round_trips_11_59_pm() {
        let expected = Time {
            hour: 11,
            minute: 59,
            am_pm: AmPm::Pm,
        };
        let actual = Time::from_minutes_after_midnight(expected.to_minutes_after_midnight());

        assert_eq!(expected, actual);
    }
}
