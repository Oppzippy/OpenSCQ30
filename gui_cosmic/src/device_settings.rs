mod equalizer;
mod quick_presets;
mod select;
mod toggle;

use std::{borrow::Cow, collections::HashMap};

use cosmic::{
    Element, Task,
    iced::{Length, alignment},
    widget::{self, nav_bar},
};
use openscq30_lib::api::{
    quick_presets::{QuickPreset, QuickPresetsHandler},
    settings::{CategoryId, Setting, SettingId, Value},
};

use crate::{app::DebugOpenSCQ30Device, fl, handle_soft_error, utils::coalesce_result};

#[derive(Debug, Clone)]
pub enum Message {
    SetSetting(SettingId<'static>, Value),
    SetEqualizerBand(SettingId<'static>, u8, i16),
    Refresh,
    Warning(String),
    ShowCreateQuickPresetDialog,
    ActivateQuickPreset(usize),
    EditQuickPreset(usize),
    SetQuickPresets(Vec<QuickPreset>),
    CreateQuickPreset,
    SetCreateQuickPresetName(String),
    CancelDialog,
    EditQuickPresetToggleField(usize, bool),
    EditQuickPresetCancel,
    EditQuickPresetSave,
    EditQuickPresetDone,
    ShowOptionalSelectAddDialog(SettingId<'static>),
    ShowOptionalSelectRemoveDialog(SettingId<'static>),
    OptionalSelectAddDialogSubmit,
    OptionalSelectAddDialogSetName(String),
    OptionalSelectRemoveDialogSubmit,
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
    settings: Vec<(SettingId<'static>, Setting)>,
    quick_presets: Option<Vec<QuickPreset>>,
    dialog: Option<Dialog>,
    editing_quick_preset: Option<QuickPreset>,
}

enum Dialog {
    CreateQuickPreset(String),
    OptionalSelectAdd(SettingId<'static>, String),
    OptionalSelectRemove(SettingId<'static>, Cow<'static, str>),
}

impl DeviceSettingsModel {
    pub fn new(
        device: DebugOpenSCQ30Device,
        quick_presets_handler: QuickPresetsHandler,
    ) -> (Self, Task<Message>) {
        let mut nav_model = nav_bar::Model::default();
        for category in device.categories() {
            nav_model
                .insert()
                .text(category.0.clone())
                .data(category.clone());
        }
        nav_model
            .insert()
            .text(fl!("quick-presets"))
            .data(CategoryId(Cow::Borrowed("QuickPresets")));
        nav_model.activate_position(0);

        let mut model = Self {
            device,
            nav_model,
            settings: Vec::new(),
            quick_presets_handler,
            quick_presets: None,
            editing_quick_preset: None,
            dialog: None,
        };
        let task = model.refresh();
        (model, task)
    }

    pub fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Message> {
        self.nav_model.activate(id);
        self.refresh()
    }

    fn refresh(&mut self) -> Task<Message> {
        let Some(category_id) = self.nav_model.active_data::<CategoryId<'static>>() else {
            return Task::none();
        };
        if category_id == &CategoryId(Cow::Borrowed("QuickPresets")) {
            self.quick_presets = None;
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
                        .on_submit(Message::CreateQuickPreset),
                )
                .primary_action(
                    widget::button::suggested(fl!("create")).on_press(Message::CreateQuickPreset),
                )
                .secondary_action(
                    widget::button::destructive(fl!("cancel")).on_press(Message::CancelDialog),
                )
                .into(),
            Dialog::OptionalSelectAdd(setting_id, name) => widget::dialog()
                .title(fl!("add-item", name = setting_id.0.as_ref()))
                .control(
                    widget::text_input(fl!("name"), name)
                        .id(widget::Id::new(
                            "optional-select-dialog-add-item-text-input",
                        ))
                        .on_input(|text| Message::OptionalSelectAddDialogSetName(text))
                        .on_submit(Message::OptionalSelectAddDialogSubmit),
                )
                .primary_action(
                    widget::button::suggested(fl!("create"))
                        .on_press(Message::OptionalSelectAddDialogSubmit),
                )
                .secondary_action(
                    widget::button::destructive(fl!("cancel")).on_press(Message::CancelDialog),
                )
                .into(),
            Dialog::OptionalSelectRemove(setting_id, name) => widget::dialog()
                .title(fl!("remove-item", name = name.as_ref()))
                .body(fl!("remove-item-confirm", name = name.as_ref()))
                .primary_action(
                    widget::button::destructive(fl!("remove"))
                        .on_press(Message::OptionalSelectRemoveDialogSubmit),
                )
                .secondary_action(
                    widget::button::text(fl!("cancel")).on_press(Message::CancelDialog),
                )
                .into(),
        })
    }

    pub fn view(&self) -> Element<'_, Message> {
        let Some(category_id) = self.nav_model.active_data::<CategoryId<'static>>() else {
            return widget::row().into();
        };
        if category_id == &CategoryId(Cow::Borrowed("QuickPresets")) {
            if let Some(editing_quick_preset) = &self.editing_quick_preset {
                widget::column()
                    .push(widget::text::title4(fl!(
                        "editing-quick-preset",
                        name = editing_quick_preset.name.as_str()
                    )))
                    .push(
                        widget::row()
                            .push(
                                widget::button::destructive(fl!("cancel"))
                                    .on_press(Message::EditQuickPresetCancel),
                            )
                            .push(
                                widget::button::suggested(fl!("save"))
                                    .on_press(Message::EditQuickPresetSave),
                            ),
                    )
                    .extend(
                        editing_quick_preset
                            .settings
                            .iter()
                            .enumerate()
                            .map(|(i, field)| {
                                widget::row()
                                    .align_y(alignment::Vertical::Center)
                                    .padding(5)
                                    .push(
                                        widget::toggler(field.value.is_some())
                                            .label(&field.setting_id.0)
                                            .width(Length::Fill)
                                            .on_toggle(move |enabled| {
                                                Message::EditQuickPresetToggleField(i, enabled)
                                            }),
                                    )
                                    .push_maybe(
                                        field
                                            .value
                                            .as_ref()
                                            .map(|v| widget::text::body(format!("{v:?}"))),
                                    )
                                    .into()
                            }),
                    )
                    .into()
            } else if let Some(quick_presets) = &self.quick_presets {
                widget::column()
                    .push(
                        widget::button::standard(fl!("create-quick-preset"))
                            .on_press(Message::ShowCreateQuickPresetDialog),
                    )
                    .push(quick_presets::quick_presets(
                        quick_presets,
                        Message::EditQuickPreset,
                        Message::ActivateQuickPreset,
                    ))
                    .into()
            } else {
                widget::text(fl!("loading-item", item = fl!("quick-presets"))).into()
            }
        } else {
            self.view_settings(category_id)
        }
    }

    fn view_settings<'a>(&'a self, category_id: &'a CategoryId<'a>) -> Element<'a, Message> {
        widget::column()
            .push(
                widget::text::title2(category_id.0.as_ref())
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center),
            )
            .extend(self.settings.iter().map(|(setting_id, setting)| {
                let setting_id = setting_id.to_owned();
                match setting {
                    Setting::Toggle { value } => {
                        toggle::toggle(setting_id.clone(), *value, move |new_value| {
                            Message::SetSetting(setting_id.clone(), new_value.into())
                        })
                    }
                    Setting::I32Range { setting, value } => todo!(),
                    Setting::Select { setting, value } => {
                        select::select(setting_id.clone(), setting, value, move |value| {
                            Message::SetSetting(
                                setting_id.clone(),
                                Cow::from(value.to_owned()).into(),
                            )
                        })
                    }
                    Setting::OptionalSelect { setting, value } => select::optional_select(
                        setting_id.clone(),
                        setting,
                        value.as_deref(),
                        {
                            let setting_id = setting_id.clone();
                            move |value| {
                                Message::SetSetting(
                                    setting_id.clone(),
                                    value.map(ToOwned::to_owned).map(Cow::from).into(),
                                )
                            }
                        },
                        setting
                            .has_add_button
                            .then_some(Message::ShowOptionalSelectAddDialog(setting_id.clone())),
                        setting
                            .has_add_button
                            .then_some(Message::ShowOptionalSelectRemoveDialog(setting_id.clone())),
                    ),
                    Setting::MultiSelect { setting, value } => todo!(),
                    Setting::Equalizer {
                        setting,
                        values: value,
                    } => equalizer::responsive_equalizer(setting, value, move |index, value| {
                        Message::SetEqualizerBand(setting_id.clone(), index, value)
                    }),
                }
            }))
            .into()
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
                        Ok(Message::Refresh)
                    })
                    .map(coalesce_result),
                )
            }
            Message::SetEqualizerBand(setting_id, index, new_value) => {
                let device = self.device.clone();
                if let Some(Setting::Equalizer { setting, values }) =
                    self.device.setting(&setting_id)
                {
                    let mut new_values = values.clone();
                    new_values[index as usize] = new_value;
                    Action::Task(
                        Task::future(async move {
                            device
                                .set_setting_values(vec![(setting_id, new_values.into())])
                                .await
                                .map_err(handle_soft_error!())?;
                            Ok(Message::Refresh)
                        })
                        .map(coalesce_result),
                    )
                } else {
                    Action::None
                }
            }
            Message::Refresh => Action::Task(self.refresh()),
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
                        Ok(Message::Refresh)
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
            Message::CreateQuickPreset => {
                let Some(Dialog::CreateQuickPreset(name)) = self.dialog.take() else {
                    return Action::None;
                };
                let device = self.device.clone();
                let quick_presets_handler = self.quick_presets_handler.clone();
                Action::Task(
                    Task::future(async move {
                        quick_presets_handler
                            .save(device.as_ref(), name, HashMap::new())
                            .await
                            .map_err(handle_soft_error!())?;
                        Ok(Message::Refresh)
                    })
                    .map(coalesce_result),
                )
            }
            Message::EditQuickPresetToggleField(field_index, is_enabled) => {
                if let Some(preset) = &mut self.editing_quick_preset {
                    let field = &mut preset.settings[field_index];
                    field.value = is_enabled
                        .then(|| self.device.setting(&field.setting_id).map(Into::into))
                        .flatten();
                } else {
                    return Action::None;
                }
                Action::None
            }
            Message::EditQuickPresetCancel => {
                self.editing_quick_preset = None;
                Action::None
            }
            Message::EditQuickPresetSave => {
                let Some(quick_preset) = self.editing_quick_preset.take() else {
                    return Action::None;
                };
                let device = self.device.clone();
                let quick_presets_handler = self.quick_presets_handler.clone();
                Action::Task(
                    Task::future(async move {
                        quick_presets_handler
                            .save(
                                device.as_ref(),
                                quick_preset.name,
                                quick_preset
                                    .settings
                                    .into_iter()
                                    .flat_map(|field| {
                                        field.value.map(|value| (field.setting_id, value))
                                    })
                                    .collect(),
                            )
                            .await
                            .map_err(handle_soft_error!())?;
                        Ok(Message::EditQuickPresetDone)
                    })
                    .map(coalesce_result),
                )
            }
            Message::EditQuickPresetDone => {
                self.editing_quick_preset = None;
                Action::Task(self.refresh())
            }
            Message::ShowOptionalSelectAddDialog(setting_id) => {
                self.dialog = Some(Dialog::OptionalSelectAdd(setting_id, String::new()));
                Action::FocusTextInput(widget::Id::new(
                    "optional-select-dialog-add-item-text-input",
                ))
            }
            Message::ShowOptionalSelectRemoveDialog(setting_id) => {
                let selected_item = self
                    .settings
                    .iter()
                    .find(|item| item.0 == setting_id)
                    .and_then(|item| {
                        if let (_setting_id, Setting::OptionalSelect { setting, value }) = item {
                            value.to_owned()
                        } else {
                            None
                        }
                    });
                if let Some(selected_item) = selected_item {
                    self.dialog = Some(Dialog::OptionalSelectRemove(setting_id, selected_item));
                } else {
                    tracing::error!(
                        r#"tried to open optional select remove dialog for {setting_id:?}, but selected item is None.
                        current settings: {:?}
                        "#,
                        self.settings,
                    );
                }
                Action::None
            }
            Message::OptionalSelectAddDialogSetName(new_name) => {
                if let Some(Dialog::OptionalSelectAdd(_setting_id, name)) = &mut self.dialog {
                    *name = new_name;
                }
                Action::None
            }
            Message::OptionalSelectAddDialogSubmit => {
                if let Some(Dialog::OptionalSelectAdd(setting_id, name)) = self.dialog.take() {
                    let device = self.device.clone();
                    Action::Task(
                        Task::future(async move {
                            device
                                .set_setting_values(vec![(
                                    setting_id,
                                    Value::OptionalString(Some(Cow::Owned(name))),
                                )])
                                .await
                                .map_err(handle_soft_error!())?;
                            Ok(Message::Refresh)
                        })
                        .map(coalesce_result),
                    )
                } else {
                    Action::None
                }
            }
            Message::OptionalSelectRemoveDialogSubmit => {
                if let Some(Dialog::OptionalSelectRemove(setting_id, name)) = self.dialog.take() {
                    let is_target_item_still_selected = self
                        .settings
                        .iter()
                        .find(|item| item.0 == setting_id)
                        .and_then(|item| {
                            if let (_setting_id, Setting::OptionalSelect { setting: _, value }) =
                                item
                            {
                                value.as_ref().map(|v| v == &name)
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    if is_target_item_still_selected {
                        let device = self.device.clone();
                        Action::Task(
                            Task::future(async move {
                                device
                                    .set_setting_values(vec![(
                                        setting_id,
                                        Value::OptionalString(None),
                                    )])
                                    .await
                                    .map_err(handle_soft_error!())?;
                                Ok(Message::Refresh)
                            })
                            .map(coalesce_result),
                        )
                    } else {
                        Action::None
                    }
                } else {
                    Action::None
                }
            }
        }
    }
}
