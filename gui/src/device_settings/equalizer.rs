use std::{
    borrow::Cow,
    ops::{Add, Sub},
};

use cosmic::{
    Apply, Element,
    iced::{Alignment, Border, Length, Shadow},
    theme, widget,
};
use openscq30_lib::api::settings::Equalizer;

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
        .cloned()
        .enumerate()
        .map(move |(i, hz)| {
            widget::column::with_children(vec![
                widget::text(fl!("hz", hz = hz)).into(),
                widget::row::with_children(vec![
                    wider_spin_button(
                        {
                            let divisor = 10i16.pow(setting.fraction_digits as u32);
                            let db_integer_portion = value[i] / divisor;
                            let db_decimal_portion = (value[i] % divisor).abs();
                            let decimal_db = format!("{db_integer_portion}.{db_decimal_portion}");
                            fl!("db", db = decimal_db)
                        },
                        value[i],
                        1,
                        setting.min,
                        setting.max,
                        {
                            let on_change = on_change.clone();
                            move |band_value| on_change(i as u8, band_value)
                        },
                        80,
                    ),
                    widget::slider(
                        setting.min..=setting.max,
                        value
                            .get(i)
                            .cloned()
                            .unwrap_or((setting.min + setting.max) / 2),
                        {
                            let on_change = on_change.clone();
                            move |band_value| on_change(i as u8, band_value)
                        },
                    )
                    .into(),
                ])
                .into(),
            ])
            .into()
        })
        .collect()
}

// Taken from libcosmic and modified to fix text not taking enough width, causing it to go to the next line
fn wider_spin_button<'a, T, M>(
    label: impl Into<Cow<'a, str>> + 'a,
    value: T,
    step: T,
    min: T,
    max: T,
    on_press: impl Fn(T) -> M + 'static,
    width: impl Into<Length>,
) -> Element<'a, M>
where
    M: Clone + 'static,
    T: Copy + Sub<Output = T> + Add<Output = T> + PartialOrd,
{
    let decrement_button = widget::icon::from_name("list-remove-symbolic")
        .apply(widget::button::icon)
        .on_press((on_press)(decrement::<T>(value, step, min, max)));

    let increment_button = widget::icon::from_name("list-add-symbolic")
        .apply(widget::button::icon)
        .on_press((on_press)(increment::<T>(value, step, min, max)));

    let label = widget::text::title4(label)
        .apply(widget::container)
        .width(width)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

    widget::row::with_capacity(3)
        .push(decrement_button)
        .push(label)
        .push(increment_button)
        .align_y(Alignment::Center)
        .apply(widget::container)
        .class(theme::Container::custom(container_style))
        .into()
}

fn increment<T>(value: T, step: T, _min: T, max: T) -> T
where
    T: Copy + Sub<Output = T> + Add<Output = T> + PartialOrd,
{
    if value > max - step {
        max
    } else {
        value + step
    }
}

fn decrement<T>(value: T, step: T, min: T, _max: T) -> T
where
    T: Copy + Sub<Output = T> + Add<Output = T> + PartialOrd,
{
    if value < min + step {
        min
    } else {
        value - step
    }
}

fn container_style(theme: &cosmic::Theme) -> cosmic::iced::widget::container::Style {
    let cosmic_theme = &theme.cosmic();
    let mut neutral_10 = cosmic_theme.palette.neutral_10;
    neutral_10.alpha = 0.1;
    let accent = &cosmic_theme.accent;
    let corners = &cosmic_theme.corner_radii;
    cosmic::iced::widget::container::Style {
        icon_color: Some(accent.base.into()),
        text_color: Some(cosmic_theme.palette.neutral_10.into()),
        background: None,
        border: Border {
            radius: corners.radius_s.into(),
            width: 0.0,
            color: accent.base.into(),
        },
        shadow: Shadow::default(),
    }
}
