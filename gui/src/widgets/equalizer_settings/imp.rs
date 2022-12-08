use std::cell::{Cell, RefCell};

use gtk::glib::{clone, ParamSpec, ParamSpecBoolean};
use gtk::subclass::prelude::*;
use gtk::subclass::widget::CompositeTemplateCallbacksClass;
use gtk::{gio, SingleSelection};
use gtk::{
    glib::{self, once_cell::sync::Lazy, subclass::Signal},
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetClassSubclassExt, WidgetImpl},
    },
    CompositeTemplate, TemplateChild,
};
use gtk::{prelude::*, SignalListItemFactory};
use openscq30_lib::packets::structures::equalizer_band_offsets::EqualizerBandOffsets;
use openscq30_lib::packets::structures::equalizer_configuration::EqualizerConfiguration;
use openscq30_lib::packets::structures::preset_equalizer_profile::PresetEqualizerProfile;
use strum::IntoEnumIterator;

use crate::objects::EqualizerProfileObject;
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

    profiles: RefCell<Option<gio::ListStore>>,
    profile_objects: RefCell<Vec<EqualizerProfileObject>>,
    is_custom_profile: Cell<bool>,
}

#[gtk::template_callbacks]
impl EqualizerSettings {
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

    pub fn set_equalizer_configuration(&self, configuration: EqualizerConfiguration) {
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

            self.profile_dropdown.set_model(Some(model));
        }
    }

    #[template_callback]
    fn handle_apply_custom_equalizer(&self, _button: &gtk::Button) {
        self.obj().emit_by_name("apply-equalizer-settings", &[])
    }

    #[template_callback]
    fn handle_refresh_custom_equalizer(&self, _button: &gtk::Button) {
        self.obj().emit_by_name("refresh-equalizer-settings", &[])
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
        let model = gio::ListStore::new(EqualizerProfileObject::static_type());
        self.profiles.replace(Some(model));

        let selection_model = SingleSelection::new(self.profiles.borrow().to_owned().as_ref());
        self.profile_dropdown.set_model(Some(&selection_model));

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = gtk::Label::new(None);
            list_item.set_child(Some(&label));
        });

        factory.connect_bind(move |_, list_item| {
            let equalizer_profile_object = list_item
                .item()
                .expect("item must exist")
                .downcast::<EqualizerProfileObject>()
                .expect("the item must be an EqualizerProfileObject");

            let label = list_item
                .child()
                .expect("must have a child")
                .downcast::<gtk::Label>()
                .expect("child must be a Label");

            let name = equalizer_profile_object.name();

            label.set_label(&name);
        });

        let this = self;
        let obj = self.obj();
        self.profile_dropdown
            .connect_selected_item_notify(clone!(@weak obj, @weak this => move |_dropdown| {
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
                obj.set_equalizer_configuration(configuration);
                obj.emit_by_name("apply-equalizer-settings", &[])
            }));

        self.profile_dropdown.set_factory(Some(&factory));

        let profiles = PresetEqualizerProfile::iter()
            .map(|preset| EqualizerProfileObject::new(&preset.to_string(), preset.id().into()))
            .chain([EqualizerProfileObject::new(
                &"Custom".to_string(),
                EqualizerConfiguration::CUSTOM_PROFILE_ID.into(),
            )])
            .collect::<Vec<_>>();
        self.set_profiles(profiles);

        self.profile_dropdown
            .bind_property("selected-item", obj.as_ref(), "is-custom-profile")
            .transform_to(|_, value: EqualizerProfileObject| {
                Some(value.profile_id() as u16 == EqualizerConfiguration::CUSTOM_PROFILE_ID)
            })
            .sync_create()
            .build();

        obj.bind_property("is-custom-profile", &self.equalizer.get(), "sensitive")
            .sync_create()
            .build();
        obj.bind_property("is-custom-profile", &self.apply_button.get(), "sensitive")
            .sync_create()
            .build();
        obj.bind_property("is-custom-profile", &self.refresh_button.get(), "sensitive")
            .sync_create()
            .build();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("apply-equalizer-settings").build(),
                Signal::builder("refresh-equalizer-settings").build(),
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
