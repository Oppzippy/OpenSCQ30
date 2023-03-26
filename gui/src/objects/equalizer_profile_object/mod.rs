use gtk::glib::{self, Object};

mod imp;

glib::wrapper! {
    pub struct EqualizerProfileObject(ObjectSubclass<imp::EqualizerProfileObject>);
}

impl EqualizerProfileObject {
    pub fn new(name: &str, profile_id: u32) -> Self {
        Object::builder()
            .property("name", name)
            .property("profile-id", &profile_id)
            .build()
    }
}
