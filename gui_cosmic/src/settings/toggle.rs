use cosmic::{Element, iced::Length, widget};
use openscq30_lib::api::settings::SettingId;

pub fn toggle<'a, M>(
    setting_id: SettingId<'a>,
    value: bool,
    on_change: impl Fn(bool) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    widget::toggler(value)
        .label(setting_id.0)
        .width(Length::FillPortion(1))
        .on_toggle(on_change)
        .into()
}
