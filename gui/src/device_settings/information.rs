use std::borrow::Cow;

use cosmic::{Element, iced_core::text::Wrapping, widget};
use openscq30_i18n::Translate;
use openscq30_lib::api::settings::SettingId;

pub fn information<M>(setting_id: SettingId, text: Cow<'_, str>, on_copy: M) -> Element<'_, M>
where
    M: Clone + 'static,
{
    widget::settings::item::builder(setting_id.translate())
        // The copy button being with the title rather than the value isn't ideal, but putting it with the value causes
        // layout issues. If those layout issues are ever fixed, remove this comment and replace the flex control with
        // a row containing text and the icon button.
        .icon(widget::button::icon(widget::icon::from_name("edit-copy-symbolic")).on_press(on_copy))
        .flex_control(widget::text(text).wrapping(Wrapping::WordOrGlyph))
        .into()
}
