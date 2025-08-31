use cosmic::{Element, widget};
use openscq30_i18n::Translate;
use openscq30_lib::settings::SettingId;

pub fn toggle<'a, M>(
    setting_id: SettingId,
    value: bool,
    on_change: impl Fn(bool) -> M + 'static,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    widget::settings::item::builder(setting_id.translate())
        .toggler(value, on_change)
        .into()
}
