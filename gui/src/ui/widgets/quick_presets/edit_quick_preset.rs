use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::device_profile::DeviceFeatures;

use crate::objects::{GlibCustomEqualizerProfile, GlibNamedQuickPresetValue};

glib::wrapper! {
    pub struct EditQuickPreset(ObjectSubclass<imp::EditQuickPreset>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EditQuickPreset {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_quick_preset(&self, quick_preset: GlibNamedQuickPresetValue) {
        self.imp().set_quick_preset(quick_preset);
    }

    pub fn set_device_features(&self, profile: &DeviceFeatures) {
        self.imp().set_device_features(profile);
    }

    pub fn set_custom_equalizer_profiles(&self, profiles: Vec<GlibCustomEqualizerProfile>) {
        self.imp().set_custom_equalizer_profiles(profiles);
    }
}

mod imp {
    use std::{
        cell::{Cell, RefCell},
        sync::{Arc, LazyLock},
    };

    use adw::prelude::*;
    use gtk::{
        gio::{self, ListStore},
        glib::{self, clone, subclass::Signal},
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, ClosureExpression, CompositeTemplate,
    };
    use openscq30_lib::{
        device_profile::{DeviceFeatures, NoiseCancelingModeType, TransparencyModeType},
        devices::standard::structures::{
            AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, PresetEqualizerProfile,
            TransparencyMode,
        },
    };
    use strum::IntoEnumIterator;
    use tracing::instrument;

    use crate::{
        objects::{
            GlibAmbientSoundMode, GlibAmbientSoundModeValue, GlibCustomEqualizerProfile,
            GlibNamedQuickPresetValue, GlibNoiseCancelingMode, GlibNoiseCancelingModeValue,
            GlibPresetEqualizerProfile, GlibPresetEqualizerProfileValue, GlibTransparencyMode,
            GlibTransparencyModeValue,
        },
        settings::{PresetOrCustomEqualizerProfile, QuickPreset},
        APPLICATION_ID_STR,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/quick_presets/edit_quick_preset.ui")]
    pub struct EditQuickPreset {
        #[template_child]
        ambient_sound_mode_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        ambient_sound_mode_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        ambient_sound_mode: TemplateChild<adw::ComboRow>,
        #[template_child]
        transparency_mode_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        transparency_mode_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        transparency_mode: TemplateChild<adw::ComboRow>,
        #[template_child]
        noise_canceling_mode_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        noise_canceling_mode_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        noise_canceling_mode: TemplateChild<adw::ComboRow>,
        #[template_child]
        custom_noise_canceling_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        custom_noise_canceling_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        custom_noise_canceling: TemplateChild<adw::SpinRow>,
        #[template_child]
        equalizer_profile_group: TemplateChild<adw::PreferencesGroup>,
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

        quick_preset_name: RefCell<Option<Arc<str>>>,

        freeze_handle_option_changed: Cell<bool>,
    }

    #[template_callbacks]
    impl EditQuickPreset {
        #[template_callback]
        fn handle_option_changed(&self) {
            let frozen = self.freeze_handle_option_changed.get();
            if !frozen {
                tracing::trace!("quick preset option changed");
                self.send_quick_preset_update();
            }
        }
    }

    impl EditQuickPreset {
        fn send_quick_preset_update(&self) {
            let Some(quick_preset_name) = self.quick_preset_name.borrow().to_owned() else {
                return;
            };
            let quick_preset = QuickPreset {
                ambient_sound_mode: if self.ambient_sound_mode_switch.is_active() {
                    self.ambient_sound_mode
                        .selected_item()
                        .and_downcast_ref::<GlibAmbientSoundMode>()
                        .map(|ambient_sound_mode| ambient_sound_mode.ambient_sound_mode().0)
                } else {
                    None
                },
                transparency_mode: if self.transparency_mode_switch.is_active() {
                    self.transparency_mode
                        .selected_item()
                        .and_downcast_ref::<GlibTransparencyMode>()
                        .map(|transparency_mode| transparency_mode.transparency_mode().0)
                } else {
                    None
                },
                noise_canceling_mode: if self.noise_canceling_mode_switch.is_active() {
                    self.noise_canceling_mode
                        .selected_item()
                        .and_downcast_ref::<GlibNoiseCancelingMode>()
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
                            .and_downcast_ref::<GlibPresetEqualizerProfile>()
                            .map(|preset_equalizer_profile| {
                                PresetOrCustomEqualizerProfile::Preset(
                                    preset_equalizer_profile.preset_equalizer_profile().0,
                                )
                            })
                    } else {
                        self.custom_equalizer_profile
                            .selected_item()
                            .and_downcast::<GlibCustomEqualizerProfile>()
                            .map(|profile| {
                                PresetOrCustomEqualizerProfile::Custom(profile.name().into())
                            })
                    }
                } else {
                    None
                },
            };
            self.obj().emit_by_name::<()>(
                "quick-preset-changed",
                &[&GlibNamedQuickPresetValue {
                    quick_preset,
                    name: quick_preset_name,
                }],
            );
        }

        fn update_list_values<T: IsA<glib::Object>>(
            row: &adw::ComboRow,
            store: &mut ListStore,
            values: impl IntoIterator<Item = T>,
        ) {
            let selection = row.selected();
            store.remove_all();
            store.extend(values);
            row.set_selected(selection);
        }

        #[instrument(skip_all)]
        pub fn set_device_features(&self, features: &DeviceFeatures) {
            self.freeze_handle_option_changed.set(true);

            if let Some(sound_mode_profile) = features.sound_mode {
                self.ambient_sound_mode_group.set_visible(true);
                let ambient_sound_modes =
                    [AmbientSoundMode::Normal, AmbientSoundMode::Transparency]
                        .into_iter()
                        .chain(
                            (sound_mode_profile.noise_canceling_mode_type
                                != NoiseCancelingModeType::None)
                                .then_some(AmbientSoundMode::NoiseCanceling),
                        )
                        .map(GlibAmbientSoundModeValue)
                        .map(GlibAmbientSoundMode::new);
                Self::update_list_values(
                    &self.ambient_sound_mode,
                    self.ambient_sound_modes_store
                        .borrow_mut()
                        .as_mut()
                        .unwrap(),
                    ambient_sound_modes,
                );

                self.transparency_mode_group.set_visible(
                    sound_mode_profile.transparency_mode_type == TransparencyModeType::Custom,
                );

                self.noise_canceling_mode_group.set_visible(
                    sound_mode_profile.noise_canceling_mode_type != NoiseCancelingModeType::None,
                );
                let noise_canceling_modes = [
                    NoiseCancelingMode::Transport,
                    NoiseCancelingMode::Indoor,
                    NoiseCancelingMode::Outdoor,
                ]
                .into_iter()
                .chain(
                    (sound_mode_profile.noise_canceling_mode_type
                        == NoiseCancelingModeType::Custom)
                        .then_some(NoiseCancelingMode::Custom),
                )
                .map(GlibNoiseCancelingModeValue)
                .map(GlibNoiseCancelingMode::new);
                Self::update_list_values(
                    &self.noise_canceling_mode,
                    self.noise_canceling_modes_store
                        .borrow_mut()
                        .as_mut()
                        .unwrap(),
                    noise_canceling_modes,
                );

                self.custom_noise_canceling_group.set_visible(
                    sound_mode_profile.noise_canceling_mode_type == NoiseCancelingModeType::Custom,
                );
            } else {
                self.ambient_sound_mode_group.set_visible(false);
                self.transparency_mode_group.set_visible(false);
                self.noise_canceling_mode_group.set_visible(false);
                self.custom_noise_canceling_group.set_visible(false);
            }

            self.equalizer_profile_group
                .set_visible(features.num_equalizer_channels != 0);

            self.freeze_handle_option_changed.set(false);
        }

        pub fn set_custom_equalizer_profiles(&self, profiles: Vec<GlibCustomEqualizerProfile>) {
            Self::update_list_values(
                &self.custom_equalizer_profile,
                self.custom_equalizer_profiles_store
                    .borrow_mut()
                    .as_mut()
                    .unwrap(),
                profiles,
            );
        }

        pub fn set_quick_preset(&self, named_quick_preset: GlibNamedQuickPresetValue) {
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
                            item.downcast_ref::<GlibAmbientSoundMode>()
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
                            item.downcast_ref::<GlibTransparencyMode>()
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
                            item.downcast_ref::<GlibNoiseCancelingMode>()
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
                                item.downcast_ref::<GlibPresetEqualizerProfile>()
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
                                item.downcast_ref::<GlibCustomEqualizerProfile>()
                                    .unwrap()
                                    .name()
                                    == profile_name.as_ref()
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
            let refresh_profile_type_visibility = clone!(
                #[weak(rename_to=this)]
                self,
                move || {
                    let is_active = this.equalizer_profile_switch.is_active();
                    let selected_type_index = this.equalizer_profile_type.selected();
                    this.preset_equalizer_profile
                        .set_visible(is_active && selected_type_index == 0);
                    this.custom_equalizer_profile
                        .set_visible(is_active && selected_type_index == 1);
                }
            );
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

            let ambient_sound_modes = gio::ListStore::new::<GlibAmbientSoundMode>();
            let mut transparency_modes = gio::ListStore::new::<GlibTransparencyMode>();
            transparency_modes.extend(
                [
                    TransparencyMode::FullyTransparent,
                    TransparencyMode::VocalMode,
                ]
                .map(GlibTransparencyModeValue)
                .map(GlibTransparencyMode::new),
            );
            let noise_canceling_modes = gio::ListStore::new::<GlibNoiseCancelingMode>();
            let preset_equalizer_profiles = gio::ListStore::new::<GlibPresetEqualizerProfile>();
            let custom_equalizer_profiles = gio::ListStore::new::<GlibCustomEqualizerProfile>();
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

            self.ambient_sound_mode
                .set_expression(Some(&ClosureExpression::with_callback(
                    gtk::Expression::NONE,
                    |args| {
                        let ambient_sound_mode: GlibAmbientSoundMode = args[0].get().unwrap();
                        glib::dpgettext2(
                            Some(APPLICATION_ID_STR),
                            "ambient sound mode",
                            ambient_sound_mode.ambient_sound_mode().0.as_ref(),
                        )
                    },
                )));
            self.transparency_mode
                .set_expression(Some(&ClosureExpression::with_callback(
                    gtk::Expression::NONE,
                    |args| {
                        let transparency_mode: GlibTransparencyMode = args[0].get().unwrap();
                        glib::dpgettext2(
                            Some(APPLICATION_ID_STR),
                            "transparency mode",
                            transparency_mode.transparency_mode().0.as_ref(),
                        )
                    },
                )));
            self.noise_canceling_mode
                .set_expression(Some(&ClosureExpression::with_callback(
                    gtk::Expression::NONE,
                    |args| {
                        let noise_canceling_mode: GlibNoiseCancelingMode = args[0].get().unwrap();
                        glib::dpgettext2(
                            Some(APPLICATION_ID_STR),
                            "noise canceling mode",
                            noise_canceling_mode.noise_canceling_mode().0.as_ref(),
                        )
                    },
                )));
            self.preset_equalizer_profile
                .set_expression(Some(&ClosureExpression::with_callback(
                    gtk::Expression::NONE,
                    |args| {
                        let preset_equalizer_profile: GlibPresetEqualizerProfile =
                            args[0].get().unwrap();
                        glib::dpgettext2(
                            Some(APPLICATION_ID_STR),
                            "preset equalizer profile",
                            preset_equalizer_profile
                                .preset_equalizer_profile()
                                .0
                                .as_ref(),
                        )
                    },
                )));
            self.custom_equalizer_profile
                .set_expression(Some(&ClosureExpression::with_callback(
                    gtk::Expression::NONE,
                    |args| {
                        let custom_equalizer_profile: GlibCustomEqualizerProfile =
                            args[0].get().unwrap();
                        custom_equalizer_profile.name()
                    },
                )));

            let mut preset_equalizer_profiles = self.preset_equalizer_profiles_store.borrow_mut();
            let preset_equalizer_profiles = preset_equalizer_profiles.as_mut().unwrap();
            preset_equalizer_profiles.remove_all();
            preset_equalizer_profiles.extend(PresetEqualizerProfile::iter().map(|profile| {
                GlibPresetEqualizerProfile::new(GlibPresetEqualizerProfileValue(profile))
            }));
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
                vec![Signal::builder("quick-preset-changed")
                    .param_types([GlibNamedQuickPresetValue::static_type()])
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }
    impl WidgetImpl for EditQuickPreset {}
    impl BoxImpl for EditQuickPreset {}
}
