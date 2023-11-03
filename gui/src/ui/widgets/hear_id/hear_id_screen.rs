use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::state::DeviceState;

use crate::actions::Action;

glib::wrapper! {
    pub struct HearIdScreen(ObjectSubclass<imp::HearIdScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl HearIdScreen {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: Sender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_device_state(&self, state: &DeviceState) {
        self.imp().set_device_state(state);
    }
}

mod imp {
    use std::cell::{OnceCell, RefCell};

    use adw::prelude::WidgetExt;
    use gtk::{
        glib::{self, Sender},
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };
    use openscq30_lib::{
        packets::structures::{BasicHearId, CustomHearId, HearId, VolumeAdjustments},
        state::DeviceState,
    };

    use crate::actions::Action;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/hear_id/hear_id_screen.ui")]
    pub struct HearIdScreen {
        #[template_child]
        is_enabled: TemplateChild<adw::SwitchRow>,
        #[template_child]
        volume_adjustments_left: TemplateChild<gtk::Label>,
        #[template_child]
        volume_adjustments_right: TemplateChild<gtk::Label>,
        #[template_child]
        time: TemplateChild<gtk::Label>,
        #[template_child]
        hear_id_type_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        hear_id_type: TemplateChild<gtk::Label>,
        #[template_child]
        hear_id_music_type_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        hear_id_music_type: TemplateChild<gtk::Label>,
        #[template_child]
        custom_volume_adjustments_left_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        custom_volume_adjustments_left: TemplateChild<gtk::Label>,
        #[template_child]
        custom_volume_adjustments_right_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        custom_volume_adjustments_right: TemplateChild<gtk::Label>,

        hear_id: RefCell<Option<HearId>>,
        sender: OnceCell<Sender<Action>>,
    }

    impl HearIdScreen {
        pub fn set_device_state(&self, state: &DeviceState) {
            *self.hear_id.borrow_mut() = state.hear_id.to_owned();
            match &state.hear_id {
                Some(HearId::Basic(hear_id)) => {
                    self.hear_id_type_row.set_visible(false);
                    self.hear_id_music_type_row.set_visible(false);
                    self.custom_volume_adjustments_left_row.set_visible(false);
                    self.custom_volume_adjustments_right_row.set_visible(false);

                    self.is_enabled.set_active(hear_id.is_enabled);
                    self.volume_adjustments_left
                        .set_label(&Self::format_volume_adjustments(
                            &hear_id.volume_adjustments.left,
                        ));
                    self.volume_adjustments_right
                        .set_label(&Self::format_volume_adjustments(
                            &hear_id.volume_adjustments.right,
                        ));
                    self.time.set_label(&hear_id.time.to_string());
                }
                Some(HearId::Custom(hear_id)) => {
                    self.hear_id_type_row.set_visible(true);
                    self.hear_id_music_type_row.set_visible(true);

                    self.is_enabled.set_active(hear_id.is_enabled);
                    self.volume_adjustments_left
                        .set_label(&Self::format_volume_adjustments(
                            &hear_id.volume_adjustments.left,
                        ));
                    self.volume_adjustments_right
                        .set_label(&Self::format_volume_adjustments(
                            &hear_id.volume_adjustments.right,
                        ));
                    self.time.set_label(&hear_id.time.to_string());
                    self.hear_id_type
                        .set_label(&hear_id.hear_id_type.0.to_string());
                    self.hear_id_music_type
                        .set_label(&hear_id.hear_id_music_type.0.to_string());

                    if let Some(custom_volume_adjustments) = &hear_id.custom_volume_adjustments {
                        self.custom_volume_adjustments_left_row.set_visible(true);
                        self.custom_volume_adjustments_right_row.set_visible(true);
                        self.custom_volume_adjustments_left.set_label(
                            &Self::format_volume_adjustments(&custom_volume_adjustments.left),
                        );
                        self.custom_volume_adjustments_right.set_label(
                            &Self::format_volume_adjustments(&custom_volume_adjustments.right),
                        );
                    } else {
                        self.custom_volume_adjustments_left_row.set_visible(false);
                        self.custom_volume_adjustments_right_row.set_visible(false);
                    }
                }
                None => {
                    tracing::error!(
                        "Hear ID screen received state with hear_id: None. This does nothing, since the screen should not be visible in this case."
                    );
                }
            }
        }

        fn format_volume_adjustments(volume_adjustments: &VolumeAdjustments) -> String {
            format!("{:?}", volume_adjustments.adjustments())
        }

        pub fn set_sender(&self, sender: Sender<Action>) {
            self.sender.set(sender.to_owned()).unwrap();
        }
    }

    #[template_callbacks]
    impl HearIdScreen {
        #[template_callback]
        pub fn handle_is_enabled_toggled(&self) {
            if let Some(hear_id) = &*self.hear_id.borrow() {
                let is_enabled = self.is_enabled.is_active();
                let new_hear_id = match hear_id {
                    HearId::Basic(hear_id) => HearId::Basic(BasicHearId {
                        is_enabled,
                        ..hear_id.to_owned()
                    }),
                    HearId::Custom(hear_id) => HearId::Custom(CustomHearId {
                        is_enabled,
                        ..hear_id.to_owned()
                    }),
                };
                self.sender
                    .get()
                    .unwrap()
                    .send(Action::SetHearId(new_hear_id))
                    .unwrap();
            } else {
                tracing::error!("tried to toggle hear id, but hear_id is None");
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HearIdScreen {
        const NAME: &'static str = "OpenSCQ30HearIdScreen";
        type Type = super::HearIdScreen;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for HearIdScreen {}
    impl WidgetImpl for HearIdScreen {}
    impl BoxImpl for HearIdScreen {}
}
