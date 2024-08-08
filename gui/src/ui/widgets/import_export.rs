mod export_profile_output;
mod export_profile_selection;
mod import_export_menu;
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

    use gtk::{
        glib,
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
        serialization::IOCustomEqualizerProfile,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/import_export.ui")]
    pub struct ImportExport {
        sender: OnceCell<UnboundedSender<Action>>,
        #[template_child]
        navigation: TemplateChild<adw::NavigationView>,
        #[template_child]
        import_export_menu: TemplateChild<ImportExportMenu>,
        #[template_child]
        export_profile_selection: TemplateChild<ExportProfileSelection>,
        #[template_child]
        export_profile_output: TemplateChild<ExportProfileOutput>,
    }

    #[template_callbacks]
    impl ImportExport {
        pub fn set_sender(&self, sender: UnboundedSender<Action>) {
            self.sender.set(sender).unwrap();
        }

        #[template_callback]
        fn start_equalizer_profile_export_flow(&self, _: ImportExportMenu) {
            self.navigation
                .push_by_tag("equalizer-profiles-export-profile-selection")
        }

        #[template_callback]
        fn start_equalizer_profile_import_flow(&self, _: ImportExportMenu) {}

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
        fn handle_reset(&self, _: Widget) {
            self.export_profile_selection.imp().reset();
            self.export_profile_output.imp().reset();
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

    impl ObjectImpl for ImportExport {}
    impl WidgetImpl for ImportExport {}
    impl BoxImpl for ImportExport {}
}
