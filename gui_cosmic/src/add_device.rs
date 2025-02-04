use std::sync::Arc;

use cosmic::{
    iced::Length,
    widget::{self, icon, Id},
    Apply, Element, Task,
};
use openscq30_lib::{
    api::device::{DeviceDescriptor, GenericDeviceDescriptor, OpenSCQ30DeviceRegistry},
    soundcore_device::device_model::DeviceModel,
};
use strum::IntoEnumIterator;
use tracing::error;

use crate::fl;

pub struct AddDeviceModel {
    stage: Stage,
}

enum Stage {
    ModelSelection(ModelSelectionModel),
    LoadingDeviceRegistry(LoadingDeviceRegistryModel),
    LoadingDevices(LoadingDevicesModel),
    SelectDevice(SelectDeviceModel),
    Error(String),
}

struct ModelSelectionModel {
    search_id: Id,
    search_query: String,
}
struct LoadingDeviceRegistryModel {
    device_model: DeviceModel,
}
struct LoadingDevicesModel {
    device_registry: Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>,
    device_model: DeviceModel,
}
struct SelectDeviceModel {
    search_id: Id,
    search_query: String,
    devices: Vec<GenericDeviceDescriptor>,
    device_model: DeviceModel,
    device_registry: Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>,
}

#[derive(Clone, Debug)]
pub enum Message {
    SetDeviceModelSearchQuery(String),
    SelectModel(DeviceModel),
    SelectDevice(usize),
    SetDeviceRegistry(DeviceRegistryPlusDebug),
    SetDeviceList(Vec<GenericDeviceDescriptor>),
    SetDeviceNameSearchQuery(String),
    SetErrorMessage(String),
    RefreshDeviceList,
}

#[derive(Clone)]
pub struct DeviceRegistryPlusDebug(pub Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>);
impl std::fmt::Debug for DeviceRegistryPlusDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OpenSCQ30DeviceRegistry").finish()
    }
}

pub enum Action {
    None,
    Task(Task<Message>),
    AddDevice {
        model: DeviceModel,
        descriptor: GenericDeviceDescriptor,
    },
}

impl AddDeviceModel {
    pub fn new() -> Self {
        Self {
            stage: Stage::ModelSelection(ModelSelectionModel {
                search_id: Id::unique(),
                search_query: String::new(),
            }),
        }
    }
    pub fn view(&self) -> Element<Message> {
        match &self.stage {
            Stage::ModelSelection(ui_model) => Self::device_model_selection(ui_model),
            Stage::LoadingDeviceRegistry(_ui_model) => Self::loading(fl!("device-registry")),
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
                    .on_input(|text| Message::SetDeviceModelSearchQuery(text)),
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
                            .on_input(|text| Message::SetDeviceNameSearchQuery(text)),
                    )
                    .push(
                        widget::button::standard(fl!("refresh"))
                            .leading_icon(icon::from_name("view-refresh-symbolic"))
                            .on_press(Message::RefreshDeviceList),
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
                                .name()
                                .to_lowercase()
                                .contains(&ui_model.search_query.to_lowercase())
                        })
                        .map(|(index, device)| {
                            widget::button::text(device.name())
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
                self.stage =
                    Stage::LoadingDeviceRegistry(LoadingDeviceRegistryModel { device_model });
                return Action::Task(Task::perform(
                    Self::get_device_registry(device_model),
                    move |result| match result {
                        Ok(registry) => {
                            Message::SetDeviceRegistry(DeviceRegistryPlusDebug(registry))
                        }
                        Err(err) => {
                            error!("{} obtaining device registry: {err:?}", device_model);
                            Message::SetErrorMessage(format!("{err}"))
                        }
                    },
                ));
            }
            Message::SetDeviceRegistry(registry) => {
                if let Stage::LoadingDeviceRegistry(ui_model) = &self.stage {
                    let device_model = ui_model.device_model;
                    self.stage = Stage::LoadingDevices(LoadingDevicesModel {
                        device_registry: registry.0.to_owned(),
                        device_model: ui_model.device_model,
                    });
                    return Action::Task(Task::perform(
                        Self::get_devices(registry.0),
                        move |result| match result {
                            Ok(devices) => Message::SetDeviceList(devices),
                            Err(err) => {
                                error!("{} fetching devices: {err:?}", device_model);
                                Message::SetErrorMessage(format!("{err}"))
                            }
                        },
                    ));
                }
            }
            Message::RefreshDeviceList => {
                if let Stage::SelectDevice(ui_model) = &self.stage {
                    let device_model = ui_model.device_model;
                    return Action::Task(Task::perform(
                        Self::get_devices(ui_model.device_registry.clone()),
                        move |result| match result {
                            Ok(devices) => Message::SetDeviceList(devices),
                            Err(err) => {
                                error!("{} fetching devices: {err:?}", device_model);
                                Message::SetErrorMessage(format!("{err}"))
                            }
                        },
                    ));
                }
            }
            Message::SetDeviceList(devices) => {
                if let Stage::LoadingDevices(ui_model) = &self.stage {
                    self.stage = Stage::SelectDevice(SelectDeviceModel {
                        devices,
                        search_id: Id::unique(),
                        search_query: String::new(),
                        device_model: ui_model.device_model,
                        device_registry: ui_model.device_registry.to_owned(),
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

    async fn get_device_registry(
        device_model: DeviceModel,
    ) -> openscq30_lib::Result<Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>> {
        device_model
            .device_registry::<openscq30_lib::futures::TokioFutures>(
                Some(tokio::runtime::Handle::current()),
                true,
            )
            .await
    }

    async fn get_devices(
        registry: Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>,
    ) -> openscq30_lib::Result<Vec<GenericDeviceDescriptor>> {
        registry.devices().await
    }
}
