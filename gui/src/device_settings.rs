mod action;
mod equalizer;
mod import_string;
mod information;
mod legacy_migration;
mod quick_presets;
mod range;
mod select;
mod toggle;

use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use cosmic::{
    Element, Task,
    app::context_drawer::ContextDrawer,
    widget::{self, nav_bar},
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
    fl, handle_soft_error,
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
    settings: Vec<(SettingId, Setting)>,
    dialog: Option<Dialog>,
    legacy_equalizer_migration: Option<legacy_migration::LegacyMigrationModel>,
    import_strings: HashMap<SettingId, String>,
    quick_presets_model: quick_presets::QuickPresetsModel,
    throttle: throttle::Throttle,
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
        let stream = cosmic::iced::stream::channel(1, |mut output| async move {
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
            import_strings: HashMap::new(),
            quick_presets_model,
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
                            if let Some((
                                _,
                                Setting::ImportString {
                                    confirmation_message: Some(confirmation_message),
                                },
                            )) = self.settings.iter().find(|(id, _)| id == setting_id)
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
            widget::row().into()
        }
    }

    fn view_settings<'a>(&'a self, category_id: &'a CategoryId) -> Element<'a, Message> {
        widget::scrollable(
            widget::settings::section()
                .title(category_id.translate())
                .extend(
                    self.settings
                        .iter()
                        .flat_map(|(setting_id, setting)| self.view_setting(*setting_id, setting)),
                ),
        )
        .into()
    }

    fn view_setting<'a>(
        &'a self,
        setting_id: SettingId,
        setting: &'a Setting,
    ) -> Vec<Element<'a, Message>> {
        match setting {
            Setting::Toggle { value } => {
                vec![toggle::toggle(setting_id, *value, move |new_value| {
                    Message::SetSetting(setting_id, new_value.into())
                })]
            }
            Setting::I32Range { setting, value } => vec![range::i32_range(
                setting_id,
                setting.range.clone(),
                *value,
                move |new_value| Message::SetSetting(setting_id, new_value.into()),
            )],
            Setting::Select { setting, value } => {
                vec![select::select(setting_id, setting, value, move |value| {
                    Message::SetSetting(setting_id, Cow::from(value.to_owned()).into())
                })]
            }
            Setting::OptionalSelect { setting, value } => {
                vec![select::optional_select(
                    setting_id,
                    setting,
                    value.as_deref(),
                    move |value| {
                        Message::SetSetting(
                            setting_id,
                            value.map(ToOwned::to_owned).map(Cow::from).into(),
                        )
                    },
                )]
            }
            Setting::ModifiableSelect { setting, value } => vec![select::modifiable_select(
                setting_id,
                setting,
                value.as_deref(),
                move |value| Message::SetSetting(setting_id, Cow::from(value.to_owned()).into()),
                Message::ShowModifiableSelectAddDialog(setting_id),
                Message::ShowModifiableSelectRemoveDialog(setting_id),
            )],
            Setting::MultiSelect { setting, values } => {
                select::multi_select(setting_id, setting, values, move |values| {
                    Message::SetSetting(setting_id, values.into())
                })
            }
            Setting::Equalizer { setting, value } => {
                equalizer::horizontal_equalizer(setting, value, move |index, value| {
                    Message::SetEqualizerBand(setting_id, index, value)
                })
            }
            Setting::Information {
                value: _,
                translated_value: translated_text,
            } => vec![information::information(
                setting_id,
                Cow::Borrowed(translated_text),
                Message::CopyToClipboard(translated_text.to_owned()),
            )],
            Setting::ImportString {
                confirmation_message: _,
            } => vec![import_string::input(
                setting_id,
                self.import_strings
                    .get(&setting_id)
                    .map(String::as_str)
                    .map_or_else(|| Cow::Borrowed(""), Cow::Borrowed),
                move |text| Message::SetImportString(setting_id, text),
                move |text| Message::AskConfirmImportString(setting_id, Cow::from(text).into()),
            )],
            Setting::Action => vec![action::action(
                setting_id,
                Message::SetSetting(setting_id, true.into()),
            )],
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
                let should_throttle =
                    matches!(device.setting(&setting_id), Some(Setting::I32Range { .. }));
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
                    .find(|item| item.0 == setting_id)
                    .and_then(|item| {
                        if let (_setting_id, Setting::ModifiableSelect { setting: _, value }) = item
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
            Message::SetImportString(setting_id, text) => {
                self.import_strings.insert(setting_id, text);
                Action::None
            }
            Message::AskConfirmImportString(setting_id, import_text) => {
                self.dialog = Some(Dialog::ImportStringConfirm(setting_id, import_text));
                Action::None
            }
            Message::ConfirmImportString => {
                if let Some(Dialog::ImportStringConfirm(setting_id, text)) = self.dialog.take() {
                    self.import_strings.remove(&setting_id);
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
