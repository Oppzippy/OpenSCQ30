use gtk::glib;
use openscq30_lib::packets::structures::CustomNoiseCanceling;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30ValuesCustomNoiseCanceling")]
pub struct GlibCustomNoiseCancelingValue(pub CustomNoiseCanceling);
