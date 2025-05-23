use cosmic::{
    Element,
    iced::{Length, alignment},
    widget,
};
use openscq30_lib::storage::QuickPreset;

use crate::fl;

pub fn quick_presets<M>(
    quick_presets: &[QuickPreset],
    on_edit: impl Fn(usize) -> M,
    on_activate: impl Fn(usize) -> M,
    on_delete: impl Fn(usize) -> M,
) -> Element<'_, M>
where
    M: Clone + 'static,
{
    widget::column()
        .extend(
            quick_presets
                .iter()
                .enumerate()
                .map(|(i, preset)| quick_preset(preset, on_edit(i), on_activate(i), on_delete(i))),
        )
        .into()
}

fn quick_preset<M>(
    quick_preset: &QuickPreset,
    on_edit: M,
    on_activate: M,
    on_delete: M,
) -> Element<'_, M>
where
    M: Clone + 'static,
{
    widget::row()
        .padding(10)
        .align_y(alignment::Vertical::Center)
        .push(widget::text(&quick_preset.name).width(Length::Fill))
        .push(widget::button::standard(fl!("activate")).on_press(on_activate))
        .push(widget::button::standard(fl!("edit")).on_press(on_edit))
        .push(widget::button::destructive(fl!("delete")).on_press(on_delete))
        .into()
}
