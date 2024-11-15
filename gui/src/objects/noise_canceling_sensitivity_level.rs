use gtk::glib;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30ValuesNoiseCancelingSensitivityLevel")]
pub struct GlibNoiseCancelingSensitivityLevelValue(pub u8);
