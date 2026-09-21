use std::borrow::Cow;

use cosmic::widget;

pub fn toggle<'a, M>(
    name: Cow<'a, str>,
    value: bool,
    on_change: impl Fn(bool) -> M + 'static,
) -> widget::list::ListButton<'a, M>
where
    M: Clone + 'static,
{
    widget::settings::item::builder(name).toggler(value, on_change)
}
