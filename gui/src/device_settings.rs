mod equalizer;
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
    iced::{Length, alignment},
    widget::{self, nav_bar},
};
use legacy_migration::LegacyMigrationModel;
use openscq30_i18n::Translate;
use openscq30_lib::{
    api::{
        quick_presets::QuickPresetsHandler,
        settings::{self, CategoryId, Setting, SettingId, Value},
    },
    storage::QuickPreset,
};
use tracing::debug;

use crate::{
    app::DebugOpenSCQ30Device,
    fl, handle_soft_error,
    openscq30_v1_migration::{self, LegacyEqualizerProfile},
    utils::coalesce_result,
};

const MIN_SETTING_ROW_HEIGHT: Length = Length::Fixed(35f32);

#[derive(Debug, Clone)]
pub enum Message {
    SetSetting(SettingId, Value),
    SetEqualizerBand(SettingId, u8, i16),
    RefreshQuickPresets,
    RefreshSettings,
    Warning(String),
    ShowCreateQuickPresetDialog,
    ActivateQuickPreset(usize),
    EditQuickPreset(usize),
    SetQuickPresets(Vec<QuickPreset>),
    CreateQuickPreset(Option<String>),
    SnapshotQuickPresetSettings(String),
    SetCreateQuickPresetName(String),
    CancelDialog,
    EditQuickPresetToggleField(usize, bool),
    EditQuickPresetClose,
    EditQuickPresetModified,
    ShowModifiableSelectAddDialog(SettingId),
    ShowModifiableSelectRemoveDialog(SettingId),
    ModifiableSelectAddDialogSubmit(Option<String>),
    ModifiableSelectAddDialogSetName(String),
    ModifiableSelectRemoveDialogSubmit,
    AddLegacyEqualizerMigrationPage(HashMap<String, LegacyEqualizerProfile>),
    LegacyMigration(legacy_migration::Message),
    None,
    ShowDeleteQuickPresetDialog(usize),
    DeleteQuickPreset(String),
}
pub enum Action {
    Task(Task<Message>),
    Warning(String),
    FocusTextInput(widget::Id),
    None,
}

pub struct DeviceSettingsModel {
    device: DebugOpenSCQ30Device,
    quick_presets_handler: QuickPresetsHandler,
    nav_model: nav_bar::Model,
    settings: Vec<(SettingId, Setting)>,
    quick_presets: Option<Vec<QuickPreset>>,
    dialog: Option<Dialog>,
    editing_quick_preset: Option<QuickPreset>,
    legacy_equalizer_migration: Option<legacy_migration::LegacyMigrationModel>,
}

enum Dialog {
    CreateQuickPreset(String),
    ModifiableSelectAdd(SettingId, String),
    ModifiableSelectRemove(SettingId, Cow<'static, str>),
    DeleteQuickPreset(String),
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
            nav_model
                .insert()
                .text(category.translate())
                .data(category.clone());
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

        let mut model = Self {
            device,
            nav_model,
            settings: Vec::new(),
            quick_presets_handler,
            quick_presets: None,
            editing_quick_preset: None,
            dialog: None,
            legacy_equalizer_migration: None,
        };
        let task = Task::batch([
            model.refresh(),
            Self::initialize_legacy_migration(config_dir),
            Task::stream(stream),
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
        Task::batch([self.refresh_settings(), self.refresh_quick_presets()])
    }

    fn refresh_settings(&mut self) -> Task<Message> {
        if let Some(category_id) = self.nav_model.active_data::<CategoryId>() {
            self.settings = self
                .device
                .settings_in_category(category_id)
                .into_iter()
                .flat_map(|setting_id| {
                    self.device
                        .setting(&setting_id)
                        .map(|value| (setting_id, value))
                })
                .collect();
        }
        Task::none()
    }

    fn refresh_quick_presets(&mut self) -> Task<Message> {
        if let Some(CustomCategory::QuickPresets) = self.nav_model.active_data() {
            let device = self.device.clone();
            let quick_presets_handler = self.quick_presets_handler.clone();
            Task::future(async move {
                quick_presets_handler
                    .quick_presets(device.as_ref())
                    .await
                    .map(Message::SetQuickPresets)
                    .map_err(handle_soft_error!())
            })
            .map(coalesce_result)
        } else {
            Task::none()
        }
    }

    pub fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    pub fn dialog(&self) -> Option<Element<'_, Message>> {
        self.dialog.as_ref().map(|dialog| match dialog {
            Dialog::CreateQuickPreset(name) => widget::dialog()
                .title(fl!("create-quick-preset"))
                .control(
                    widget::text_input(fl!("name"), name)
                        .id(widget::Id::new("create-quick-preset-name"))
                        .on_input(Message::SetCreateQuickPresetName)
                        .on_submit(|name| Message::CreateQuickPreset(Some(name))),
                )
                .primary_action(
                    widget::button::suggested(fl!("create"))
                        .on_press(Message::CreateQuickPreset(None)),
                )
                .secondary_action(
                    widget::button::destructive(fl!("cancel")).on_press(Message::CancelDialog),
                )
                .into(),
            Dialog::DeleteQuickPreset(name) => widget::dialog()
                .title(fl!("delete-quick-preset"))
                .body(fl!("delete-confirm", name = name))
                .primary_action(
                    widget::button::destructive(fl!("delete"))
                        .on_press(Message::DeleteQuickPreset(name.to_owned())),
                )
                .secondary_action(
                    widget::button::text(fl!("cancel")).on_press(Message::CancelDialog),
                )
                .into(),
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
                        .on_submit(|name| Message::ModifiableSelectAddDialogSubmit(Some(name))),
                )
                .primary_action(
                    widget::button::suggested(fl!("create"))
                        .on_press(Message::ModifiableSelectAddDialogSubmit(None)),
                )
                .secondary_action(
                    widget::button::destructive(fl!("cancel")).on_press(Message::CancelDialog),
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
        })
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(custom_category) = self.nav_model.active_data::<CustomCategory>() {
            match custom_category {
                CustomCategory::QuickPresets => {
                    if let Some(quick_presets) = &self.quick_presets {
                        widget::column()
                            .push(
                                widget::button::standard(fl!("create-quick-preset"))
                                    .on_press(Message::ShowCreateQuickPresetDialog),
                            )
                            .push(quick_presets::quick_presets(
                                quick_presets,
                                Message::EditQuickPreset,
                                Message::ActivateQuickPreset,
                                Message::ShowDeleteQuickPresetDialog,
                            ))
                            .into()
                    } else {
                        widget::text(fl!("loading-item", item = fl!("quick-presets"))).into()
                    }
                }
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
        widget::column()
            .push(
                widget::text::title2(category_id.translate())
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center),
            )
            .extend(
                self.settings
                    .iter()
                    .map(|(setting_id, setting)| Self::view_setting(*setting_id, setting)),
            )
            .into()
    }

    fn view_setting<'a>(setting_id: SettingId, setting: &'a Setting) -> Element<'a, Message> {
        match setting {
            Setting::Toggle { value } => toggle::toggle(setting_id, *value, move |new_value| {
                Message::SetSetting(setting_id, new_value.into())
            }),
            Setting::I32Range { setting, value } => range::i32_range(
                setting_id,
                setting.range.clone(),
                *value,
                move |new_value| Message::SetSetting(setting_id, new_value.into()),
            ),
            Setting::Select { setting, value } => {
                select::select(setting_id, setting, value, move |value| {
                    Message::SetSetting(setting_id, Cow::from(value.to_owned()).into())
                })
            }
            Setting::OptionalSelect { setting, value } => {
                select::optional_select(setting_id, setting, value.as_deref(), move |value| {
                    Message::SetSetting(
                        setting_id,
                        value.map(ToOwned::to_owned).map(Cow::from).into(),
                    )
                })
            }
            Setting::ModifiableSelect { setting, value } => select::modifiable_select(
                setting_id,
                setting,
                value.as_deref(),
                {
                    move |value| Message::SetSetting(setting_id, Cow::from(value.to_owned()).into())
                },
                Message::ShowModifiableSelectAddDialog(setting_id),
                Message::ShowModifiableSelectRemoveDialog(setting_id),
            ),
            Setting::Equalizer { setting, value } => {
                equalizer::responsive_equalizer(setting, value, move |index, value| {
                    Message::SetEqualizerBand(setting_id, index, value)
                })
            }
            Setting::Information {
                value: _,
                translated_value: translated_text,
            } => information::information(setting_id, Cow::Borrowed(translated_text)),
        }
    }

    pub fn context_drawer(&self) -> Option<ContextDrawer<Message>> {
        if let Some(CustomCategory::QuickPresets) = self.nav_model.active_data() {
            self.editing_quick_preset
                .as_ref()
                .map(|editing_quick_preset| ContextDrawer {
                    title: Some(editing_quick_preset.name.as_str().into()),
                    header_actions: Vec::new(),
                    header: None,
                    content: widget::column()
                        .extend(
                            editing_quick_preset
                                .fields
                                .iter()
                                .enumerate()
                                .map(|(i, field)| {
                                    widget::column()
                                        .padding(8)
                                        .push(
                                            widget::toggler(field.is_enabled)
                                                .label(field.setting_id.translate())
                                                .width(Length::Fill)
                                                .on_toggle(move |enabled| {
                                                    Message::EditQuickPresetToggleField(i, enabled)
                                                }),
                                        )
                                        .push(widget::text::body(settings::localize_value(
                                            self.device.setting(&field.setting_id).as_ref(),
                                            &field.value,
                                        )))
                                        .into()
                                }),
                        )
                        .push(
                            widget::button::standard(fl!("overwrite-with-current-settings"))
                                .on_press(Message::SnapshotQuickPresetSettings(
                                    editing_quick_preset.name.to_owned(),
                                )),
                        )
                        .into(),
                    footer: None,
                    on_close: Message::EditQuickPresetClose,
                })
        } else {
            None
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SetSetting(setting_id, value) => {
                let device = self.device.clone();
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
            Message::SetEqualizerBand(setting_id, index, new_value) => {
                let device = self.device.clone();
                if let Some(Setting::Equalizer {
                    setting: _,
                    value: values,
                }) = self.device.setting(&setting_id)
                {
                    let mut new_values = values.clone();
                    new_values[index as usize] = new_value;
                    Action::Task(
                        Task::future(async move {
                            device
                                .set_setting_values(vec![(setting_id, new_values.into())])
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
            Message::RefreshQuickPresets => Action::Task(self.refresh_quick_presets()),
            Message::RefreshSettings => Action::Task(self.refresh_settings()),
            Message::Warning(message) => Action::Warning(message),
            Message::ActivateQuickPreset(index) => {
                let Some(name) = self
                    .quick_presets
                    .as_ref()
                    .and_then(|presets| presets.get(index))
                    .map(|preset| preset.name.clone())
                else {
                    return Action::None;
                };
                let device = self.device.0.clone();
                let quick_presets_handler = self.quick_presets_handler.clone();
                Action::Task(
                    Task::future(async move {
                        quick_presets_handler
                            .activate(device.as_ref(), name)
                            .await
                            .map_err(handle_soft_error!())?;
                        Ok(Message::RefreshQuickPresets)
                    })
                    .map(coalesce_result),
                )
            }
            Message::EditQuickPreset(index) => {
                self.editing_quick_preset = self
                    .quick_presets
                    .as_ref()
                    .and_then(|presets| presets.get(index))
                    .cloned();
                Action::None
            }
            Message::SetQuickPresets(quick_presets) => {
                if let Some(editing_quick_preset) = &self.editing_quick_preset {
                    self.editing_quick_preset = quick_presets
                        .iter()
                        .find(|preset| preset.name == editing_quick_preset.name)
                        .cloned()
                }
                self.quick_presets = Some(quick_presets);
                Action::None
            }
            Message::ShowCreateQuickPresetDialog => {
                self.dialog = Some(Dialog::CreateQuickPreset(String::new()));
                Action::FocusTextInput(widget::Id::new("create-quick-preset-name"))
            }
            Message::SetCreateQuickPresetName(name) => {
                self.dialog = Some(Dialog::CreateQuickPreset(name));
                Action::None
            }
            Message::CancelDialog => {
                self.dialog = None;
                Action::None
            }
            Message::CreateQuickPreset(override_name) => {
                let Some(Dialog::CreateQuickPreset(name)) = self.dialog.take() else {
                    return Action::None;
                };
                let name = override_name.unwrap_or(name);

                let device = self.device.clone();
                let quick_presets_handler = self.quick_presets_handler.clone();
                Action::Task(
                    Task::future(async move {
                        quick_presets_handler
                            .save(device.as_ref(), name)
                            .await
                            .map_err(handle_soft_error!())?;
                        Ok(Message::RefreshQuickPresets)
                    })
                    .map(coalesce_result),
                )
            }
            Message::ShowDeleteQuickPresetDialog(index) => {
                if let Some(quick_presets) = &self.quick_presets {
                    self.dialog = quick_presets
                        .get(index)
                        .map(|preset| Dialog::DeleteQuickPreset(preset.name.to_owned()));
                }
                Action::None
            }
            Message::DeleteQuickPreset(name) => {
                self.dialog = None;
                let quick_presets_handler = self.quick_presets_handler.clone();
                let device = self.device.clone();
                Action::Task(
                    Task::future(async move {
                        quick_presets_handler
                            .delete(device.as_ref(), name)
                            .await
                            .map_err(handle_soft_error!())?;
                        Ok(Message::RefreshQuickPresets)
                    })
                    .map(coalesce_result),
                )
            }
            Message::SnapshotQuickPresetSettings(name) => {
                let device = self.device.clone();
                let quick_presets_handler = self.quick_presets_handler.clone();
                Action::Task(
                    Task::future(async move {
                        quick_presets_handler
                            .save(device.as_ref(), name)
                            .await
                            .map_err(handle_soft_error!())?;
                        Ok(Message::RefreshQuickPresets)
                    })
                    .map(coalesce_result),
                )
            }
            Message::EditQuickPresetToggleField(field_index, is_enabled) => {
                if let Some(preset) = &mut self.editing_quick_preset {
                    let preset_name = preset.name.to_owned();

                    // eagerly update the dispalyed value. ideally we would revert back to
                    // what it was if the returned future fails.
                    let field = &mut preset.fields[field_index];
                    field.is_enabled = is_enabled;

                    let setting_id = field.setting_id;
                    let device = self.device.clone();
                    let quick_presets_handler = self.quick_presets_handler.clone();
                    return Action::Task(
                        Task::future(async move {
                            quick_presets_handler
                                .toggle_field(device.as_ref(), preset_name, setting_id, is_enabled)
                                .await
                                .map_err(handle_soft_error!())?;
                            Ok(Message::EditQuickPresetModified)
                        })
                        .map(coalesce_result),
                    );
                }
                Action::None
            }
            Message::EditQuickPresetClose => {
                self.editing_quick_preset = None;
                Action::None
            }
            Message::EditQuickPresetModified => {
                // TODO either modify in place instead of re-fetching all presets, or fetch only the modified preset
                let quick_presets_handler = self.quick_presets_handler.clone();
                let device = self.device.clone();
                Action::Task(
                    Task::future(async move {
                        quick_presets_handler
                            .quick_presets(device.as_ref())
                            .await
                            .map(Message::SetQuickPresets)
                            .map_err(handle_soft_error!())
                    })
                    .map(coalesce_result),
                )
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
        }
    }
}

fn labeled_setting_row<'a, M>(
    label: impl Into<Cow<'a, str>> + 'a,
    element: impl Into<Element<'a, M>>,
) -> Element<'a, M>
where
    M: 'a,
{
    widget::row::with_children(vec![
        widget::vertical_space()
            .height(MIN_SETTING_ROW_HEIGHT)
            .into(),
        widget::text(label)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .into(),
        element.into(),
    ])
    .spacing(20)
    .align_y(alignment::Vertical::Center)
    .into()
}
