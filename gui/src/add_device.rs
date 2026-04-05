use std::sync::Arc;

use cosmic::{
    Apply, Element, Task,
    iced::{Length, alignment},
    widget::{self, Id},
};
use openscq30_i18n::Translate;
use openscq30_lib::{
    DeviceModel, OpenSCQ30Session, connection::ConnectionDescriptor, storage::PairedDevice,
};
use strum::IntoEnumIterator;
use tracing::error;

use crate::fl;

pub struct AddDeviceModel {
    stage: Stage,
    session: Arc<OpenSCQ30Session>,
}

enum Stage {
    ModelSelection(ModelSelectionModel),
    SelectDevice(SelectDeviceModel),
    Error(String),
}

struct ModelSelectionModel {
    search_id: Id,
    search_query: String,
}
struct SelectDeviceModel {
    search_id: Id,
    search_query: String,
    devices: Option<Vec<ConnectionDescriptor>>,
    device_model: DeviceModel,
    is_demo_mode: bool,
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Message {
    SetDeviceModelSearchQuery(String),
    SelectModel(DeviceModel, bool),
    SelectDevice(usize, bool),
    SetDeviceList(Vec<ConnectionDescriptor>, bool),
    SetDeviceNameSearchQuery(String),
    SetErrorMessage(String),
}

pub enum Action {
    None,
    Task(Task<Message>),
    AddDevice(PairedDevice),
}

impl AddDeviceModel {
    pub fn new(session: Arc<OpenSCQ30Session>) -> Self {
        Self {
            session,
            stage: Stage::ModelSelection(ModelSelectionModel {
                search_id: Id::unique(),
                search_query: String::new(),
            }),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.stage {
            Stage::ModelSelection(ui_model) => Self::device_model_selection(ui_model),
            Stage::SelectDevice(ui_model) => Self::select_device(ui_model),
            Stage::Error(message) => Self::error(message),
        }
    }

    fn device_model_selection(ui_model: &ModelSelectionModel) -> Element<'_, Message> {
        widget::column![
            widget::column![
                widget::text::title2(fl!("select-device-model")),
                widget::search_input(fl!("device-model"), &ui_model.search_query)
                    .id(ui_model.search_id.clone())
                    .on_input(Message::SetDeviceModelSearchQuery),
            ]
            .spacing(8)
            // padding should not apply to the list of devices, since those are buttons with their own padding
            .padding([0, 10]),
            widget::scrollable(widget::column(
                DeviceModel::iter()
                    .filter(|device_model| {
                        device_model
                            .translate()
                            .to_lowercase()
                            .contains(&ui_model.search_query.to_lowercase())
                    })
                    .map(|device_model| {
                        // custom button with ButtonClass::Text because button::text ignores width(Length::Fill)
                        widget::button::custom(widget::text(device_model.translate()))
                            .class(widget::button::ButtonClass::Text)
                            .width(Length::Fill)
                            .on_press(Message::SelectModel(device_model, false))
                            .into()
                    }),
            ),)
        ]
        .spacing(8)
        .into()
    }

    fn select_device(ui_model: &SelectDeviceModel) -> Element<'_, Message> {
        widget::responsive(|size| {
            widget::column![
                widget::column![
                    widget::text::title2(fl!(
                        "select-your",
                        name = ui_model.device_model.translate()
                    )),
                    {
                        let search_input =
                            widget::search_input(fl!("device-name"), &ui_model.search_query)
                                .id(ui_model.search_id.clone())
                                .on_input(Message::SetDeviceNameSearchQuery);
                        let demo_toggle = widget::toggler(ui_model.is_demo_mode)
                            .label(fl!("demo-mode"))
                            .spacing(4)
                            .on_toggle(|enabled| {
                                Message::SelectModel(ui_model.device_model, enabled)
                            });
                        let refresh = widget::button::standard(fl!("refresh"))
                            .leading_icon(widget::icon::from_name("view-refresh-symbolic"))
                            .on_press(Message::SelectModel(
                                ui_model.device_model,
                                ui_model.is_demo_mode,
                            ));
                        if size.width < 450f32 {
                            Element::from(
                                widget::column![
                                    search_input,
                                    widget::row![
                                        demo_toggle.apply(widget::container).width(Length::Fill),
                                        refresh
                                            .apply(widget::container)
                                            .width(Length::Fill)
                                            .align_x(alignment::Horizontal::Right),
                                    ]
                                    .spacing(8)
                                    .align_y(alignment::Vertical::Center),
                                ]
                                .spacing(8),
                            )
                        } else {
                            widget::row![search_input, demo_toggle, refresh]
                                .align_y(alignment::Vertical::Center)
                                .spacing(8)
                                .into()
                        }
                    }
                ]
                .spacing(8)
                .padding([0, 10]),
                if let Some(devices) = &ui_model.devices {
                    widget::scrollable(widget::column(
                        devices
                            .iter()
                            .enumerate()
                            .filter(|(_, device)| {
                                device
                                    .name
                                    .to_lowercase()
                                    .contains(&ui_model.search_query.to_lowercase())
                            })
                            .map(|(index, device)| {
                                widget::button::custom(
                                    widget::row![
                                        widget::text(&device.name),
                                        widget::text(device.mac_address.to_string())
                                            .align_x(alignment::Horizontal::Right)
                                            .width(Length::Fill),
                                    ]
                                    .align_y(alignment::Vertical::Center),
                                )
                                .name(&device.name)
                                .class(widget::button::ButtonClass::Text)
                                .width(Length::Fill)
                                .on_press(Message::SelectDevice(index, ui_model.is_demo_mode))
                                .into()
                            }),
                    ))
                    .into()
                } else {
                    Self::loading(fl!("device-list"))
                }
            ]
            .spacing(8)
            .into()
        })
        .into()
    }

    fn loading(item: String) -> Element<'static, Message> {
        widget::text::title2(fl!("loading-item", item = item))
            .apply(widget::container)
            .center(Length::Fill)
            .into()
    }

    fn error(message: &str) -> Element<'_, Message> {
        widget::column![
            widget::text::title2(fl!("error-loading-devices")),
            widget::text::monotext(message),
        ]
        .apply(widget::container)
        .center(Length::Fill)
        .into()
    }

    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SetDeviceModelSearchQuery(query) => {
                if let Stage::ModelSelection(ref mut ui_model) = self.stage {
                    ui_model.search_query = query;
                }
            }
            Message::SelectModel(device_model, is_demo_mode) => {
                if let Stage::SelectDevice(ref mut ui_model) = self.stage {
                    // if we're already on the select device page, don't clear the search and such
                    ui_model.device_model = device_model;
                    ui_model.is_demo_mode = is_demo_mode;
                    ui_model.devices = None;
                } else {
                    self.stage = Stage::SelectDevice(SelectDeviceModel {
                        search_id: Id::unique(),
                        search_query: String::new(),
                        devices: None,
                        device_model,
                        is_demo_mode,
                    });
                }
                let session = self.session.clone();
                return Action::Task(Task::perform(
                    async move {
                        if is_demo_mode {
                            session.list_demo_devices(device_model).await
                        } else {
                            session.list_devices(device_model).await
                        }
                    },
                    move |result| match result {
                        Ok(devices) => Message::SetDeviceList(devices, is_demo_mode),
                        Err(err) => {
                            error!("{} fetching devices: {err:?}", device_model);
                            Message::SetErrorMessage(format!("{err}"))
                        }
                    },
                ));
            }
            Message::SetDeviceList(devices, is_demo_mode) => {
                if let Stage::SelectDevice(ref mut ui_model) = self.stage {
                    ui_model.devices = Some(devices);
                    ui_model.is_demo_mode = is_demo_mode;
                }
            }
            Message::SetDeviceNameSearchQuery(query) => {
                if let Stage::SelectDevice(ref mut ui_model) = self.stage {
                    ui_model.search_query = query;
                }
            }
            Message::SetErrorMessage(message) => self.stage = Stage::Error(message),
            Message::SelectDevice(index, is_demo) => {
                if let Stage::SelectDevice(ui_model) = &self.stage
                    && let Some(devices) = &ui_model.devices
                {
                    let descriptor = devices[index].clone();
                    return Action::AddDevice(PairedDevice {
                        mac_address: descriptor.mac_address,
                        model: ui_model.device_model,
                        is_demo,
                    });
                }
            }
        }
        Action::None
    }
}
