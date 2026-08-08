use std::borrow::Cow;

use cosmic::widget;

pub fn toggle_with_label<'a, M>(
    label: Cow<'static, str>,
    value: bool,
    on_change: impl Fn(bool) -> M + 'static,
) -> widget::list::ListButton<'a, M>
where
    M: Clone + 'static,
{
    widget::settings::item::builder(label).toggler(value, on_change)
}
