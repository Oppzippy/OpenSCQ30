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

use crate::{fl, icons::view_refresh_symbolic};

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
    devices: Vec<ConnectionDescriptor>,
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
            Stage::LoadingDevices(_ui_model) => Self::loading(fl!("device-list")),
            Stage::SelectDevice(ui_model) => Self::select_device(ui_model),
            Stage::Error(message) => Self::error(message),
        }
    }

    fn device_model_selection(ui_model: &ModelSelectionModel) -> Element<'_, Message> {
        widget::column()
            .spacing(8)
            .push(
                widget::column()
                    .spacing(8)
                    // padding should not apply to the list of devices, since those are buttons with their own padding
                    .padding([0, 10])
                    .push(widget::text::title2(fl!("select-device-model")))
                    .push(
                        widget::search_input(fl!("device-model"), &ui_model.search_query)
                            .id(ui_model.search_id.clone())
                            .on_input(Message::SetDeviceModelSearchQuery),
                    ),
            )
            .push(widget::scrollable(
                widget::column().extend(
                    DeviceModel::iter()
                        .filter(|device_model| {
                            device_model
                                .translate()
                                .to_lowercase()
                                .contains(&ui_model.search_query.to_lowercase())
                        })
                        .map(|device_model| {
                            widget::button::text(device_model.translate())
                                .width(Length::Fill)
                                .on_press(Message::SelectModel(device_model, false))
                                .into()
                        }),
                ),
            ))
            .into()
    }

    fn select_device(ui_model: &SelectDeviceModel) -> Element<'_, Message> {
        widget::responsive(|size| {
            widget::column()
                .spacing(8)
                .push(
                    widget::column()
                        .spacing(8)
                        .padding([0, 10])
                        .push(widget::text::title2(fl!(
                            "select-your",
                            name = ui_model.device_model.translate()
                        )))
                        .push({
                            let search_input =
                                widget::search_input(fl!("device-name"), &ui_model.search_query)
                                    .id(ui_model.search_id.clone())
                                    .on_input(Message::SetDeviceNameSearchQuery);
                            let demo_toggle = widget::toggler(ui_model.is_demo_mode)
                                .label(fl!("demo-mode"))
                                .on_toggle(|enabled| {
                                    Message::SelectModel(ui_model.device_model, enabled)
                                });
                            let refresh = widget::button::standard(fl!("refresh"))
                                .leading_icon(view_refresh_symbolic())
                                .on_press(Message::SelectModel(
                                    ui_model.device_model,
                                    ui_model.is_demo_mode,
                                ));
                            if size.width < 450f32 {
                                Element::from(
                                    widget::column()
                                        .spacing(8)
                                        .push(search_input)
                                        .push(
                                            widget::row::with_children(vec![
                                                demo_toggle
                                                    .apply(widget::container)
                                                    .width(Length::Fill)
                                                    .into(),
                                                refresh
                                                    .apply(widget::container)
                                                    .width(Length::Fill)
                                                    .align_x(alignment::Horizontal::Right)
                                                    .into(),
                                            ])
                                            .spacing(8)
                                            .align_y(alignment::Vertical::Center),
                                        )
                                        .spacing(8),
                                )
                            } else {
                                widget::row::with_children(vec![
                                    search_input.into(),
                                    demo_toggle.into(),
                                    refresh.into(),
                                ])
                                .align_y(alignment::Vertical::Center)
                                .spacing(8)
                                .into()
                            }
                        }),
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
                                widget::button::custom(
                                    widget::row::with_children(vec![
                                        widget::text(&device.name).into(),
                                        widget::text(device.mac_address.to_string())
                                            .align_x(alignment::Horizontal::Right)
                                            .width(Length::Fill)
                                            .into(),
                                    ])
                                    .align_y(alignment::Vertical::Center),
                                )
                                .name(&device.name)
                                .class(widget::button::ButtonClass::Text)
                                .width(Length::Fill)
                                .on_press(Message::SelectDevice(index, ui_model.is_demo_mode))
                                .into()
                            }),
                    ),
                ))
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
            Message::SelectModel(device_model, is_demo_mode) => {
                self.stage = Stage::LoadingDevices(LoadingDevicesModel { device_model });
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
                if let Stage::LoadingDevices(ui_model) = &self.stage {
                    self.stage = Stage::SelectDevice(SelectDeviceModel {
                        devices,
                        search_id: Id::unique(),
                        search_query: String::new(),
                        device_model: ui_model.device_model,
                        is_demo_mode,
                    });
                }
            }
            Message::SetDeviceNameSearchQuery(query) => {
                if let Stage::SelectDevice(ref mut ui_model) = self.stage {
                    ui_model.search_query = query;
                }
            }
            Message::SetErrorMessage(message) => self.stage = Stage::Error(message),
            Message::SelectDevice(index, is_demo) => {
                if let Stage::SelectDevice(ui_model) = &self.stage {
                    let descriptor = ui_model.devices[index].clone();
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
