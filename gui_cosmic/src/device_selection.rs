use std::sync::Arc;

use cosmic::{
    iced::{alignment, Length, Pixels},
    iced_core::text::LineHeight,
    widget, Apply, Element, Task,
};
use openscq30_lib::{api::OpenSCQ30Session, storage::PairedDevice};

use crate::{fl, handle_soft_error, utils::coalesce_result};

pub struct DeviceSelectionModel {
    paired_devices: Vec<PairedDevice>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConnectToDevice(usize),
    RemoveDevice(usize),
    AddDevice,
    SetPairedDevices(Vec<PairedDevice>),
    Warning(String),
}

pub enum Action {
    ConnectToDevice(PairedDevice),
    RemoveDevice(PairedDevice),
    AddDevice,
    None,
    Warning(String),
}

impl DeviceSelectionModel {
    pub fn new(session: Arc<OpenSCQ30Session>) -> (Self, Task<Message>) {
        let model = DeviceSelectionModel {
            paired_devices: Vec::new(),
        };
        (model, Self::refresh_paired_devices(session))
    }

    pub fn refresh_paired_devices(session: Arc<OpenSCQ30Session>) -> Task<Message> {
        Task::future(async move {
            Ok(Message::SetPairedDevices(
                session
                    .paired_devices()
                    .await
                    .map_err(handle_soft_error!())?,
            ))
        })
        .map(coalesce_result)
    }

    pub fn view(&self) -> Element<Message> {
        widget::scrollable(
            widget::column()
                .push(
                    widget::row()
                        .align_y(alignment::Vertical::Center)
                        .push(widget::text::title2(fl!("select-device")).width(Length::Fill))
                        .push(
                            widget::button::standard(fl!("add-device"))
                                .font_size(16)
                                .height(32)
                                .on_press(Message::AddDevice),
                        ),
                )
                .extend(self.items())
                .padding(10)
                .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn items(&self) -> impl Iterator<Item = Element<'_, Message>> {
        self.paired_devices
            .iter()
            .enumerate()
            .map(move |(index, device)| {
                widget::row()
                    .align_y(alignment::Vertical::Center)
                    .push(
                        widget::column()
                            .push(
                                widget::text::heading(&device.name)
                                    .size(16)
                                    .line_height(LineHeight::Absolute(Pixels(24.0))),
                            )
                            .push(
                                widget::text::body(device.mac_address.to_string())
                                    .size(16)
                                    .line_height(LineHeight::Absolute(Pixels(24.0))),
                            )
                            .push(
                                widget::text::body(device.model.to_string())
                                    .size(16)
                                    .line_height(LineHeight::Absolute(Pixels(24.0))),
                            )
                            .width(Length::Fill),
                    )
                    .push(
                        widget::button::destructive(fl!("remove"))
                            .font_size(16)
                            .height(32)
                            .on_press(Message::RemoveDevice(index)),
                    )
                    .push(widget::horizontal_space().width(Length::Fixed(6f32)))
                    .push(
                        widget::button::suggested(fl!("connect"))
                            .font_size(16)
                            .height(32)
                            .on_press(Message::ConnectToDevice(index)),
                    )
                    .apply(widget::container)
                    .width(Length::Fill)
                    .padding(16)
                    .class(cosmic::style::Container::List)
                    .into()
            })
    }

    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ConnectToDevice(index) => {
                return Action::ConnectToDevice(self.paired_devices[index].clone())
            }
            Message::RemoveDevice(index) => {
                return Action::RemoveDevice(self.paired_devices[index].clone())
            }
            Message::AddDevice => return Action::AddDevice,
            Message::SetPairedDevices(paired_devices) => self.paired_devices = paired_devices,
            Message::Warning(message) => return Action::Warning(message),
        }
        Action::None
    }
}
