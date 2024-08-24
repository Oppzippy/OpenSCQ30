mod export_profile_output;
mod export_profile_selection;
mod import_export_menu;
mod import_profile_selection;
mod import_profile_selection_row;
mod import_profile_string;
mod serialization;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{actions::Action, objects::GlibCustomEqualizerProfile};

glib::wrapper! {
    pub struct ImportExport(ObjectSubclass<imp::ImportExport>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImportExport {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: UnboundedSender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_custom_equalizer_profiles(&self, profiles: &[GlibCustomEqualizerProfile]) {
        self.imp().set_custom_equalizer_profiles(profiles);
    }
}

mod imp {
    use std::cell::OnceCell;

    use adw::prelude::*;
    use gtk::{
        glib,
        prelude::ObjectExt,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate, Widget,
    };
    use tokio::sync::mpsc::UnboundedSender;

    use crate::{actions::Action, objects::GlibCustomEqualizerProfile};

    use super::{
        export_profile_output::ExportProfileOutput,
        export_profile_selection::ExportProfileSelection, import_export_menu::ImportExportMenu,
        import_profile_selection::ImportProfileSelection,
        import_profile_string::ImportProfileString, serialization::IOCustomEqualizerProfile,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/import_export.ui")]
    pub struct ImportExport {
        sender: OnceCell<UnboundedSender<Action>>,
        #[template_child]
        title: TemplateChild<gtk::Label>,
        #[template_child]
        back_button: TemplateChild<gtk::Button>,
        #[template_child]
        navigation: TemplateChild<adw::NavigationView>,
        #[template_child]
        import_export_menu: TemplateChild<ImportExportMenu>,
        #[template_child]
        export_profile_selection: TemplateChild<ExportProfileSelection>,
        #[template_child]
        export_profile_output: TemplateChild<ExportProfileOutput>,
        #[template_child]
        import_profile_string: TemplateChild<ImportProfileString>,
        #[template_child]
        import_profile_selection: TemplateChild<ImportProfileSelection>,
    }

    #[template_callbacks]
    impl ImportExport {
        pub fn set_sender(&self, sender: UnboundedSender<Action>) {
            self.sender.set(sender).unwrap();
        }

        #[template_callback]
        fn start_equalizer_profile_export_flow(&self, _: ImportExportMenu) {
            self.navigation
                .push_by_tag("equalizer-profiles-export-profile-selection");
        }

        #[template_callback]
        fn start_equalizer_profile_import_flow(&self, _: ImportExportMenu) {
            self.navigation
                .push_by_tag("equalizer-profiles-import-string");
        }

        #[template_callback]
        fn navigate_to_equalizer_profiles_export_json(
            &self,
            profile_selection: ExportProfileSelection,
        ) {
            let profiles = profile_selection
                .imp()
                .selected_profiles()
                .into_iter()
                .map(IOCustomEqualizerProfile::from)
                .collect::<Vec<_>>();
            match serde_json::to_string(&profiles) {
                Ok(json) => {
                    self.export_profile_output.imp().set_text(&json);
                    self.navigation
                        .push_by_tag("equalizer-profiles-export-output");
                }
                Err(err) => {
                    tracing::error!(
                        "error serializing custom equalizer profiles for export: {err:?}"
                    );
                    self.navigation.pop_to_tag("menu");
                }
            }
        }

        #[template_callback]
        fn navigate_to_equalizer_profiles_import_selection(&self, _: ImportProfileString) {
            if let Some(profiles) = self.import_profile_string.profiles_or_show_parse_error() {
                self.import_profile_selection.set_profiles(profiles);
                self.navigation
                    .push_by_tag("equalizer-profiles-import-selection");
            }
        }

        #[template_callback]
        fn handle_import_profiles(&self, _: ImportProfileSelection) {
            self.sender
                .get()
                .unwrap()
                .send(Action::ImportCustomEqualizerProfiles {
                    profiles: self.import_profile_selection.selected_profiles(),
                    overwrite: self.import_profile_selection.should_overwrite(),
                })
                .unwrap();
            self.reset();
        }

        #[template_callback]
        fn handle_reset(&self, _: Widget) {
            self.reset();
        }

        fn reset(&self) {
            self.export_profile_selection.imp().reset();
            self.export_profile_output.imp().reset();
            self.import_profile_string.imp().reset();
            self.import_profile_selection.imp().reset();
            self.navigation.pop_to_tag("menu");
        }

        pub fn set_custom_equalizer_profiles(&self, profiles: &[GlibCustomEqualizerProfile]) {
            self.export_profile_selection.set_profiles(profiles);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImportExport {
        const NAME: &'static str = "OpenSCQ30ImportExport";
        type Type = super::ImportExport;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImportExport {
        fn constructed(&self) {
            self.navigation
                .bind_property("visible-page", &self.title.get(), "label")
                .transform_to(|_, page: adw::NavigationPage| Some(page.title()))
                .sync_create()
                .build();
            self.navigation
                .bind_property("visible-page", &self.back_button.get(), "visible")
                .transform_to(|_, page: adw::NavigationPage| page.tag().map(|tag| tag != "menu"))
                .sync_create()
                .build();
        }
    }
    impl WidgetImpl for ImportExport {}
    impl BoxImpl for ImportExport {}
}
