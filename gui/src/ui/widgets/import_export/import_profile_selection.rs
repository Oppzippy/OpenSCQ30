use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::objects::GlibCustomEqualizerProfile;

glib::wrapper! {
    pub struct ImportProfileSelection(ObjectSubclass<imp::ImportProfileSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImportProfileSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn reset(&self) {
        self.imp().reset();
    }

    pub fn set_profiles(&self, profiles: Vec<GlibCustomEqualizerProfile>) {
        self.imp().set_profiles(profiles)
    }

    pub fn selected_profiles(&self) -> Vec<GlibCustomEqualizerProfile> {
        self.imp().selected_profiles()
    }

    pub fn should_overwrite(&self) -> bool {
        self.imp().should_overwrite()
    }
}

mod imp {
    use std::{cell::RefCell, sync::OnceLock};

    use adw::prelude::*;
    use gtk::{
        glib::{self, subclass::Signal},
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };

    use crate::{
        objects::GlibCustomEqualizerProfile,
        ui::widgets::import_export::import_profile_selection_row::ImportProfileSelectionRow,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/import_export/import_profile_selection.ui"
    )]
    pub struct ImportProfileSelection {
        #[template_child]
        overwrite_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        profile_selection_group: TemplateChild<adw::PreferencesGroup>,

        rows: RefCell<Vec<ImportProfileSelectionRow>>,
        profiles: RefCell<Vec<GlibCustomEqualizerProfile>>,
    }

    #[template_callbacks]
    impl ImportProfileSelection {
        #[template_callback]
        fn handle_import_clicked(&self, _: gtk::Button) {
            self.obj().emit_by_name::<()>("next", &[]);
        }

        pub fn reset(&self) {
            self.overwrite_switch.set_active(false);
            let mut rows = self.rows.borrow_mut();
            rows.iter()
                .for_each(|row| self.profile_selection_group.remove(row));
            *rows = Vec::new();
            *self.profiles.borrow_mut() = Vec::new();
        }

        pub fn set_profiles(&self, profiles: Vec<GlibCustomEqualizerProfile>) {
            let mut rows = self.rows.borrow_mut();
            rows.iter()
                .for_each(|row| self.profile_selection_group.remove(row));
            *rows = profiles
                .iter()
                .map(|profile| ImportProfileSelectionRow::new(&profile.name()))
                .collect();
            rows.iter()
                .for_each(|row| self.profile_selection_group.add(row));
            *self.profiles.borrow_mut() = profiles;
        }

        pub fn selected_profiles(&self) -> Vec<GlibCustomEqualizerProfile> {
            let rows = self.rows.borrow();
            self.profiles
                .borrow()
                .iter()
                .enumerate()
                .filter_map(|(index, profile)| {
                    let row = rows.get(index).unwrap();
                    if row.enabled() {
                        match row.rename_to() {
                            Some(new_name) => Some(GlibCustomEqualizerProfile::new(
                                &new_name,
                                profile.volume_adjustments(),
                            )),
                            None => Some(profile.to_owned()),
                        }
                    } else {
                        None
                    }
                })
                .collect()
        }

        pub fn should_overwrite(&self) -> bool {
            self.overwrite_switch.is_active()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImportProfileSelection {
        const NAME: &'static str = "OpenSCQ30ImportProfileSelection";
        type Type = super::ImportProfileSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImportProfileSelection {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("next").build()])
        }
    }
    impl WidgetImpl for ImportProfileSelection {}
    impl BoxImpl for ImportProfileSelection {}
}
