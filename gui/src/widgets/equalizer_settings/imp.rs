use std::{cell::RefCell, time::Duration};

use gtk::{
    gio::{self},
    glib::{self, clone, once_cell::sync::Lazy, subclass::Signal, timeout_future, MainContext},
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
use once_cell::unsync::OnceCell;
use openscq30_lib::packets::structures::{
    EqualizerBandOffsets, EqualizerConfiguration, PresetEqualizerProfile,
};
use strum::IntoEnumIterator;

use crate::objects::{CustomEqualizerProfileObject, EqualizerProfileObject};
use crate::widgets::Equalizer;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/OpenSCQ30/equalizer_settings/template.ui")]
pub struct EqualizerSettings {
    #[template_child]
    pub equalizer: TemplateChild<Equalizer>,
    #[template_child]
    pub profile_dropdown: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub custom_profile_selection: TemplateChild<gtk::Box>,
    #[template_child]
    pub custom_profile_dropdown: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub create_custom_profile_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub delete_custom_profile_button: TemplateChild<gtk::Button>,

    profiles: OnceCell<gio::ListStore>,
    custom_profiles: OnceCell<gio::ListStore>,

    update_signal_debounce_handle: RefCell<Option<glib::JoinHandle<()>>>,
}

#[gtk::template_callbacks]
impl EqualizerSettings {
    #[template_callback]
    fn handle_create_custom_profile(&self, _button: &gtk::Button) {
        self.obj().emit_by_name(
            "create-custom-equalizer-profile",
            &[&CustomEqualizerProfileObject::new(
                "", // TODO use a different object that doesn't have a name field
                self.equalizer.volumes(),
            )],
        )
    }

    #[template_callback]
    fn handle_delete_custom_profile(&self, _button: &gtk::Button) {
        if let Some(profile) = self.custom_profile_dropdown.selected_item() {
            self.obj()
                .emit_by_name::<()>("delete-custom-equalizer-profile", &[&profile]);
        }
    }

    #[template_callback]
    fn handle_volumes_changed(&self, _equalizer: &Equalizer) {
        self.update_custom_profile_selection();
        let context = MainContext::default();
        let mut maybe_handle = self.update_signal_debounce_handle.borrow_mut();
        if let Some(handle) = &*maybe_handle {
            handle.abort();
        }
        // apply-equalizer-settings fires instantly when changing the preset profile, so we only need to be concerned
        // with custom profiles here.
        if self.is_custom_profile() {
            *maybe_handle = Some(
                context.spawn_local(clone!(@weak self as this => async move {
                    timeout_future(Duration::from_secs(1)).await;
                    *this.update_signal_debounce_handle.borrow_mut() = None;
                    this.obj().emit_by_name::<()>("apply-equalizer-settings", &[]);
                })),
            );
        }
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        if self.is_custom_profile() {
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
        let profile_index = self
            .profiles
            .get()
            .expect("profiles should have been intitialized already")
            .iter::<EqualizerProfileObject>()
            .position(|profile| profile.unwrap().profile_id() as u16 == configuration.profile_id())
            .unwrap_or(0)
            .try_into()
            .expect("could not convert usize to u32");
        self.profile_dropdown.set_selected(profile_index);
    }

    fn set_profiles(&self, profiles: Vec<EqualizerProfileObject>) {
        if let Some(model) = self.profiles.get() {
            model.remove_all();
            model.extend_from_slice(&profiles);
        }
    }

    fn set_up_custom_profile(&self) {
        self.set_up_custom_profile_selection_model();
        self.set_up_custom_profile_expression();
        self.set_up_custom_profile_selection_changed_handler();
    }

    fn set_up_custom_profile_selection_model(&self) {
        let model = gio::ListStore::new(CustomEqualizerProfileObject::static_type());
        self.custom_profile_dropdown.set_model(Some(&model));
        self.custom_profiles
            .set(model)
            .expect("set up should only run once");
    }

    fn set_up_custom_profile_expression(&self) {
        self.custom_profile_dropdown
            .set_expression(Some(PropertyExpression::new(
                CustomEqualizerProfileObject::static_type(),
                None::<Expression>,
                "name",
            )));
    }

    fn set_up_custom_profile_selection_changed_handler(&self) {
        self.custom_profile_dropdown.connect_selected_item_notify(
            clone!(@weak self as this => move |_dropdown| {
                let maybe_selected_item = this.custom_profile_dropdown.selected_item()
                    .map(|item| item.downcast::<CustomEqualizerProfileObject>().unwrap());
                if let Some(selected_item) = maybe_selected_item {
                    this.obj().emit_by_name("custom-equalizer-profile-selected", &[&selected_item])
                }
            }),
        );
    }

    pub fn set_custom_profiles(&self, mut profiles: Vec<CustomEqualizerProfileObject>) {
        if let Some(model) = self.custom_profiles.get() {
            profiles.sort_unstable_by_key(|left| left.name());
            // Notifications need to be frozen to prevent the selection changes while removing and adding items from
            // causing the profile to change. We can't force having no selection when adding new items, so it
            // will change the selection to the newly added item. We can set it back to what it's supposed to be
            // afterwards.
            let _notify_freeze_guard = self.custom_profile_dropdown.freeze_notify();
            model.remove_all();
            model.extend_from_slice(&profiles);
            self.update_custom_profile_selection();
        }
    }

    fn update_custom_profile_selection(&self) {
        match self.custom_profiles.get() {
            Some(custom_profiles) if self.is_custom_profile() => {
                let volumes = self.equalizer.volumes();
                let custom_profile_index = custom_profiles
                    .iter::<CustomEqualizerProfileObject>()
                    .enumerate()
                    .find(|(_i, profile)| profile.as_ref().unwrap().volume_offsets() == volumes)
                    .map(|(i, _profile)| i as u32)
                    .unwrap_or(u32::MAX);

                self.custom_profile_dropdown
                    .set_selected(custom_profile_index);
            }
            _ => {
                self.custom_profile_dropdown.set_selected(u32::MAX);
            }
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
        self.profile_dropdown.set_model(Some(&model));
        self.profiles
            .set(model)
            .expect("set up should only run once");
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

    fn is_custom_profile(&self) -> bool {
        self.profile_dropdown
            .selected_item()
            .map(|item| {
                item.downcast::<EqualizerProfileObject>()
                    .expect("must be EqualizerProfileObject")
            })
            .map(|profile| profile.profile_id() as u16 == EqualizerConfiguration::CUSTOM_PROFILE_ID)
            .unwrap_or(false)
    }

    fn set_up_preset_profile_disabled_fields(&self) {
        let is_custom_profile_transform = |_, value: EqualizerProfileObject| {
            Some(value.profile_id() as u16 == EqualizerConfiguration::CUSTOM_PROFILE_ID)
        };

        self.profile_dropdown
            .bind_property(
                "selected-item",
                &self.custom_profile_selection.get(),
                "sensitive",
            )
            .transform_to(is_custom_profile_transform)
            .sync_create()
            .build();

        self.profile_dropdown
            .bind_property("selected-item", &self.equalizer.get(), "sensitive")
            .transform_to(is_custom_profile_transform)
            .sync_create()
            .build();
    }

    fn set_up_preset_profile_items(&self) {
        let custom_profile_iter = [EqualizerProfileObject::new(
            "Custom",
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
                Signal::builder("custom-equalizer-profile-selected")
                    .param_types([CustomEqualizerProfileObject::static_type()])
                    .build(),
                Signal::builder("create-custom-equalizer-profile")
                    .param_types([CustomEqualizerProfileObject::static_type()])
                    .build(),
                Signal::builder("delete-custom-equalizer-profile")
                    .param_types([CustomEqualizerProfileObject::static_type()])
                    .build(),
            ]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for EqualizerSettings {}
impl BoxImpl for EqualizerSettings {}
