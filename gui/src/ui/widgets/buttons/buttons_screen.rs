use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::state::DeviceState;

use crate::actions::Action;

glib::wrapper! {
    pub struct ButtonsScreen(ObjectSubclass<imp::ButtonsScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ButtonsScreen {
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
    use std::cell::{Cell, OnceCell};

    use adw::prelude::*;
    use gtk::{
        gio,
        glib::{self, Sender},
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, ClosureExpression, CompositeTemplate,
    };
    use openscq30_lib::{
        packets::structures::{
            ButtonAction, CustomButtonModel, NoTwsButtonAction, TwsButtonAction,
        },
        state::DeviceState,
    };
    use strum::IntoEnumIterator;

    use crate::{
        actions::Action,
        objects::{GlibButtonAction, GlibButtonActionValue},
        APPLICATION_ID_STR,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/buttons/buttons_screen.ui")]
    pub struct ButtonsScreen {
        #[template_child]
        left_single_click: TemplateChild<adw::ComboRow>,
        #[template_child]
        left_double_click: TemplateChild<adw::ComboRow>,
        #[template_child]
        left_long_press: TemplateChild<adw::ComboRow>,
        #[template_child]
        right_single_click: TemplateChild<adw::ComboRow>,
        #[template_child]
        right_double_click: TemplateChild<adw::ComboRow>,
        #[template_child]
        right_long_press: TemplateChild<adw::ComboRow>,

        button_actions: OnceCell<gio::ListStore>,
        custom_button_model: Cell<Option<CustomButtonModel>>,

        ignore_changes: Cell<bool>,

        sender: OnceCell<Sender<Action>>,
    }

    impl ButtonsScreen {
        pub fn set_device_state(&self, state: &DeviceState) {
            let button_actions = self.button_actions.get().unwrap();
            if let Some(button_model) = state.custom_button_model {
                self.custom_button_model.set(Some(button_model));
                self.ignore_changes.set(true);
                Self::set_row_button_action_no_tws(
                    &self.left_single_click,
                    button_actions,
                    button_model.left_single_click,
                );
                Self::set_row_button_action_tws(
                    &self.left_double_click,
                    button_actions,
                    button_model.left_double_click,
                );
                Self::set_row_button_action_tws(
                    &self.left_long_press,
                    button_actions,
                    button_model.left_long_press,
                );
                Self::set_row_button_action_no_tws(
                    &self.right_single_click,
                    button_actions,
                    button_model.right_single_click,
                );
                Self::set_row_button_action_tws(
                    &self.right_double_click,
                    button_actions,
                    button_model.right_double_click,
                );
                Self::set_row_button_action_tws(
                    &self.right_long_press,
                    button_actions,
                    button_model.right_long_press,
                );
                self.ignore_changes.set(false);
            } else {
                tracing::error!(
                    "Buttons screen received state with custom_button_model: None. This does nothing, since the screen should not be visible in this case."
                );
            }
        }

        fn set_row_button_action_tws(
            row: &adw::ComboRow,
            button_actions: &gio::ListStore,
            selection: TwsButtonAction,
        ) {
            Self::set_row_button_action(
                row,
                button_actions,
                if selection.is_enabled {
                    // TODO what about TWS disabled action?
                    Some(selection.tws_connected_action)
                } else {
                    None
                },
            );
        }

        fn set_row_button_action_no_tws(
            row: &adw::ComboRow,
            button_actions: &gio::ListStore,
            selection: NoTwsButtonAction,
        ) {
            Self::set_row_button_action(
                row,
                button_actions,
                if selection.is_enabled {
                    Some(selection.action)
                } else {
                    None
                },
            );
        }

        fn set_row_button_action(
            row: &adw::ComboRow,
            button_actions: &gio::ListStore,
            selection: Option<ButtonAction>,
        ) {
            let index = button_actions.find_with_equal_func(|action| {
                action
                    .downcast_ref::<GlibButtonAction>()
                    .unwrap()
                    .button_action()
                    .0
                    == selection
            });
            if let Some(index) = index {
                row.set_selected(index);
            } else {
                panic!(
                    "every possible button action should be listed but {selection:?} was not found"
                );
            }
        }

        pub fn set_sender(&self, sender: Sender<Action>) {
            self.sender.set(sender.to_owned()).unwrap();
        }
    }

    #[template_callbacks]
    impl ButtonsScreen {
        #[template_callback]
        pub fn handle_changed(&self) {
            if self.ignore_changes.get() {
                return;
            }
            let Some(button_model) = self.custom_button_model.get() else {
                return;
            };
            let left_single_click_action = Self::get_row_action(&self.left_single_click);
            let left_double_click_action = Self::get_row_action(&self.left_double_click);
            let left_long_press_action = Self::get_row_action(&self.left_long_press);
            let right_single_click_action = Self::get_row_action(&self.right_single_click);
            let right_double_click_action = Self::get_row_action(&self.right_double_click);
            let right_long_press_action = Self::get_row_action(&self.right_long_press);

            let model = CustomButtonModel {
                left_single_click: NoTwsButtonAction {
                    is_enabled: left_single_click_action.is_some(),
                    action: left_single_click_action
                        .unwrap_or(button_model.left_single_click.action),
                },
                left_double_click: TwsButtonAction {
                    is_enabled: left_double_click_action.is_some(),
                    tws_connected_action: left_double_click_action
                        .unwrap_or(button_model.left_double_click.tws_connected_action),
                    tws_disconnected_action: left_double_click_action
                        .unwrap_or(button_model.left_double_click.tws_disconnected_action),
                },
                left_long_press: TwsButtonAction {
                    is_enabled: left_double_click_action.is_some(),
                    tws_connected_action: left_long_press_action
                        .unwrap_or(button_model.left_long_press.tws_connected_action),
                    tws_disconnected_action: left_long_press_action
                        .unwrap_or(button_model.left_long_press.tws_disconnected_action),
                },
                right_single_click: NoTwsButtonAction {
                    is_enabled: right_single_click_action.is_some(),
                    action: right_single_click_action
                        .unwrap_or(button_model.right_single_click.action),
                },
                right_double_click: TwsButtonAction {
                    is_enabled: right_double_click_action.is_some(),
                    tws_connected_action: right_double_click_action
                        .unwrap_or(button_model.right_double_click.tws_connected_action),
                    tws_disconnected_action: right_double_click_action
                        .unwrap_or(button_model.right_double_click.tws_disconnected_action),
                },
                right_long_press: TwsButtonAction {
                    is_enabled: right_double_click_action.is_some(),
                    tws_connected_action: right_long_press_action
                        .unwrap_or(button_model.right_long_press.tws_connected_action),
                    tws_disconnected_action: right_long_press_action
                        .unwrap_or(button_model.right_long_press.tws_disconnected_action),
                },
            };

            self.sender
                .get()
                .unwrap()
                .send(Action::SetCustomButtonModel(model))
                .unwrap();
        }

        fn get_row_action(row: &adw::ComboRow) -> Option<ButtonAction> {
            row.selected_item()
                .and_downcast_ref::<GlibButtonAction>()
                .map(|value| value.button_action().0)
                .expect("all items should be GlibButtonAction")
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ButtonsScreen {
        const NAME: &'static str = "OpenSCQ30ButtonsScreen";
        type Type = super::ButtonsScreen;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for ButtonsScreen {
        fn constructed(&self) {
            let mut button_actions = gio::ListStore::new::<GlibButtonAction>();
            button_actions.append(&GlibButtonAction::new(GlibButtonActionValue(None)));
            button_actions.extend(
                ButtonAction::iter()
                    .map(|action| GlibButtonAction::new(GlibButtonActionValue(Some(action)))),
            );

            let expression = ClosureExpression::with_callback(gtk::Expression::NONE, |args| {
                let button_action: GlibButtonAction = args[0].get().unwrap();
                if let Some(button_action) = button_action.button_action().0 {
                    glib::dpgettext2(Some(APPLICATION_ID_STR), "buttons", button_action.as_ref())
                } else {
                    glib::dpgettext2(Some(APPLICATION_ID_STR), "buttons", "Disabled")
                }
            });

            [
                &self.left_single_click,
                &self.left_double_click,
                &self.left_long_press,
                &self.right_single_click,
                &self.right_double_click,
                &self.right_long_press,
            ]
            .into_iter()
            .for_each(|row| {
                row.set_model(Some(&button_actions));
                row.set_expression(Some(&expression));
            });

            self.button_actions.set(button_actions).unwrap();
        }
    }
    impl WidgetImpl for ButtonsScreen {}
    impl BoxImpl for ButtonsScreen {}
}
