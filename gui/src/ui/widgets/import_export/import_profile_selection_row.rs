use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct ImportProfileSelectionRow(ObjectSubclass<imp::ImportProfileSelectionRow>)
        @extends adw::ExpanderRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImportProfileSelectionRow {
    pub fn new(name: &str) -> Self {
        let obj: Self = Object::builder().build();
        obj.imp().set_name(name);
        obj
    }

    pub fn rename_to(&self) -> Option<String> {
        self.imp().rename_to()
    }

    pub fn enabled(&self) -> bool {
        self.imp().enabled()
    }
}

mod imp {
    use adw::{prelude::*, subclass::prelude::*};
    use gtk::{
        glib,
        subclass::widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        template_callbacks, CompositeTemplate,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/import_export/import_profile_selection_row.ui"
    )]
    pub struct ImportProfileSelectionRow {
        #[template_child]
        rename_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        rename_entry: TemplateChild<adw::EntryRow>,
    }

    #[template_callbacks]
    impl ImportProfileSelectionRow {
        pub fn rename_to(&self) -> Option<String> {
            if self.rename_switch.is_active() {
                Some(self.rename_entry.text().into())
            } else {
                None
            }
        }

        pub fn set_name(&self, name: &str) {
            self.obj().set_title(name);
            self.rename_entry.set_text(name);
        }

        pub fn enabled(&self) -> bool {
            self.obj().enables_expansion()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImportProfileSelectionRow {
        const NAME: &'static str = "OpenSCQ30ImportProfileSelectionRow";
        type Type = super::ImportProfileSelectionRow;
        type ParentType = adw::ExpanderRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImportProfileSelectionRow {}
    impl WidgetImpl for ImportProfileSelectionRow {}
    impl BoxImpl for ImportProfileSelectionRow {}
    impl ListBoxRowImpl for ImportProfileSelectionRow {}
    impl PreferencesRowImpl for ImportProfileSelectionRow {}
    impl ExpanderRowImpl for ImportProfileSelectionRow {}
}
