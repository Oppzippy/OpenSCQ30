use std::sync::Arc;

use gtk::glib;

use crate::settings::QuickPreset;

#[derive(Clone, PartialEq, Eq, Debug, Hash, glib::Boxed)]
#[boxed_type(name = "OpenSCQ30ValuesNamedQuickPreset")]
pub struct GlibNamedQuickPresetValue {
    pub name: Arc<str>,
    pub quick_preset: QuickPreset,
}

impl Default for GlibNamedQuickPresetValue {
    fn default() -> Self {
        Self {
            name: "".into(),
            quick_preset: Default::default(),
        }
    }
}
