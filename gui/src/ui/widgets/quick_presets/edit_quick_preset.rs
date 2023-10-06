use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::packets::structures::DeviceFeatureFlags;

use crate::{
    actions::Action,
    objects::{CustomEqualizerProfileObject, NamedQuickPreset},
};

glib::wrapper! {
    pub struct EditQuickPreset(ObjectSubclass<imp::EditQuickPreset>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EditQuickPreset {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: Sender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_quick_preset(&self, quick_preset: NamedQuickPreset) {
        self.imp().set_quick_preset(quick_preset);
    }

    pub fn set_device_feature_flags(&self, flags: DeviceFeatureFlags) {
        self.imp().set_device_feature_flags(flags);
    }

    pub fn set_custom_equalizer_profiles(&self, profiles: Vec<CustomEqualizerProfileObject>) {
        self.imp().set_custom_equalizer_profiles(profiles);
    }
}

mod imp {
    use std::cell::{OnceCell, RefCell};

    use adw::prelude::*;
    use gtk::{
        gio::{self, ListStore},
        glib::{self, clone, Object, Sender},
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, ClosureExpression, CompositeTemplate,
    };
    use openscq30_lib::packets::structures::{
        AmbientSoundMode, CustomNoiseCanceling, DeviceFeatureFlags, NoiseCancelingMode,
        PresetEqualizerProfile, TransparencyMode,
    };
    use strum::IntoEnumIterator;

    use crate::{
        actions::Action,
        objects::{
            BoxedAmbientSoundMode, BoxedNoiseCancelingMode, BoxedPresetEqualizerProfile,
            BoxedTransparencyMode, CustomEqualizerProfileObject, NamedQuickPreset,
        },
        settings::{PresetOrCustomEqualizerProfile, QuickPreset},
        ui::widgets::quick_presets::{
            ambient_sound_mode_model::AmbientSoundModeModel,
            noise_canceling_mode_model::NoiseCancelingModeModel,
            preset_equalizer_profile_model::PresetEqualizerProfileModel,
            transparency_mode_model::TransparencyModeModel,
        },
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/quick_presets/edit_quick_preset.ui")]
    pub struct EditQuickPreset {
        #[template_child]
        ambient_sound_mode_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        ambient_sound_mode: TemplateChild<adw::ComboRow>,
        #[template_child]
        transparency_mode_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        transparency_mode: TemplateChild<adw::ComboRow>,
        #[template_child]
        noise_canceling_mode_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        noise_canceling_mode: TemplateChild<adw::ComboRow>,
        #[template_child]
        custom_noise_canceling_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        custom_noise_canceling: TemplateChild<adw::SpinRow>,
        #[template_child]
        equalizer_profile_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        equalizer_profile_type: TemplateChild<adw::ComboRow>,
        #[template_child]
        preset_equalizer_profile: TemplateChild<adw::ComboRow>,
        #[template_child]
        custom_equalizer_profile: TemplateChild<adw::ComboRow>,

        ambient_sound_modes_store: RefCell<Option<ListStore>>,
        transparency_modes_store: RefCell<Option<ListStore>>,
        noise_canceling_modes_store: RefCell<Option<ListStore>>,
        preset_equalizer_profiles_store: RefCell<Option<ListStore>>,
        custom_equalizer_profiles_store: RefCell<Option<ListStore>>,

        quick_preset_name: RefCell<Option<String>>,

        sender: OnceCell<Sender<Action>>,
    }

    #[template_callbacks]
    impl EditQuickPreset {
        #[template_callback]
        fn handle_option_changed(&self) {
            self.send_quick_preset_update();
        }
    }

    impl EditQuickPreset {
        pub fn set_sender(&self, sender: Sender<Action>) {
            self.sender.set(sender.clone()).unwrap();
        }

        fn send_quick_preset_update(&self) {
            let Some(quick_preset_name) = self.quick_preset_name.borrow().to_owned() else {
                return;
            };
            let quick_preset = QuickPreset {
                ambient_sound_mode: if self.ambient_sound_mode_switch.is_active() {
                    self.ambient_sound_mode
                        .selected_item()
                        .and_downcast_ref::<AmbientSoundModeModel>()
                        .map(|ambient_sound_mode| ambient_sound_mode.ambient_sound_mode().0)
                } else {
                    None
                },
                transparency_mode: if self.transparency_mode_switch.is_active() {
                    self.transparency_mode
                        .selected_item()
                        .and_downcast_ref::<TransparencyModeModel>()
                        .map(|transparency_mode| transparency_mode.transparency_mode().0)
                } else {
                    None
                },
                noise_canceling_mode: if self.noise_canceling_mode_switch.is_active() {
                    self.noise_canceling_mode
                        .selected_item()
                        .and_downcast_ref::<NoiseCancelingModeModel>()
                        .map(|noise_canceling_mode| noise_canceling_mode.noise_canceling_mode().0)
                } else {
                    None
                },
                custom_noise_canceling: if self.custom_noise_canceling_switch.is_active() {
                    Some(CustomNoiseCanceling::new(
                        self.custom_noise_canceling.value() as u8,
                    ))
                } else {
                    None
                },
                equalizer_profile: if self.equalizer_profile_switch.is_active() {
                    if self.equalizer_profile_type.selected() == 0 {
                        self.preset_equalizer_profile
                            .selected_item()
                            .and_downcast_ref::<PresetEqualizerProfileModel>()
                            .map(|preset_equalizer_profile| {
                                PresetOrCustomEqualizerProfile::Preset(
                                    preset_equalizer_profile.preset_equalizer_profile().0,
                                )
                            })
                    } else {
                        self.custom_equalizer_profile
                            .selected_item()
                            .and_downcast::<CustomEqualizerProfileObject>()
                            .map(|profile| PresetOrCustomEqualizerProfile::Custom(profile.name()))
                    }
                } else {
                    None
                },
            };
            self.sender
                .get()
                .unwrap()
                .send(Action::CreateQuickPreset(NamedQuickPreset {
                    quick_preset,
                    name: quick_preset_name,
                }))
                .unwrap();
        }

        pub fn set_device_feature_flags(&self, flags: DeviceFeatureFlags) {
            let mut ambient_sound_modes = self.ambient_sound_modes_store.borrow_mut();
            let ambient_sound_modes = ambient_sound_modes.as_mut().unwrap();
            ambient_sound_modes.remove_all();
            ambient_sound_modes.append(&AmbientSoundModeModel::new(
                BoxedAmbientSoundMode(AmbientSoundMode::Normal),
                "Normal",
            ));
            ambient_sound_modes.append(&AmbientSoundModeModel::new(
                BoxedAmbientSoundMode(AmbientSoundMode::Transparency),
                "Transparency",
            ));
            if flags.contains(DeviceFeatureFlags::NOISE_CANCELING_MODE) {
                ambient_sound_modes.append(&AmbientSoundModeModel::new(
                    BoxedAmbientSoundMode(AmbientSoundMode::NoiseCanceling),
                    "Noise Canceling",
                ));
            }

            let mut transparency_modes = self.transparency_modes_store.borrow_mut();
            let transparency_modes = transparency_modes.as_mut().unwrap();
            transparency_modes.remove_all();
            transparency_modes.append(&TransparencyModeModel::new(
                BoxedTransparencyMode(TransparencyMode::FullyTransparent),
                "Fully Transparent",
            ));
            transparency_modes.append(&TransparencyModeModel::new(
                BoxedTransparencyMode(TransparencyMode::VocalMode),
                "Vocal Mode",
            ));

            let mut noise_canceling_modes = self.noise_canceling_modes_store.borrow_mut();
            let noise_canceling_modes = noise_canceling_modes.as_mut().unwrap();
            noise_canceling_modes.remove_all();
            noise_canceling_modes.append(&NoiseCancelingModeModel::new(
                BoxedNoiseCancelingMode(NoiseCancelingMode::Transport),
                "Transport",
            ));
            noise_canceling_modes.append(&NoiseCancelingModeModel::new(
                BoxedNoiseCancelingMode(NoiseCancelingMode::Indoor),
                "Indoor",
            ));
            noise_canceling_modes.append(&NoiseCancelingModeModel::new(
                BoxedNoiseCancelingMode(NoiseCancelingMode::Outdoor),
                "Outdoor",
            ));
            if flags.contains(DeviceFeatureFlags::CUSTOM_NOISE_CANCELING) {
                noise_canceling_modes.append(&NoiseCancelingModeModel::new(
                    BoxedNoiseCancelingMode(NoiseCancelingMode::Custom),
                    "Custom",
                ));
            }
        }

        pub fn set_custom_equalizer_profiles(&self, profiles: Vec<CustomEqualizerProfileObject>) {
            let mut profiles_store = self.custom_equalizer_profiles_store.borrow_mut();
            let profiles_store = profiles_store.as_mut().unwrap();

            profiles_store.remove_all();
            profiles_store.extend(profiles.iter());
        }

        pub fn set_quick_preset(&self, named_quick_preset: NamedQuickPreset) {
            *self.quick_preset_name.borrow_mut() = None;
            let quick_preset = named_quick_preset.quick_preset;

            self.ambient_sound_mode_switch
                .set_active(quick_preset.ambient_sound_mode.is_some());
            if let Some(ambient_sound_mode) = quick_preset.ambient_sound_mode {
                self.ambient_sound_mode.set_selected(
                    self.ambient_sound_modes_store
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .find_with_equal_func(|item| {
                            item.downcast_ref::<AmbientSoundModeModel>()
                                .unwrap()
                                .ambient_sound_mode()
                                .0
                                == ambient_sound_mode
                        })
                        .unwrap_or_default(),
                );
            }
            self.transparency_mode_switch
                .set_active(quick_preset.transparency_mode.is_some());
            if let Some(transparency_mode) = quick_preset.transparency_mode {
                self.transparency_mode.set_selected(
                    self.transparency_modes_store
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .find_with_equal_func(|item| {
                            item.downcast_ref::<TransparencyModeModel>()
                                .unwrap()
                                .transparency_mode()
                                .0
                                == transparency_mode
                        })
                        .unwrap_or_default(),
                );
            }
            self.noise_canceling_mode_switch
                .set_active(quick_preset.noise_canceling_mode.is_some());
            if let Some(noise_canceling_mode) = quick_preset.noise_canceling_mode {
                self.noise_canceling_mode.set_selected(
                    self.noise_canceling_modes_store
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .find_with_equal_func(|item| {
                            item.downcast_ref::<NoiseCancelingModeModel>()
                                .unwrap()
                                .noise_canceling_mode()
                                .0
                                == noise_canceling_mode
                        })
                        .unwrap_or_default(),
                );
            }
            self.custom_noise_canceling_switch
                .set_active(quick_preset.custom_noise_canceling.is_some());
            self.custom_noise_canceling.set_value(
                quick_preset
                    .custom_noise_canceling
                    .unwrap_or_default()
                    .value() as f64,
            );
            self.equalizer_profile_switch
                .set_active(quick_preset.equalizer_profile.is_some());
            match quick_preset.equalizer_profile {
                Some(PresetOrCustomEqualizerProfile::Preset(profile)) => {
                    self.equalizer_profile_type.set_selected(0);
                    self.preset_equalizer_profile.set_selected(
                        self.preset_equalizer_profiles_store
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .find_with_equal_func(|item| {
                                item.downcast_ref::<PresetEqualizerProfileModel>()
                                    .unwrap()
                                    .preset_equalizer_profile()
                                    .0
                                    == profile
                            })
                            .unwrap_or_default(),
                    )
                }
                Some(PresetOrCustomEqualizerProfile::Custom(profile_name)) => {
                    self.equalizer_profile_type.set_selected(1);
                    self.custom_equalizer_profile.set_selected(
                        self.custom_equalizer_profiles_store
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .find_with_equal_func(|item| {
                                item.downcast_ref::<CustomEqualizerProfileObject>()
                                    .unwrap()
                                    .name()
                                    == profile_name
                            })
                            .unwrap_or_default(),
                    )
                }
                None => {
                    self.equalizer_profile_type.set_selected(0);
                    self.preset_equalizer_profile.set_selected(0);
                    self.custom_equalizer_profile.set_selected(0);
                }
            }

            *self.quick_preset_name.borrow_mut() = Some(named_quick_preset.name);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EditQuickPreset {
        const NAME: &'static str = "OpenSCQ30EditQuickPreset";
        type Type = super::EditQuickPreset;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for EditQuickPreset {
        fn constructed(&self) {
            let refresh_profile_type_visibility = clone!(@weak self as this => move || {
                let is_active = this.equalizer_profile_switch.is_active();
                let selected_type_index = this.equalizer_profile_type.selected();
                this.preset_equalizer_profile.set_visible(is_active && selected_type_index == 0);
                this.custom_equalizer_profile.set_visible(is_active && selected_type_index == 1);
            });
            refresh_profile_type_visibility();
            {
                let refresh_profile_type_visibility = refresh_profile_type_visibility.clone();
                self.equalizer_profile_switch
                    .connect_notify_local(Some("active"), move |_, _| {
                        refresh_profile_type_visibility();
                    });
            }
            self.equalizer_profile_type
                .connect_notify_local(Some("selected"), move |_, _| {
                    refresh_profile_type_visibility();
                });

            let ambient_sound_modes = gio::ListStore::new::<AmbientSoundModeModel>();
            let transparency_modes = gio::ListStore::new::<TransparencyModeModel>();
            let noise_canceling_modes = gio::ListStore::new::<NoiseCancelingModeModel>();
            let preset_equalizer_profiles = gio::ListStore::new::<PresetEqualizerProfileModel>();
            let custom_equalizer_profiles = gio::ListStore::new::<CustomEqualizerProfileObject>();
            self.ambient_sound_mode
                .set_model(Some(&ambient_sound_modes));
            self.transparency_mode.set_model(Some(&transparency_modes));
            self.noise_canceling_mode
                .set_model(Some(&noise_canceling_modes));
            self.preset_equalizer_profile
                .set_model(Some(&preset_equalizer_profiles));
            self.custom_equalizer_profile
                .set_model(Some(&custom_equalizer_profiles));
            *self.ambient_sound_modes_store.borrow_mut() = Some(ambient_sound_modes);
            *self.transparency_modes_store.borrow_mut() = Some(transparency_modes);
            *self.noise_canceling_modes_store.borrow_mut() = Some(noise_canceling_modes);
            *self.preset_equalizer_profiles_store.borrow_mut() = Some(preset_equalizer_profiles);
            *self.custom_equalizer_profiles_store.borrow_mut() = Some(custom_equalizer_profiles);

            let name_expression = ClosureExpression::with_callback(gtk::Expression::NONE, |args| {
                let object: Object = args[0].get().unwrap();
                let name: String = object.property("name");
                name
            });
            self.ambient_sound_mode
                .set_expression(Some(&name_expression));
            self.transparency_mode
                .set_expression(Some(&name_expression));
            self.noise_canceling_mode
                .set_expression(Some(&name_expression));
            self.preset_equalizer_profile
                .set_expression(Some(&name_expression));
            self.custom_equalizer_profile
                .set_expression(Some(&name_expression));

            let mut preset_equalizer_profiles = self.preset_equalizer_profiles_store.borrow_mut();
            let preset_equalizer_profiles = preset_equalizer_profiles.as_mut().unwrap();
            preset_equalizer_profiles.remove_all();
            preset_equalizer_profiles.extend(PresetEqualizerProfile::iter().map(|profile| {
                PresetEqualizerProfileModel::new(
                    BoxedPresetEqualizerProfile(profile),
                    profile.as_ref(),
                )
            }));
        }
    }
    impl WidgetImpl for EditQuickPreset {}
    impl BoxImpl for EditQuickPreset {}
}
