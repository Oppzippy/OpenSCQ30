use std::borrow::Cow;

use cosmic::{Element, widget};

use crate::{device_settings::labeled_setting_row, fl};

pub fn action<M>(name: Cow<'_, str>, on_execute: M) -> Element<'_, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(
        name,
        widget::button::standard(fl!("execute")).on_press(on_execute),
    )
}
