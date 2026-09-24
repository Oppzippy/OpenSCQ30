mod controls;
mod legacy_migration;
mod quick_presets;

use std::{
    borrow::Cow,
    collections::{HashMap, hash_map},
    path::PathBuf,
};

use cosmic::{
    Element, Task,
    app::context_drawer::ContextDrawer,
    iced::{Length, keyboard},
    widget::{self, menu::KeyBind, nav_bar},
};
use legacy_migration::LegacyMigrationModel;
use openscq30_i18n::Translate;
use openscq30_lib::{
    connection::ConnectionStatus,
    quick_presets::QuickPresetsHandler,
    settings::{self, CategoryId, Setting, SettingId, Value},
};
use tracing::{Instrument, debug};

use crate::{
    app::DebugOpenSCQ30Device,
    equalizer_line, fl, handle_soft_error,
    openscq30_v1_migration::{self, LegacyEqualizerProfile},
    throttle,
    utils::coalesce_result,
};

#[derive(Debug, Clone)]
pub enum Message {
    QuickPresets(quick_presets::Message),
    Throttle(throttle::Message),
    SetSetting(SettingId, Value),
    SetEqualizerBand(SettingId, u8, i16),
    RefreshSettings,
    Warning(String),
    CancelDialog,
    ShowModifiableSelectAddDialog(SettingId),
    ShowModifiableSelectRemoveDialog(SettingId),
    ModifiableSelectAddDialogSubmit(Option<String>),
    ModifiableSelectAddDialogSetName(String),
    ModifiableSelectRemoveDialogSubmit,
    AddLegacyEqualizerMigrationPage(HashMap<String, LegacyEqualizerProfile>),
    LegacyMigration(legacy_migration::Message),
    None,
    CopyToClipboard(String),
    SetImportString(SettingId, String),
    AskConfirmImportString(SettingId, String),
    ConfirmImportString,
    Disconnect,
    SetTimeTwelveHour(SettingId, String),
    SetTimeMinute(SettingId, String),
    SetTimePm(SettingId, bool),
    RefreshTimeTwelveHour(SettingId),
}

impl From<quick_presets::Message> for Message {
    fn from(message: quick_presets::Message) -> Self {
        Self::QuickPresets(message)
    }
}

impl From<throttle::Message> for Message {
    fn from(message: throttle::Message) -> Self {
        Self::Throttle(message)
    }
}

#[must_use]
pub enum Action {
    Task(Task<Message>),
    Warning(String),
    FocusTextInput(widget::Id),
    None,
    Disconnect,
}

pub struct DeviceSettingsModel {
    device: DebugOpenSCQ30Device,
    nav_model: nav_bar::Model,
    settings_order: Vec<SettingId>,
    settings: HashMap<SettingId, SettingUiState>,
    dialog: Option<Dialog>,
    legacy_equalizer_migration: Option<legacy_migration::LegacyMigrationModel>,
    quick_presets_model: quick_presets::QuickPresetsModel,
    throttle: throttle::Throttle,
    key_binds: HashMap<KeyBind, KeyBindAction>,
}

#[derive(Debug)]
struct SettingUiState {
    setting_id: SettingId,
    translated_name: String,
    variant_state: SettingVariantUiState,
}

#[derive(Debug)]
enum SettingVariantUiState {
    Toggle {
        value: bool,
    },
    I32Range {
        setting: settings::Range<i32>,
        value: i32,
    },
    Select {
        setting: settings::Select,
        value: Cow<'static, str>,
    },
    OptionalSelect {
        setting: settings::Select,
        value: Option<Cow<'static, str>>,
    },
    ModifiableSelect {
        setting: settings::Select,
        value: Option<Cow<'static, str>>,
    },
    MultiSelect {
        setting: settings::Select,
        values: Vec<Cow<'static, str>>,
    },
    MultiSelectWithRemove {
        setting: settings::Select,
        values: Vec<Cow<'static, str>>,
    },
    Equalizer {
        setting: settings::Equalizer,
        read_only: bool,
        value: Vec<i16>,
    },
    PresetEqualizerProfileSelect {
        select: settings::Select,
        value: Option<Cow<'static, str>>,
    },
    Information {
        translated_value: String,
    },
    ImportString {
        text: String,
        confirmation_message: Option<String>,
    },
    HueColorPicker {
        /// in degrees, 0 to 360
        hue: f32,
    },
    Action,
    TimeOfDay {
        minutes_after_midnight: MinutesAfterMidnight,
        hour_text: Option<String>,
        minute_text: Option<String>,
    },
}

impl From<Setting> for SettingVariantUiState {
    fn from(setting: Setting) -> Self {
        match setting {
            Setting::Toggle { value } => Self::Toggle { value },
            Setting::I32Range { setting, value } => Self::I32Range { setting, value },
            Setting::Select { setting, value } => Self::Select { setting, value },
            Setting::OptionalSelect { setting, value } => Self::OptionalSelect { setting, value },
            Setting::ModifiableSelect { setting, value } => {
                Self::ModifiableSelect { setting, value }
            }
            Setting::MultiSelect { setting, values } => Self::MultiSelect { setting, values },
            Setting::MultiSelectWithRemove { setting, values } => {
                Self::MultiSelectWithRemove { setting, values }
            }
            Setting::Equalizer {
                setting,
                read_only,
                value,
            } => Self::Equalizer {
                setting,
                read_only,
                value,
            },
            Setting::PresetEqualizerProfileSelect {
                equalizer: _,
                select,
                presets: _,
                value,
            } => Self::PresetEqualizerProfileSelect { select, value },
            Setting::Information {
                value: _,
                translated_value,
            } => Self::Information { translated_value },
            Setting::ImportString {
                confirmation_message,
            } => Self::ImportString {
                text: String::new(),
                confirmation_message,
            },
            Setting::HueColorPicker { hue } => Self::HueColorPicker { hue },
            Setting::Action => Self::Action,
            Setting::TimeOfDay {
                minutes_after_midnight,
            } => Self::TimeOfDay {
                minutes_after_midnight: MinutesAfterMidnight(minutes_after_midnight),
                hour_text: None,
                minute_text: None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct MinutesAfterMidnight(i32);

impl MinutesAfterMidnight {
    fn twenty_four_hour(self) -> i32 {
        self.0 / 60
    }

    fn twelve_hour(self) -> i32 {
        let twelve_hour_from_zero = self.twenty_four_hour() % 12;
        if twelve_hour_from_zero == 0 {
            12
        } else {
            twelve_hour_from_zero
        }
    }

    fn minute(self) -> i32 {
        self.0 % 60
    }

    fn with_twenty_four_hour(self, twenty_four_hour: i32) -> Self {
        let twenty_four_hour = twenty_four_hour.clamp(0, 23);
        Self(twenty_four_hour * 60 + self.minute())
    }

    fn with_twelve_hour(self, twelve_hour: i32) -> Self {
        let twelve_hour = twelve_hour.clamp(1, 12);
        let twelve_hour_from_zero = if twelve_hour == 12 { 0 } else { twelve_hour };
        let pm_offset = self.is_pm().then_some(12 * 60).unwrap_or_default();
        Self(pm_offset + twelve_hour_from_zero * 60 + self.minute())
    }

    fn with_minute(self, minute: i32) -> Self {
        let minute = minute.clamp(0, 59);
        Self(self.twenty_four_hour() * 60 + minute)
    }

    fn is_pm(self) -> bool {
        self.twenty_four_hour() >= 12
    }

    fn with_pm(self, is_pm: bool) -> Self {
        if is_pm && !self.is_pm() {
            self.with_twenty_four_hour(self.twenty_four_hour() + 12)
        } else if !is_pm && self.is_pm() {
            self.with_twenty_four_hour(self.twenty_four_hour() - 12)
        } else {
            self
        }
    }
}

impl SettingVariantUiState {
    fn update(&mut self, setting: Setting) {
        // We only need to cover the cases where self and setting are the same variant and there is
        // state that will not be reconstructed by Self::from.
        // For the variant mismatch case, we can throw away any variant specific state.
        match (self, setting) {
            (
                Self::ImportString {
                    text: _,
                    confirmation_message,
                },
                Setting::ImportString {
                    confirmation_message: new_confirmation_message,
                },
            ) => {
                *confirmation_message = new_confirmation_message;
            }
            (
                Self::TimeOfDay {
                    minutes_after_midnight,
                    ..
                },
                Setting::TimeOfDay {
                    minutes_after_midnight: new_minutes_after_midnight,
                },
            ) if minutes_after_midnight.0 == new_minutes_after_midnight => {
                // Don't reset text inputs if the time hasn't changed
            }
            (this, setting) => *this = setting.into(),
        }
    }
}

enum Dialog {
    ModifiableSelectAdd(SettingId, String),
    ModifiableSelectRemove(SettingId, Cow<'static, str>),
    ImportStringConfirm(SettingId, String),
}

enum CustomCategory {
    QuickPresets,
    LegacyEqualizerMigration,
}

enum SettingDisplayKind<'a, Message> {
    Single(Element<'a, Message>),
    Vec(Vec<Element<'a, Message>>),
    ListButton(widget::list::ListButton<'a, Message>),
    ListButtonVec(Vec<widget::list::ListButton<'a, Message>>),
}

impl<'a, Message> From<Element<'a, Message>> for SettingDisplayKind<'a, Message> {
    fn from(value: Element<'a, Message>) -> Self {
        Self::Single(value)
    }
}

impl<'a, Message> From<Vec<Element<'a, Message>>> for SettingDisplayKind<'a, Message> {
    fn from(value: Vec<Element<'a, Message>>) -> Self {
        Self::Vec(value)
    }
}

impl<'a, Message> From<widget::list::ListButton<'a, Message>> for SettingDisplayKind<'a, Message> {
    fn from(value: widget::list::ListButton<'a, Message>) -> Self {
        Self::ListButton(value)
    }
}

impl<'a, Message> From<Vec<widget::list::ListButton<'a, Message>>>
    for SettingDisplayKind<'a, Message>
{
    fn from(value: Vec<widget::list::ListButton<'a, Message>>) -> Self {
        Self::ListButtonVec(value)
    }
}

impl DeviceSettingsModel {
    pub fn new(
        device: DebugOpenSCQ30Device,
        quick_presets_handler: QuickPresetsHandler,
        config_dir: PathBuf,
    ) -> (Self, Task<Message>) {
        let mut nav_model = nav_bar::Model::default();
        for category in device.categories() {
            nav_model.insert().text(category.translate()).data(category);
        }
        nav_model
            .insert()
            .text(fl!("quick-presets"))
            .data(CustomCategory::QuickPresets);
        nav_model.activate_position(0);

        // watch will close when we drop the device, so this will clean itself up
        let mut watch = device.0.watch_for_changes();
        let stream = cosmic::iced::stream::channel(1, async move |mut output| {
            while watch.changed().await.is_ok() {
                match output.try_send(Message::RefreshSettings) {
                    Err(err) if err.is_disconnected() => return,
                    _ => (),
                }
            }
            debug!("stopping state change watcher task");
        });

        let (quick_presets_model, quick_presets_refresh_task) =
            quick_presets::QuickPresetsModel::new(device.clone(), quick_presets_handler);

        let mut connection_status = device.0.connection_status();
        let watch_for_disconnect_task = Task::future(
            async move {
                loop {
                    if matches!(*connection_status.borrow(), ConnectionStatus::Disconnected) {
                        tracing::info!("device disconnected");
                        return Message::Disconnect;
                    }
                    if connection_status.changed().await.is_err() {
                        // sender is dropped, which means device was dropped, which means DeviceSettingsModel was dropped
                        // in that case, bail
                        tracing::debug!("connection status sender dropped, bailing");
                        return Message::None;
                    }
                }
            }
            .instrument(tracing::info_span!("watch_for_disconnect_task")),
        );

        let mut model = Self {
            throttle: throttle::Throttle::new(device.0.clone()),
            device,
            nav_model,
            settings_order: Vec::new(),
            settings: HashMap::new(),
            dialog: None,
            legacy_equalizer_migration: None,
            quick_presets_model,
            key_binds: key_binds(),
        };
        let task = Task::batch([
            model.refresh(),
            quick_presets_refresh_task.map(Into::into),
            Self::initialize_legacy_migration(config_dir),
            Task::stream(stream),
            watch_for_disconnect_task,
        ]);
        (model, task)
    }

    fn initialize_legacy_migration(config_dir: PathBuf) -> Task<Message> {
        Task::future(async move {
            let profiles = match openscq30_v1_migration::all_equalizer_profiles(config_dir).await {
                Ok(profiles) => profiles,
                Err(err) => match err {
                    openscq30_v1_migration::FetchProfilesError::NoLegacyConfig => {
                        return Message::None;
                    }
                    _ => {
                        tracing::error!("error loading legacy config file: {err:?}");
                        return Message::None;
                    }
                },
            };
            Message::AddLegacyEqualizerMigrationPage(profiles)
        })
    }

    pub fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Message> {
        self.nav_model.activate(id);
        self.refresh()
    }

    fn refresh(&mut self) -> Task<Message> {
        Task::batch([self.refresh_settings()])
    }

    fn refresh_settings(&mut self) -> Task<Message> {
        if let Some(category_id) = self.nav_model.active_data::<CategoryId>() {
            let mut settings_order = self.device.settings_in_category(category_id);
            let settings = settings_order
                .iter()
                .copied()
                .flat_map(|setting_id| {
                    self.throttle
                        .setting(&setting_id)
                        .map(|value| (setting_id, value))
                })
                .collect::<HashMap<SettingId, Setting>>();

            // remove setting ids that are currently unavailable from the ordering
            settings_order.retain(|setting_id| settings.contains_key(setting_id));
            self.settings_order = settings_order;

            // remove setting ui state for settings that no longer exist
            self.settings
                .retain(|setting_id, _| settings.contains_key(setting_id));

            // update existing setting ui state, or add new state for newly introduced settings
            for (setting_id, setting) in settings.into_iter() {
                match self.settings.entry(setting_id) {
                    hash_map::Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().variant_state.update(setting)
                    }
                    hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(SettingUiState {
                            setting_id,
                            translated_name: setting_id.translate(),
                            variant_state: setting.into(),
                        });
                    }
                }
            }
        }
        Task::none()
    }

    pub fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    pub fn dialog(&self) -> Option<Element<'_, Message>> {
        self.quick_presets_model
            .dialog()
            .map(|dialog| dialog.map(Into::into))
            .or_else(|| {
                self.dialog.as_ref().map(|dialog| match dialog {
                    Dialog::ModifiableSelectAdd(setting_id, name) => widget::dialog()
                        .title({
                            let setting_name = setting_id.translate();
                            fl!("add-item", name = setting_name.as_str())
                        })
                        .control(
                            widget::text_input(fl!("name"), name)
                                .id(widget::Id::new(
                                    "optional-select-dialog-add-item-text-input",
                                ))
                                .on_input(Message::ModifiableSelectAddDialogSetName)
                                .on_submit(|name| {
                                    Message::ModifiableSelectAddDialogSubmit(Some(name))
                                }),
                        )
                        .primary_action(
                            widget::button::suggested(fl!("create"))
                                .on_press(Message::ModifiableSelectAddDialogSubmit(None)),
                        )
                        .secondary_action(
                            widget::button::destructive(fl!("cancel"))
                                .on_press(Message::CancelDialog),
                        )
                        .into(),
                    Dialog::ModifiableSelectRemove(_setting_id, name) => widget::dialog()
                        .title(fl!("remove-item", name = name.as_ref()))
                        .body(fl!("remove-item-confirm", name = name.as_ref()))
                        .primary_action(
                            widget::button::destructive(fl!("remove"))
                                .on_press(Message::ModifiableSelectRemoveDialogSubmit),
                        )
                        .secondary_action(
                            widget::button::text(fl!("cancel")).on_press(Message::CancelDialog),
                        )
                        .into(),
                    Dialog::ImportStringConfirm(setting_id, _text) => widget::dialog()
                        .title(setting_id.translate())
                        .body(
                            if let Some(SettingVariantUiState::ImportString {
                                confirmation_message: Some(confirmation_message),
                                text: _,
                            }) = self
                                .settings
                                .get(setting_id)
                                .map(|setting| &setting.variant_state)
                            {
                                confirmation_message.as_str()
                            } else {
                                ""
                            },
                        )
                        .primary_action(
                            widget::button::suggested(fl!("confirm"))
                                .on_press(Message::ConfirmImportString),
                        )
                        .secondary_action(
                            widget::button::text(fl!("cancel")).on_press(Message::CancelDialog),
                        )
                        .into(),
                })
            })
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(custom_category) = self.nav_model.active_data::<CustomCategory>() {
            match custom_category {
                CustomCategory::QuickPresets => self.quick_presets_model.view().map(Into::into),
                CustomCategory::LegacyEqualizerMigration => {
                    if let Some(model) = &self.legacy_equalizer_migration {
                        model.view().map(Message::LegacyMigration)
                    } else {
                        widget::text("unreachable").into()
                    }
                }
            }
        } else if let Some(category_id) = self.nav_model.active_data::<CategoryId>() {
            self.view_settings(category_id)
        } else {
            widget::space().into()
        }
    }

    fn view_settings<'a>(&'a self, category_id: &'a CategoryId) -> Element<'a, Message> {
        let mut section = widget::settings::section().title(category_id.translate());
        for setting_id in &self.settings_order {
            let Some(setting_ui_state) = self.settings.get(setting_id) else {
                tracing::error!("setting id {setting_id} is missing ui state");
                continue;
            };
            match self.view_setting(setting_ui_state) {
                SettingDisplayKind::Single(element) => {
                    section = section.add(element);
                }
                SettingDisplayKind::Vec(elements) => {
                    section = section.extend(elements);
                }
                SettingDisplayKind::ListButton(list_button) => {
                    section = section.add(list_button);
                }
                SettingDisplayKind::ListButtonVec(list_buttons) => {
                    section = section.extend(list_buttons);
                }
            }
        }

        widget::scrollable(section).into()
    }

    fn view_setting<'a>(&'a self, setting: &'a SettingUiState) -> SettingDisplayKind<'a, Message> {
        let setting_id = setting.setting_id;
        let translated_name = Cow::Borrowed(setting.translated_name.as_str());
        match &setting.variant_state {
            SettingVariantUiState::Toggle { value } => {
                controls::toggle(translated_name, *value, move |new_value| {
                    Message::SetSetting(setting_id, new_value.into())
                })
                .into()
            }
            SettingVariantUiState::I32Range { setting, value } => {
                controls::i32_range(translated_name, setting.clone(), *value, move |new_value| {
                    Message::SetSetting(setting_id, new_value.into())
                })
                .into()
            }
            SettingVariantUiState::Select { setting, value } => {
                controls::select(translated_name, setting, value, move |value| {
                    Message::SetSetting(setting_id, Cow::from(value.to_owned()).into())
                })
                .into()
            }
            SettingVariantUiState::OptionalSelect { setting, value }
            | SettingVariantUiState::PresetEqualizerProfileSelect {
                select: setting,
                value,
                ..
            } => controls::optional_select(
                translated_name,
                setting,
                value.as_deref(),
                move |value| {
                    Message::SetSetting(
                        setting_id,
                        value.map(ToOwned::to_owned).map(Cow::from).into(),
                    )
                },
            )
            .into(),
            SettingVariantUiState::ModifiableSelect { setting, value } => {
                controls::modifiable_select(
                    translated_name,
                    setting,
                    value.as_deref(),
                    move |value| {
                        Message::SetSetting(setting_id, Cow::from(value.to_owned()).into())
                    },
                    Message::ShowModifiableSelectAddDialog(setting_id),
                    Message::ShowModifiableSelectRemoveDialog(setting_id),
                )
                .into()
            }
            SettingVariantUiState::MultiSelect { setting, values } => {
                controls::multi_select(translated_name, setting, values, move |values| {
                    Message::SetSetting(setting_id, values.into())
                })
                .into()
            }
            SettingVariantUiState::MultiSelectWithRemove { setting, values } => {
                controls::multi_select_with_remove(
                    translated_name,
                    setting,
                    values,
                    move |values| Message::SetSetting(setting_id, values.into()),
                    |option| {
                        Message::SetSetting(
                            setting_id,
                            Value::MultiSelectWithRemoveCommand(
                                settings::MultiSelectWithRemoveCommand::Remove(option),
                            ),
                        )
                    },
                )
                .into()
            }
            SettingVariantUiState::Equalizer {
                setting,
                read_only,
                value,
            } => {
                if !read_only {
                    controls::horizontal_equalizer(setting, value, move |index, value| {
                        Message::SetEqualizerBand(setting_id, index, value)
                    })
                    .into()
                } else {
                    // TODO reuse EqualizerLine so that the canvas cache is in effect
                    Element::from(
                        widget::canvas(equalizer_line::EqualizerLine::new(
                            setting.min,
                            setting.max,
                            value.to_vec(),
                        ))
                        .width(Length::Fill),
                    )
                    .into()
                }
            }
            SettingVariantUiState::Information { translated_value } => controls::information(
                translated_name,
                Cow::Borrowed(translated_value),
                Message::CopyToClipboard(translated_value.to_owned()),
            )
            .into(),
            SettingVariantUiState::ImportString {
                text,
                confirmation_message: _,
            } => controls::import_string(
                translated_name,
                Cow::Borrowed(text),
                move |text| Message::SetImportString(setting_id, text),
                move |text| Message::AskConfirmImportString(setting_id, Cow::from(text).into()),
            )
            .into(),
            SettingVariantUiState::HueColorPicker { hue } => {
                controls::hue_color_picker(translated_name, *hue, move |new_hue| {
                    Message::SetSetting(setting_id, new_hue.into())
                })
                .into()
            }
            SettingVariantUiState::Action => controls::action(
                translated_name,
                Message::SetSetting(setting_id, true.into()),
            )
            .into(),
            SettingVariantUiState::TimeOfDay {
                minutes_after_midnight,
                hour_text,
                minute_text,
            } => controls::time_of_day(
                translated_name,
                Some(minutes_after_midnight.is_pm()),
                hour_text.as_ref().map_or_else(
                    || Cow::Owned(minutes_after_midnight.twelve_hour().to_string()),
                    |text| Cow::Borrowed(text.as_str()),
                ),
                minute_text.as_ref().map_or_else(
                    || Cow::Owned(format!("{:02}", minutes_after_midnight.minute())),
                    |text| Cow::Borrowed(text.as_str()),
                ),
                move |hour| Message::SetTimeTwelveHour(setting_id, hour),
                move |minute| Message::SetTimeMinute(setting_id, minute),
                move |is_pm| Message::SetTimePm(setting_id, is_pm),
                Message::RefreshTimeTwelveHour(setting_id),
            )
            .into(),
        }
    }

    pub fn context_drawer(&self) -> Option<ContextDrawer<'_, Message>> {
        if matches!(
            self.nav_model.active_data(),
            Some(CustomCategory::QuickPresets)
        ) {
            self.quick_presets_model
                .context_drawer()
                .map(|context_drawer| context_drawer.map(Into::into))
        } else {
            None
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::QuickPresets(inner) => match self.quick_presets_model.update(inner) {
                quick_presets::Action::Warning(text) => Action::Warning(text),
                quick_presets::Action::None => Action::None,
                quick_presets::Action::Task(task) => Action::Task(task.map(Into::into)),
                quick_presets::Action::FocusTextInput(id) => Action::FocusTextInput(id),
            },
            Message::SetSetting(setting_id, value) => self.set_setting(setting_id, value),
            Message::SetTimeTwelveHour(setting_id, new_hour_text) => {
                let maybe_value = if new_hour_text.len() <= 2
                    && let Some(setting) = self.settings.get_mut(&setting_id)
                    && let SettingVariantUiState::TimeOfDay {
                        minutes_after_midnight,
                        hour_text,
                        minute_text,
                    } = &mut setting.variant_state
                {
                    *minute_text = None;
                    if let Ok(hour) = new_hour_text.parse() {
                        *minutes_after_midnight = minutes_after_midnight.with_twelve_hour(hour);
                        // If hour is out of range and gets clamped, update the text to match
                        *hour_text =
                            (hour == minutes_after_midnight.twelve_hour()).then_some(new_hour_text);
                        Some(minutes_after_midnight.0.into())
                    } else if new_hour_text.is_empty() {
                        *hour_text = Some(new_hour_text);
                        None
                    } else {
                        *hour_text = None;
                        None
                    }
                } else {
                    None
                };
                if let Some(value) = maybe_value {
                    self.set_setting(setting_id, value)
                } else {
                    Action::None
                }
            }
            Message::SetTimeMinute(setting_id, new_minute_text) => {
                let maybe_value = if new_minute_text.len() <= 2
                    && let Some(setting) = self.settings.get_mut(&setting_id)
                    && let SettingVariantUiState::TimeOfDay {
                        minutes_after_midnight,
                        hour_text,
                        minute_text,
                    } = &mut setting.variant_state
                {
                    *hour_text = None;
                    if let Ok(minute) = new_minute_text.parse() {
                        *minutes_after_midnight = minutes_after_midnight.with_minute(minute);
                        // If minute is out of range and gets clamped, update the text to match
                        *minute_text =
                            (minute == minutes_after_midnight.minute()).then_some(new_minute_text);
                        Some(minutes_after_midnight.0.into())
                    } else if new_minute_text.is_empty() {
                        *minute_text = Some(new_minute_text);
                        None
                    } else {
                        *minute_text = None;
                        None
                    }
                } else {
                    None
                };
                // outside of the if statement to avoid double mutable borrow
                if let Some(value) = maybe_value {
                    self.set_setting(setting_id, value)
                } else {
                    Action::None
                }
            }
            Message::SetTimePm(setting_id, is_pm) => {
                let maybe_value = if let Some(setting) = self.settings.get_mut(&setting_id)
                    && let SettingVariantUiState::TimeOfDay {
                        minutes_after_midnight,
                        hour_text,
                        minute_text,
                    } = &mut setting.variant_state
                {
                    *minutes_after_midnight = minutes_after_midnight.with_pm(is_pm);
                    *hour_text = None;
                    *minute_text = None;
                    Some(minutes_after_midnight.0.into())
                } else {
                    None
                };

                if let Some(value) = maybe_value {
                    self.set_setting(setting_id, value)
                } else {
                    Action::None
                }
            }
            Message::RefreshTimeTwelveHour(setting_id) => {
                if let Some(setting) = self.settings.get_mut(&setting_id)
                    && let SettingVariantUiState::TimeOfDay {
                        hour_text,
                        minute_text,
                        ..
                    } = &mut setting.variant_state
                {
                    *hour_text = None;
                    *minute_text = None;
                }
                Action::None
            }
            Message::SetEqualizerBand(setting_id, index, new_value) => {
                if let Some(Setting::Equalizer {
                    setting: _,
                    read_only: _,
                    value: values,
                }) = self.throttle.setting(&setting_id)
                {
                    let mut new_values = values.clone();
                    new_values[index as usize] = new_value;
                    let maybe_task = self.throttle.set_setting(setting_id, new_values.into());
                    _ = self.refresh_settings();
                    maybe_task.map_or(Action::None, |task| Action::Task(task.map(Into::into)))
                } else {
                    Action::None
                }
            }
            Message::RefreshSettings => Action::Task(self.refresh_settings()),
            Message::Warning(message) => Action::Warning(message),
            Message::CancelDialog => {
                self.dialog = None;
                Action::None
            }
            Message::ShowModifiableSelectAddDialog(setting_id) => {
                self.dialog = Some(Dialog::ModifiableSelectAdd(setting_id, String::new()));
                Action::FocusTextInput(widget::Id::new(
                    "modifiable-select-dialog-add-item-text-input",
                ))
            }
            Message::ShowModifiableSelectRemoveDialog(setting_id) => {
                let selected_item = self.settings.get(&setting_id).and_then(|item| {
                    if let SettingVariantUiState::ModifiableSelect { setting: _, value } =
                        &item.variant_state
                    {
                        value.to_owned()
                    } else {
                        None
                    }
                });
                if let Some(selected_item) = selected_item {
                    self.dialog = Some(Dialog::ModifiableSelectRemove(setting_id, selected_item));
                } else {
                    tracing::error!(
                        r#"tried to open modifiable select remove dialog for {setting_id:?}, but selected item is None.
                        current settings: {:?}
                        "#,
                        self.settings,
                    );
                }
                Action::None
            }
            Message::ModifiableSelectAddDialogSetName(new_name) => {
                if let Some(Dialog::ModifiableSelectAdd(_setting_id, name)) = &mut self.dialog {
                    *name = new_name;
                }
                Action::None
            }
            Message::ModifiableSelectAddDialogSubmit(override_name) => {
                if let Some(Dialog::ModifiableSelectAdd(setting_id, name)) = self.dialog.take() {
                    let name = override_name.unwrap_or(name);

                    let device = self.device.clone();
                    Action::Task(
                        Task::future(async move {
                            device
                                .set_setting_values(vec![(
                                    setting_id,
                                    Value::ModifiableSelectCommand(
                                        settings::ModifiableSelectCommand::Add(name.into()),
                                    ),
                                )])
                                .await
                                .map_err(handle_soft_error!())?;
                            Ok(Message::RefreshSettings)
                        })
                        .map(coalesce_result),
                    )
                } else {
                    Action::None
                }
            }
            Message::ModifiableSelectRemoveDialogSubmit => {
                if let Some(Dialog::ModifiableSelectRemove(setting_id, name)) = self.dialog.take() {
                    let device = self.device.clone();
                    Action::Task(
                        Task::future(async move {
                            device
                                .set_setting_values(vec![(
                                    setting_id,
                                    Value::ModifiableSelectCommand(
                                        settings::ModifiableSelectCommand::Remove(name),
                                    ),
                                )])
                                .await
                                .map_err(handle_soft_error!())?;
                            Ok(Message::RefreshSettings)
                        })
                        .map(coalesce_result),
                    )
                } else {
                    Action::None
                }
            }
            Message::AddLegacyEqualizerMigrationPage(profiles) => {
                self.legacy_equalizer_migration = Some(LegacyMigrationModel::new(profiles));
                self.nav_model
                    .insert()
                    .text(fl!("legacy-equalizer-profile-migration"))
                    .data(CustomCategory::LegacyEqualizerMigration);

                Action::None
            }
            Message::None => Action::None,
            Message::LegacyMigration(message) => match message {
                legacy_migration::Message::Migrate(name, volume_adjustments) => {
                    let device = self.device.to_owned();
                    Action::Task(
                        Task::<anyhow::Result<Message>>::future(async move {
                            openscq30_v1_migration::migrate_legacy_profile(
                                device.as_ref(),
                                name,
                                volume_adjustments,
                            )
                            .await?;
                            Ok(Message::RefreshSettings)
                        })
                        .map(|r| r.map_err(handle_soft_error!()))
                        .map(coalesce_result),
                    )
                }
            },
            Message::CopyToClipboard(text) => Action::Task(cosmic::iced::clipboard::write(text)),
            Message::SetImportString(setting_id, new_text) => {
                if let Some(setting) = self.settings.get_mut(&setting_id)
                    && let SettingVariantUiState::ImportString { text, .. } =
                        &mut setting.variant_state
                {
                    *text = new_text;
                }
                Action::None
            }
            Message::AskConfirmImportString(setting_id, import_text) => {
                self.dialog = Some(Dialog::ImportStringConfirm(setting_id, import_text));
                Action::None
            }
            Message::ConfirmImportString => {
                if let Some(Dialog::ImportStringConfirm(setting_id, text)) = self.dialog.take() {
                    if let Some(setting) = self.settings.get_mut(&setting_id)
                        && let SettingVariantUiState::ImportString { text, .. } =
                            &mut setting.variant_state
                    {
                        *text = String::new();
                    }
                    let device = self.device.clone();
                    Action::Task(
                        Task::future(async move {
                            device
                                .set_setting_values(vec![(setting_id, Cow::from(text).into())])
                                .await
                                .map_err(handle_soft_error!())?;
                            Ok(Message::RefreshSettings)
                        })
                        .map(coalesce_result),
                    )
                } else {
                    Action::None
                }
            }
            Message::Throttle(message) => match self.throttle.update(message) {
                throttle::Action::Task(task) => Action::Task(task.map(Into::into)),
                throttle::Action::Error(err) => Action::Task(Task::done(handle_soft_error!()(err))),
                throttle::Action::None => Action::None,
            },
            Message::Disconnect => Action::Disconnect,
        }
    }

    fn set_setting(&mut self, setting_id: SettingId, value: Value) -> Action {
        let device = self.device.clone();
        let should_throttle = matches!(
            device.setting(&setting_id),
            Some(
                // throttled because it's implemented as a slider, which would update every tick
                Setting::I32Range { .. }
                    // throttled because it's implemented as a slider, which would update every tick
                    | Setting::HueColorPicker { .. }
                    // throttled so that if the user types both digits of an hour or minute
                    // quickly, we only update once
                    | Setting::TimeOfDay { .. }
            ),
        );
        if should_throttle {
            let maybe_task = self.throttle.set_setting(setting_id, value);
            _ = self.refresh_settings();
            maybe_task.map_or(Action::None, |task| Action::Task(task.map(Into::into)))
        } else {
            Action::Task(
                Task::future(async move {
                    device
                        .set_setting_values(vec![(setting_id, value)])
                        .await
                        .map_err(handle_soft_error!())?;
                    Ok(Message::RefreshSettings)
                })
                .map(coalesce_result),
            )
        }
    }

    pub fn on_key_pressed(
        &mut self,
        modifiers: keyboard::Modifiers,
        key: &keyboard::Key,
        physical_key: &keyboard::key::Physical,
    ) -> Action {
        let action = self
            .key_binds
            .iter()
            .find(|(bind, _)| bind.matches(modifiers, key, Some(physical_key)))
            .map(|(_, action)| action)
            .copied();
        if let Some(action) = action {
            self.handle_key_bind_action(action)
        } else {
            Action::None
        }
    }

    fn handle_key_bind_action(&mut self, action: KeyBindAction) -> Action {
        match action {
            KeyBindAction::CloseDialog => {
                self.dialog = None;
                self.quick_presets_model.close_dialog();
                Action::None
            }
        }
    }
}

fn labeled_setting_row<'a, M>(
    label: impl Into<Cow<'a, str>> + 'a,
    element: impl Into<Element<'a, M>>,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    widget::settings::item::builder(label)
        .flex_control(element)
        .into()
}

#[derive(Clone, Copy)]
enum KeyBindAction {
    CloseDialog,
}

fn key_binds() -> HashMap<KeyBind, KeyBindAction> {
    let mut key_binds = HashMap::new();

    key_binds.insert(
        KeyBind {
            modifiers: Vec::new(),
            key: keyboard::Key::Named(keyboard::key::Named::Escape),
        },
        KeyBindAction::CloseDialog,
    );

    key_binds
}
