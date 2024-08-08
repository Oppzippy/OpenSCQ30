use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::objects::GlibCustomEqualizerProfile;

glib::wrapper! {
    pub struct ExportProfileSelection(ObjectSubclass<imp::ExportProfileSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ExportProfileSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }
    pub fn set_profiles(&self, profiles: &[GlibCustomEqualizerProfile]) {
        self.imp().set_profiles(profiles.to_vec())
    }
}

mod imp {
    use std::{
        cell::RefCell,
        collections::{BTreeSet, HashSet},
        sync::OnceLock,
    };

    use adw::prelude::*;
    use gtk::{
        glib::{self, clone, subclass::Signal},
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };

    use crate::objects::GlibCustomEqualizerProfile;

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/import_export/export_profile_selection.ui"
    )]
    pub struct ExportProfileSelection {
        #[template_child]
        pub profile_list: TemplateChild<adw::PreferencesGroup>,

        profiles: RefCell<Vec<GlibCustomEqualizerProfile>>,
        enabled_profiles: RefCell<BTreeSet<String>>,
        child_rows: RefCell<Vec<adw::SwitchRow>>,
    }

    #[template_callbacks]
    impl ExportProfileSelection {
        #[template_callback]
        fn handle_select_all_clicked(&self, _: gtk::Button) {
            let child_rows = self.child_rows.borrow();
            let target_state = child_rows.iter().any(|row| !row.is_active());
            self.child_rows
                .borrow()
                .iter()
                .for_each(|row| row.set_active(target_state));
        }

        #[template_callback]
        fn handle_next_clicked(&self, _: gtk::Button) {
            self.obj().emit_by_name("next", &[])
        }

        pub fn reset(&self) {
            self.child_rows
                .borrow()
                .iter()
                .for_each(|row| row.set_active(false));
        }

        pub fn set_profiles(&self, mut new_profiles: Vec<GlibCustomEqualizerProfile>) {
            new_profiles.sort_by_cached_key(|profile| profile.name().to_lowercase());

            let profile_names = Self::profile_names(&new_profiles);
            self.clear_selected_profiles_except_subset(&profile_names);
            self.update_rows(profile_names);

            *self.profiles.borrow_mut() = new_profiles;
        }

        fn profile_names<'a>(
            profiles: impl IntoIterator<Item = &'a GlibCustomEqualizerProfile>,
        ) -> HashSet<String> {
            profiles
                .into_iter()
                .map(|profile| profile.name())
                .collect::<HashSet<_>>()
        }

        fn clear_selected_profiles_except_subset(&self, keep: &HashSet<String>) {
            self.enabled_profiles
                .borrow_mut()
                .retain(|name| keep.contains(name))
        }

        fn update_rows(&self, profile_names: impl IntoIterator<Item = String>) {
            let enabled_profiles = self.enabled_profiles.borrow();
            let mut child_rows = self.child_rows.borrow_mut();
            child_rows
                .iter()
                .for_each(|row| self.profile_list.remove(row));
            *child_rows = profile_names
                .into_iter()
                .map(|profile_name| {
                    let row = adw::SwitchRow::builder()
                        .active(enabled_profiles.contains(&profile_name))
                        .title(&profile_name)
                        .build();
                    row.connect_active_notify(clone!(
                        #[weak(rename_to=this)]
                        self,
                        move |row: &adw::SwitchRow| {
                            let mut enabled_profiles = this.enabled_profiles.borrow_mut();
                            if row.is_active() {
                                enabled_profiles.insert(profile_name.to_owned());
                            } else {
                                enabled_profiles.remove(&profile_name);
                            }
                        }
                    ));
                    row
                })
                .collect();
            child_rows.iter().for_each(|row| self.profile_list.add(row));
        }

        pub fn selected_profiles(&self) -> Vec<GlibCustomEqualizerProfile> {
            let enabled_profiles = self.enabled_profiles.borrow();
            self.profiles
                .borrow()
                .iter()
                .filter(|profile| enabled_profiles.contains(&profile.name()))
                .cloned()
                .collect()
        }

        pub fn enabled_profile_names(&self) -> BTreeSet<String> {
            self.enabled_profiles.borrow().to_owned()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExportProfileSelection {
        const NAME: &'static str = "OpenSCQ30ExportProfileSelection";
        type Type = super::ExportProfileSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExportProfileSelection {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("next").build()])
        }
    }
    impl WidgetImpl for ExportProfileSelection {}
    impl BoxImpl for ExportProfileSelection {}
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeSet, HashSet},
        sync::Arc,
    };

    use adw::prelude::*;
    use gtk::subclass::prelude::*;

    use super::ExportProfileSelection;
    use crate::{load_resources, objects::GlibCustomEqualizerProfile, test_utils::WalkChildren};

    #[gtk::test]
    fn test_remove_old_selected_profiles() {
        load_resources();
        let volume_adjustments: Arc<[f64]> = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0].into();
        let widget = ExportProfileSelection::new();
        widget.set_profiles(&vec![
            GlibCustomEqualizerProfile::new("test", volume_adjustments.to_owned()),
            GlibCustomEqualizerProfile::new("test2", volume_adjustments.to_owned()),
            GlibCustomEqualizerProfile::new("test3", volume_adjustments.to_owned()),
        ]);
        check_child_rows(
            &widget.imp().profile_list,
            HashSet::from(["test2".into(), "test3".into()]),
        );
        widget.set_profiles(&vec![
            GlibCustomEqualizerProfile::new("test", volume_adjustments.to_owned()),
            GlibCustomEqualizerProfile::new("test2", volume_adjustments.to_owned()),
        ]);
        assert_eq!(
            BTreeSet::from(["test2".to_string()]),
            widget.imp().enabled_profile_names(),
        )
    }

    fn check_child_rows(parent: &adw::PreferencesGroup, row_titles_to_check: HashSet<String>) {
        parent
            .walk_children()
            .filter_map(|child| child.downcast::<adw::SwitchRow>().ok())
            .for_each(|row| {
                if row_titles_to_check.contains(row.title().as_str()) {
                    row.set_active(true);
                }
            });
    }
}
