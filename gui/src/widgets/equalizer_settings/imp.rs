use std::cell::{Cell, RefCell};

use gtk::{
    gio,
    glib::{self, clone, once_cell::sync::Lazy, subclass::Signal, ParamSpec, ParamSpecBoolean},
    prelude::*,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass, *},
        widget::{
            CompositeTemplateCallbacksClass, CompositeTemplateClass, WidgetClassSubclassExt,
            WidgetImpl,
        },
    },
    CompositeTemplate, Expression, PropertyExpression, TemplateChild,
};
use openscq30_lib::packets::structures::{
    EqualizerBandOffsets, EqualizerConfiguration, PresetEqualizerProfile,
};
use strum::IntoEnumIterator;

use crate::objects::{EqualizerCustomProfileObject, EqualizerProfileObject};
use crate::widgets::Equalizer;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/equalizer_settings/template.ui")]
pub struct EqualizerSettings {
    #[template_child]
    pub equalizer: TemplateChild<Equalizer>,
    #[template_child]
    pub profile_dropdown: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub apply_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub refresh_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub custom_profile_selection: TemplateChild<gtk::Box>,
    #[template_child]
    pub custom_profile_dropdown: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub create_custom_profile_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub delete_custom_profile_button: TemplateChild<gtk::Button>,

    profiles: RefCell<Option<gio::ListStore>>,
    profile_objects: RefCell<Vec<EqualizerProfileObject>>,
    custom_profiles: RefCell<Option<gio::ListStore>>,
    custom_profile_objects: RefCell<Vec<EqualizerCustomProfileObject>>,
    is_custom_profile: Cell<bool>,
}

#[gtk::template_callbacks]
impl EqualizerSettings {
    #[template_callback]
    fn handle_apply_custom_equalizer(&self, _button: &gtk::Button) {
        self.obj().emit_by_name("apply-equalizer-settings", &[])
    }

    #[template_callback]
    fn handle_refresh_custom_equalizer(&self, _button: &gtk::Button) {
        self.obj().emit_by_name("refresh-equalizer-settings", &[])
    }

    #[template_callback]
    fn handle_create_custom_profile(&self, _button: &gtk::Button) {
        self.obj().emit_by_name(
            "create-custom-equalizer-profile",
            &[&EqualizerCustomProfileObject::new(
                &"".to_string(), // TODO use a different object that doesn't have a name field
                self.equalizer.volumes(),
            )],
        )
    }

    #[template_callback]
    fn handle_delete_custom_profile(&self, _button: &gtk::Button) {
        let profiles = self.custom_profile_objects.borrow();
        if let Some(profile) = profiles.get(self.custom_profile_dropdown.selected() as usize) {
            let profile = profile.clone();
            // The signal handlers won't be able to access profiles if we don't drop it before firing the signal
            std::mem::drop(profiles);
            self.obj()
                .emit_by_name::<()>("delete-custom-equalizer-profile", &[&profile]);
        }
    }

    #[template_callback]
    fn handle_volumes_changed(&self, _equalizer: &Equalizer) {
        self.update_custom_profile_selection();
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        if self.is_custom_profile.get() {
            EqualizerConfiguration::new_custom_profile(EqualizerBandOffsets::new(
                self.equalizer.volumes(),
            ))
        } else {
            let selection = self
                .profile_dropdown
                .selected_item()
                .expect("an item must be selected")
                .downcast::<EqualizerProfileObject>()
                .expect("selected item must be an EqualizerProfileObject");
            EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::from_id(selection.profile_id() as u16).unwrap_or_else(
                    || {
                        panic!(
                            "equalizer preset with selected profile id {} not found",
                            selection.profile_id()
                        );
                    },
                ),
            )
        }
    }

    pub fn set_equalizer_configuration(&self, configuration: &EqualizerConfiguration) {
        self.equalizer
            .set_volumes(configuration.band_offsets().volume_offsets());
        self.profile_dropdown.set_selected(
            self.profile_objects
                .borrow()
                .iter()
                .position(|profile| profile.profile_id() as u16 == configuration.profile_id())
                .unwrap_or(0)
                .try_into()
                .expect("could not convert usize to u32"),
        );
    }

    fn set_profiles(&self, profiles: Vec<EqualizerProfileObject>) {
        if let Some(model) = &*self.profiles.borrow() {
            model.remove_all();
            model.extend_from_slice(&profiles);
            self.profile_objects.replace(profiles);
        }
    }

    fn set_up_custom_profile(&self) {
        self.set_up_custom_profile_selection_model();
        self.set_up_custom_profile_expression();
        self.set_up_custom_profile_selection_changed_handler();
    }

    fn set_up_custom_profile_selection_model(&self) {
        let model = gio::ListStore::new(EqualizerCustomProfileObject::static_type());
        self.custom_profiles.replace(Some(model.to_owned()));

        self.custom_profile_dropdown.set_model(Some(&model));
    }

    fn set_up_custom_profile_expression(&self) {
        self.custom_profile_dropdown
            .set_expression(Some(PropertyExpression::new(
                EqualizerCustomProfileObject::static_type(),
                None::<Expression>,
                "name",
            )));
    }

    fn set_up_custom_profile_selection_changed_handler(&self) {
        self.custom_profile_dropdown.connect_selected_item_notify(
            clone!(@weak self as this => move |_dropdown| {
                let maybe_selected_item  = this.custom_profile_dropdown.selected_item()
                    .map(|item| item.downcast::<EqualizerCustomProfileObject>().unwrap());
                if let Some(selected_item) = maybe_selected_item {
                    this.obj().emit_by_name("custom-equalizer-profile-selected", &[&selected_item])
                }
            }),
        );
    }

    pub fn set_custom_profiles(&self, mut profiles: Vec<EqualizerCustomProfileObject>) {
        if let Some(model) = &*self.custom_profiles.borrow() {
            profiles.sort_unstable_by(|left, right| left.name().cmp(&right.name()));
            // Notifications need to be frozen to prevent the selection changes while removing and adding items from
            // causing the profile to change. We can't force having no selection when adding new items, so it
            // will change the selection to the newly added item. We can set it back to what it's supposed to be
            // afterwards.
            let _notify_freeze_guard = self.custom_profile_dropdown.freeze_notify();
            model.remove_all();
            model.extend_from_slice(&profiles);
            self.custom_profile_objects.replace(profiles);
            self.update_custom_profile_selection();
        }
    }

    fn update_custom_profile_selection(&self) {
        if self.is_custom_profile.get() {
            let profiles = self.custom_profile_objects.borrow();
            let volumes = self.equalizer.volumes();
            let custom_profile_index = profiles
                .iter()
                .enumerate()
                .find(|(_i, profile)| profile.volume_offsets() == volumes)
                .map(|(i, _profile)| i as u32)
                .unwrap_or(u32::MAX);
            self.custom_profile_dropdown
                .set_selected(custom_profile_index);
        } else {
            self.custom_profile_dropdown.set_selected(u32::MAX);
        }
    }

    fn set_up_preset_profile(&self) {
        self.set_up_preset_profile_selection_model();
        self.set_up_preset_profile_expression();
        self.set_up_preset_profile_selection_changed_handler();
        self.set_up_preset_profile_items();
        self.set_up_preset_profile_disabled_fields();
    }

    fn set_up_preset_profile_selection_model(&self) {
        let model = gio::ListStore::new(EqualizerProfileObject::static_type());
        self.profiles.replace(Some(model.to_owned()));

        self.profile_dropdown.set_model(Some(&model));
    }

    fn set_up_preset_profile_expression(&self) {
        self.profile_dropdown
            .set_expression(Some(PropertyExpression::new(
                EqualizerProfileObject::static_type(),
                None::<Expression>,
                "name",
            )));
    }

    fn set_up_preset_profile_selection_changed_handler(&self) {
        self.profile_dropdown
            .connect_selected_item_notify(clone!(@weak self as this => move |_dropdown| {
                let selected_item: EqualizerProfileObject = this.profile_dropdown
                    .selected_item()
                    .expect("an item must be selected")
                    .downcast()
                    .expect("selected item must be an EqualizerProfileObject");
                let profile_id = selected_item.profile_id() as u16;
                let configuration = if profile_id == EqualizerConfiguration::CUSTOM_PROFILE_ID {
                    EqualizerConfiguration::new_custom_profile(EqualizerBandOffsets::new(this.equalizer.volumes()))
                } else {
                    let preset_profile = PresetEqualizerProfile::from_id(profile_id).unwrap_or_else(|| {
                        panic!("invalid preset profile id {profile_id}");
                    });
                    EqualizerConfiguration::new_from_preset_profile(preset_profile)
                };
                this.set_equalizer_configuration(&configuration);
                this.obj().emit_by_name("apply-equalizer-settings", &[])
            }));
    }

    fn set_up_preset_profile_disabled_fields(&self) {
        let obj = self.obj();
        self.profile_dropdown
            .bind_property("selected-item", obj.as_ref(), "is-custom-profile")
            .transform_to(|_, value: EqualizerProfileObject| {
                Some(value.profile_id() as u16 == EqualizerConfiguration::CUSTOM_PROFILE_ID)
            })
            .sync_create()
            .build();

        obj.bind_property(
            "is-custom-profile",
            &self.custom_profile_selection.get(),
            "sensitive",
        )
        .sync_create()
        .build();
        obj.bind_property("is-custom-profile", &self.equalizer.get(), "sensitive")
            .sync_create()
            .build();
        obj.connect_notify_local(Some("is-custom-profile"), |this, _param| {
            this.imp().update_custom_profile_selection();
        });
    }

    fn set_up_preset_profile_items(&self) {
        let custom_profile_iter = [EqualizerProfileObject::new(
            &"Custom".to_string(),
            EqualizerConfiguration::CUSTOM_PROFILE_ID.into(),
        )]
        .into_iter();
        let preset_profile_iter = PresetEqualizerProfile::iter()
            .map(|preset| EqualizerProfileObject::new(&preset.to_string(), preset.id().into()));

        let profiles = custom_profile_iter
            .chain(preset_profile_iter)
            .collect::<Vec<_>>();
        self.set_profiles(profiles);
        self.profile_dropdown.set_selected(1); // Select Soundcore Signature by default
    }
}

#[glib::object_subclass]
impl ObjectSubclass for EqualizerSettings {
    const NAME: &'static str = "OpenSCQ30EqualizerSettings";
    type Type = super::EqualizerSettings;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EqualizerSettings {
    fn constructed(&self) {
        self.parent_constructed();
        self.set_up_preset_profile();
        self.set_up_custom_profile();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("apply-equalizer-settings").build(),
                Signal::builder("refresh-equalizer-settings").build(),
                Signal::builder("custom-equalizer-profile-selected")
                    .param_types([EqualizerCustomProfileObject::static_type()])
                    .build(),
                Signal::builder("create-custom-equalizer-profile")
                    .param_types([EqualizerCustomProfileObject::static_type()])
                    .build(),
                Signal::builder("delete-custom-equalizer-profile")
                    .param_types([EqualizerCustomProfileObject::static_type()])
                    .build(),
            ]
        });
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecBoolean::builder("is-custom-profile").build()]);
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "is-custom-profile" => self.is_custom_profile.get().to_value(),
            _ => unimplemented!(),
        }
    }
    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "is-custom-profile" => self
                .is_custom_profile
                .replace(value.get().expect("is-custom-profile must be a bool")),
            _ => unimplemented!(),
        };
    }
}
impl WidgetImpl for EqualizerSettings {}
impl BoxImpl for EqualizerSettings {}
