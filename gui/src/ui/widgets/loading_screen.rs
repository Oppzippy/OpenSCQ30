use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::actions::Action;

glib::wrapper! {
    pub struct LoadingScreen(ObjectSubclass<imp::LoadingScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl LoadingScreen {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: UnboundedSender<Action>) {
        self.imp().set_sender(sender);
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
        template_callbacks, CompositeTemplate,
    };
    use tokio::sync::mpsc::UnboundedSender;

    use crate::actions::Action;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/loading_screen.ui")]
    pub struct LoadingScreen {
        sender: OnceCell<UnboundedSender<Action>>,
    }

    #[template_callbacks]
    impl LoadingScreen {
        pub fn set_sender(&self, sender: UnboundedSender<Action>) {
            self.sender.set(sender).unwrap();
        }

        #[template_callback]
        fn handle_cancel_clicked(&self, _: gtk::Button) {
            self.sender.get().unwrap().send(Action::Disconnect).unwrap();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LoadingScreen {
        const NAME: &'static str = "OpenSCQ30LoadingScreen";
        type Type = super::LoadingScreen;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LoadingScreen {}
    impl WidgetImpl for LoadingScreen {}
    impl BoxImpl for LoadingScreen {}
}
