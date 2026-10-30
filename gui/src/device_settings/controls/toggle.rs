use std::borrow::Cow;

use cosmic::widget;

pub fn toggle<M>(
    name: Cow<'_, str>,
    value: bool,
    on_change: impl Fn(bool) -> M + 'static,
) -> widget::list::ListButton<'_, M>
where
    M: Clone + 'static,
{
    widget::settings::item::builder(name).toggler(value, on_change)
}
