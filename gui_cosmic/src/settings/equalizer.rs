use cosmic::{
    iced::{alignment, Length},
    widget, Element,
};
use openscq30_lib::api::settings::Equalizer;

use crate::fl;

pub fn responsive_equalizer<'a, M>(
    setting: &'a Equalizer,
    value: &'a [i16],
    on_change: impl Fn(u8, i16) -> M + 'static + Clone,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    widget::responsive(move |size| {
        if size.width < 700f32 || size.height < 300f32 {
            widget::scrollable(crate::settings::horizontal_equalizer(
                setting,
                value,
                on_change.clone(),
            ))
            .into()
        } else {
            vertical_equalizer(setting, value, on_change.clone())
        }
    })
    .into()
}

pub fn horizontal_equalizer<'a, M>(
    setting: &Equalizer,
    value: &[i16],
    on_change: impl Fn(u8, i16) -> M + 'static + Clone,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    widget::column()
        .extend(setting.band_hz.iter().cloned().enumerate().map(|(i, hz)| {
            widget::row()
                .width(Length::Fill)
                .align_y(alignment::Vertical::Center)
                .spacing(8)
                .push(widget::text::text(fl!("hz", hz = hz)))
                .push(widget::spin_button(
                    {
                        let divisor = 10i16.pow(setting.fraction_digits as u32);
                        let db_integer_portion = value[i as usize] / divisor;
                        let db_decimal_portion = (value[i as usize] % divisor).abs();
                        let decimal_db = format!("{db_integer_portion}.{db_decimal_portion}");
                        fl!("db", db = decimal_db)
                    },
                    value[i as usize],
                    1,
                    setting.min,
                    setting.max,
                    {
                        let on_change = on_change.clone();
                        move |band_value| on_change(i as u8, band_value)
                    },
                ))
                .push(widget::slider(
                    setting.min..=setting.max,
                    value
                        .get(i as usize)
                        .cloned()
                        .unwrap_or((setting.min + setting.max) / 2),
                    {
                        let on_change = on_change.clone();
                        move |band_value| on_change(i as u8, band_value)
                    },
                ))
                .into()
        }))
        .into()
}

pub fn vertical_equalizer<'a, M>(
    setting: &Equalizer,
    value: &[i16],
    on_change: impl Fn(u8, i16) -> M + 'static + Clone,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    widget::row()
        .extend(setting.band_hz.iter().cloned().enumerate().map(|(i, hz)| {
            widget::column()
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .spacing(8)
                .push(widget::vertical_slider(
                    setting.min..=setting.max,
                    value
                        .get(i as usize)
                        .cloned()
                        .unwrap_or((setting.min + setting.max) / 2),
                    {
                        let on_change = on_change.clone();
                        move |band_value| on_change(i as u8, band_value)
                    },
                ))
                .push(widget::vertical_spin_button(
                    {
                        let divisor = 10i16.pow(setting.fraction_digits as u32);
                        let db_integer_portion = value[i as usize] / divisor;
                        let db_decimal_portion = (value[i as usize] % divisor).abs();
                        let decimal_db = format!("{db_integer_portion}.{db_decimal_portion}");
                        fl!("db", db = decimal_db)
                    },
                    value[i as usize],
                    1,
                    setting.min,
                    setting.max,
                    {
                        let on_change = on_change.clone();
                        move |band_value| on_change(i as u8, band_value)
                    },
                ))
                .push(widget::text::text(fl!("hz", hz = hz)))
                .into()
        }))
        .into()
}
