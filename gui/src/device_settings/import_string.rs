use std::borrow::Cow;

use cosmic::{Element, widget};

use crate::device_settings::labeled_setting_row;

pub fn input<'a, M>(
    name: Cow<'a, str>,
    text: Cow<'a, str>,
    on_input: impl Fn(String) -> M + 'static,
    on_submit: impl Fn(String) -> M + 'static,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(
        name,
        widget::text_input("", text)
            .on_input(on_input)
            .on_submit(on_submit),
    )
}
