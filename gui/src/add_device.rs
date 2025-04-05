use std::sync::Arc;

use cosmic::{
    Apply, Element, Task,
    iced::Length,
    widget::{self, Id, icon},
};
use openscq30_lib::{
    api::{OpenSCQ30Session, connection::DeviceDescriptor},
    devices::DeviceModel,
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
    LoadingDevices(LoadingDevicesModel),
    SelectDevice(SelectDeviceModel),
    Error(String),
}

struct ModelSelectionModel {
    search_id: Id,
    search_query: String,
}
struct LoadingDevicesModel {
    device_model: DeviceModel,
}
struct SelectDeviceModel {
    search_id: Id,
    search_query: String,
    devices: Vec<DeviceDescriptor>,
    device_model: DeviceModel,
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Message {
    SetDeviceModelSearchQuery(String),
    SelectModel(DeviceModel),
    SelectDevice(usize),
    SetDeviceList(Vec<DeviceDescriptor>),
    SetDeviceNameSearchQuery(String),
    SetErrorMessage(String),
}

pub enum Action {
    None,
    Task(Task<Message>),
    AddDevice {
        model: DeviceModel,
        descriptor: DeviceDescriptor,
    },
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
    pub fn view(&self) -> Element<Message> {
        match &self.stage {
            Stage::ModelSelection(ui_model) => Self::device_model_selection(ui_model),
            Stage::LoadingDevices(_ui_model) => Self::loading(fl!("device-list")),
            Stage::SelectDevice(ui_model) => Self::select_device(ui_model),
            Stage::Error(message) => Self::error(message),
        }
    }

    fn device_model_selection(ui_model: &ModelSelectionModel) -> Element<Message> {
        widget::column()
            .push(
                widget::search_input(fl!("device-model"), &ui_model.search_query)
                    .id(ui_model.search_id.clone())
                    .on_input(Message::SetDeviceModelSearchQuery),
            )
            .push(widget::scrollable(
                widget::column().extend(
                    DeviceModel::iter()
                        .filter(|device_type| {
                            let name: &'static str = device_type.into();
                            name.to_lowercase()
                                .contains(&ui_model.search_query.to_lowercase())
                        })
                        .map(|device_type| {
                            widget::button::text(<&'static str>::from(device_type))
                                .width(Length::Fill)
                                .on_press(Message::SelectModel(device_type))
                                .into()
                        }),
                ),
            ))
            .into()
    }

    fn select_device(ui_model: &SelectDeviceModel) -> Element<Message> {
        widget::column()
            .push(widget::text::title2(fl!(
                "select-your",
                name = <&'static str>::from(ui_model.device_model)
            )))
            .push(
                widget::row()
                    .push(
                        widget::search_input(fl!("device-name"), &ui_model.search_query)
                            .id(ui_model.search_id.clone())
                            .on_input(Message::SetDeviceNameSearchQuery),
                    )
                    .push(
                        widget::button::standard(fl!("refresh"))
                            .leading_icon(icon::from_name("view-refresh-symbolic"))
                            .on_press(Message::SelectModel(ui_model.device_model)),
                    ),
            )
            .push(widget::scrollable(
                widget::column().extend(
                    ui_model
                        .devices
                        .iter()
                        .enumerate()
                        .filter(|(_, device)| {
                            device
                                .name
                                .to_lowercase()
                                .contains(&ui_model.search_query.to_lowercase())
                        })
                        .map(|(index, device)| {
                            widget::button::text(&device.name)
                                .width(Length::Fill)
                                .on_press(Message::SelectDevice(index))
                                .into()
                        }),
                ),
            ))
            .into()
    }

    fn loading(item: String) -> Element<'static, Message> {
        widget::container(widget::text::title2(fl!("loading-item", item = item)))
            .center(Length::Fill)
            .into()
    }

    fn error(message: &str) -> Element<Message> {
        widget::column()
            .push(widget::text::title2(fl!("error-loading-devices")))
            .push(widget::text::monotext(message))
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
            Message::SelectModel(device_model) => {
                self.stage = Stage::LoadingDevices(LoadingDevicesModel { device_model });
                let session = self.session.clone();
                return Action::Task(Task::perform(
                    async move { session.list_devices(device_model).await },
                    move |result| match result {
                        Ok(devices) => Message::SetDeviceList(devices),
                        Err(err) => {
                            error!("{} fetching devices: {err:?}", device_model);
                            Message::SetErrorMessage(format!("{err}"))
                        }
                    },
                ));
            }
            Message::SetDeviceList(devices) => {
                if let Stage::LoadingDevices(ui_model) = &self.stage {
                    self.stage = Stage::SelectDevice(SelectDeviceModel {
                        devices,
                        search_id: Id::unique(),
                        search_query: String::new(),
                        device_model: ui_model.device_model,
                    })
                }
            }
            Message::SetDeviceNameSearchQuery(query) => {
                if let Stage::SelectDevice(ref mut ui_model) = self.stage {
                    ui_model.search_query = query;
                }
            }
            Message::SetErrorMessage(message) => self.stage = Stage::Error(message),
            Message::SelectDevice(index) => {
                if let Stage::SelectDevice(ui_model) = &self.stage {
                    return Action::AddDevice {
                        model: ui_model.device_model,
                        descriptor: ui_model.devices[index].clone(),
                    };
                }
            }
        }
        Action::None
    }
}
