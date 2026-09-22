use std::borrow::Cow;

use cosmic::{Element, iced::alignment, widget};

use crate::{device_settings::labeled_setting_row, fl};

pub fn time<'a, M>(
    name: Cow<'a, str>,
    maybe_is_pm: Option<bool>,
    hour_text: Cow<'a, str>,
    minute_text: Cow<'a, str>,
    on_hour_change: impl Fn(String) -> M + Clone + 'static,
    on_minute_change: impl Fn(String) -> M + Clone + 'static,
    on_pm_change: impl Fn(bool) -> M + Clone + Send + Sync + 'static,
    on_refresh: M,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(
        name,
        widget::row![
            widget::inline_input(fl!("hour"), hour_text)
                .on_input(on_hour_change.clone())
                .on_submit({
                    let on_refresh = on_refresh.clone();
                    move |_| on_refresh.clone()
                }),
            widget::text(":"),
            widget::inline_input(fl!("minute"), minute_text)
                .on_input(on_minute_change.clone())
                .on_submit(move |_| on_refresh.clone()),
            maybe_is_pm.map(|is_pm| widget::dropdown(
                vec![fl!("am"), fl!("pm")],
                Some(usize::from(is_pm)),
                move |index| on_pm_change(index == 1)
            )),
        ]
        .align_y(alignment::Vertical::Center),
    )
}
