mod action;
mod equalizer;
mod hue_color_picker;
mod import_string;
mod information;
mod legacy_migration;
mod quick_presets;
mod range;
mod select;
mod time_of_day;
mod toggle;

use std::{borrow::Cow, collections::HashMap, path::PathBuf};

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
    settings: Vec<SettingUiState>,
    dialog: Option<Dialog>,
    legacy_equalizer_migration: Option<legacy_migration::LegacyMigrationModel>,
    quick_presets_model: quick_presets::QuickPresetsModel,
    throttle: throttle::Throttle,
    key_binds: HashMap<KeyBind, KeyBindAction>,
}

#[derive(Debug)]
struct SettingUiState {
    setting_id: SettingId,
    localized_name: String,
    setting_kind_state: SettingKindUiState,
}

#[derive(Debug)]
enum SettingKindUiState {
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
        equalizer: settings::Equalizer,
        select: settings::Select,
        /// Each index corresponds to the same index in select.options
        presets: Vec<Vec<i16>>,
        value: Option<Cow<'static, str>>,
    },
    Information {
        value: String,
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
        minutes_after_midnight: i32,
        hour_text: String,
        minute_text: String,
    },
}

impl From<Setting> for SettingKindUiState {
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
                equalizer,
                select,
                presets,
                value,
            } => Self::PresetEqualizerProfileSelect {
                equalizer,
                select,
                presets,
                value,
            },
            Setting::Information {
                value,
                translated_value,
            } => Self::Information {
                value,
                translated_value,
            },
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
                minutes_after_midnight,
                hour_text: String::new(),
                minute_text: String::new(),
            },
        }
    }
}

impl SettingKindUiState {
    fn update(&mut self, setting: Setting) {
        match (self, setting) {
            (
                Self::TimeOfDay {
                    minutes_after_midnight,
                    hour_text: _,
                    minute_text: _,
                },
                Setting::TimeOfDay {
                    minutes_after_midnight: new_minutes_after_midnight,
                },
            ) => *minutes_after_midnight = new_minutes_after_midnight,
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
            settings: Vec::new(),
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
            self.settings = self
                .device
                .settings_in_category(category_id)
                .into_iter()
                .flat_map(|setting_id| {
                    self.throttle
                        .setting(&setting_id)
                        .map(|value| (setting_id, value))
                })
                .map(|(setting_id, setting)| SettingUiState {
                    setting_id: setting_id,
                    localized_name: setting_id.translate(),
                    setting_kind_state: setting.into(),
                })
                .collect();
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
                            if let Some(SettingKindUiState::ImportString {
                                confirmation_message: Some(confirmation_message),
                                text: _,
                            }) = self
                                .settings
                                .iter()
                                .find(|setting| *setting_id == setting.setting_id)
                                .map(|setting| &setting.setting_kind_state)
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
        for setting in &self.settings {
            match self.view_setting(setting) {
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
        let translated_name = Cow::Borrowed(setting.localized_name.as_str());
        match &setting.setting_kind_state {
            SettingKindUiState::Toggle { value } => {
                toggle::toggle(translated_name, *value, move |new_value| {
                    Message::SetSetting(setting_id, new_value.into())
                })
                .into()
            }
            SettingKindUiState::I32Range { setting, value } => {
                range::i32_range(translated_name, setting.clone(), *value, move |new_value| {
                    Message::SetSetting(setting_id, new_value.into())
                })
                .into()
            }
            SettingKindUiState::Select { setting, value } => {
                select::select(translated_name, setting, value, move |value| {
                    Message::SetSetting(setting_id, Cow::from(value.to_owned()).into())
                })
                .into()
            }
            SettingKindUiState::OptionalSelect { setting, value }
            | SettingKindUiState::PresetEqualizerProfileSelect {
                select: setting,
                value,
                ..
            } => {
                select::optional_select(translated_name, setting, value.as_deref(), move |value| {
                    Message::SetSetting(
                        setting_id,
                        value.map(ToOwned::to_owned).map(Cow::from).into(),
                    )
                })
                .into()
            }
            SettingKindUiState::ModifiableSelect { setting, value } => select::modifiable_select(
                translated_name,
                setting,
                value.as_deref(),
                move |value| Message::SetSetting(setting_id, Cow::from(value.to_owned()).into()),
                Message::ShowModifiableSelectAddDialog(setting_id),
                Message::ShowModifiableSelectRemoveDialog(setting_id),
            )
            .into(),
            SettingKindUiState::MultiSelect { setting, values } => {
                select::multi_select(translated_name, setting, values, move |values| {
                    Message::SetSetting(setting_id, values.into())
                })
                .into()
            }
            SettingKindUiState::MultiSelectWithRemove { setting, values } => {
                select::multi_select_with_remove(
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
            SettingKindUiState::Equalizer {
                setting,
                read_only,
                value,
            } => {
                if !read_only {
                    equalizer::horizontal_equalizer(setting, value, move |index, value| {
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
            SettingKindUiState::Information {
                value: _,
                translated_value,
            } => information::information(
                translated_name,
                Cow::Borrowed(translated_value),
                Message::CopyToClipboard(translated_value.to_owned()),
            )
            .into(),
            SettingKindUiState::ImportString {
                text,
                confirmation_message: _,
            } => import_string::input(
                translated_name,
                Cow::Borrowed(text),
                move |text| Message::SetImportString(setting_id, text),
                move |text| Message::AskConfirmImportString(setting_id, Cow::from(text).into()),
            )
            .into(),
            SettingKindUiState::HueColorPicker { hue } => {
                hue_color_picker::hue_color_picker(translated_name, *hue, move |new_hue| {
                    Message::SetSetting(setting_id, new_hue.into())
                })
                .into()
            }
            SettingKindUiState::Action => action::action(
                translated_name,
                Message::SetSetting(setting_id, true.into()),
            )
            .into(),
            SettingKindUiState::TimeOfDay {
                minutes_after_midnight,
                hour_text,
                minute_text,
            } => time_of_day::time(
                translated_name,
                *minutes_after_midnight,
                move |new_minutes_after_midnight| {
                    Message::SetSetting(setting_id, new_minutes_after_midnight.into())
                },
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
            Message::SetSetting(setting_id, value) => {
                let device = self.device.clone();
                let should_throttle = matches!(
                    device.setting(&setting_id),
                    Some(Setting::I32Range { .. } | Setting::HueColorPicker { .. }),
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
                let selected_item = self
                    .settings
                    .iter()
                    .find(|item| item.setting_id == setting_id)
                    .and_then(|item| {
                        if let SettingKindUiState::ModifiableSelect { setting: _, value } =
                            &item.setting_kind_state
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
                if let Some(setting) = self
                    .settings
                    .iter_mut()
                    .find(|setting| setting.setting_id == setting_id)
                    && let SettingKindUiState::ImportString { text, .. } =
                        &mut setting.setting_kind_state
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
                    if let Some(setting) = self
                        .settings
                        .iter_mut()
                        .find(|setting| setting.setting_id == setting_id)
                        && let SettingKindUiState::ImportString { text, .. } =
                            &mut setting.setting_kind_state
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

    #[must_use]
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
