use cosmic::{Element, iced::alignment, widget};
use openscq30_lib::settings::Equalizer;

use crate::fl;

pub fn horizontal_equalizer<'a, M>(
    setting: &'a Equalizer,
    value: &'a [i16],
    on_change: impl Fn(u8, i16) -> M + 'static + Clone,
) -> Vec<Element<'a, M>>
where
    M: Clone + 'static,
{
    setting
        .band_hz
        .iter()
        .copied()
        .enumerate()
        .map(move |(i, hz)| {
            widget::column![
                widget::text(fl!("hz", hz = hz)),
                widget::row![
                    widget::spin_button(
                        {
                            let divisor = 10i16.pow(setting.fraction_digits as u32);
                            let db_integer_portion = value[i] / divisor;
                            let db_decimal_portion = (value[i] % divisor).abs();
                            let decimal_db = format!("{db_integer_portion}.{db_decimal_portion}");
                            decimal_db.to_string()
                        },
                        fl!("hz", hz = hz),
                        value[i],
                        1,
                        setting.min,
                        setting.max,
                        {
                            let on_change = on_change.clone();
                            move |band_value| on_change(i as u8, band_value)
                        },
                    ),
                    widget::slider(
                        setting.min..=setting.max,
                        value
                            .get(i)
                            .copied()
                            .unwrap_or((setting.min + setting.max) / 2),
                        {
                            let on_change = on_change.clone();
                            move |band_value| on_change(i as u8, band_value)
                        },
                    )
                    .name(fl!("hz", hz = hz)),
                ]
                .align_y(alignment::Vertical::Center),
            ]
            .into()
        })
        .collect()
}
