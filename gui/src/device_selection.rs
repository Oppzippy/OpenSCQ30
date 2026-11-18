use std::{collections::HashSet, iter, sync::Arc};

use cosmic::{
    Apply, Element, Task,
    iced::{Length, alignment},
    widget,
};
use macaddr::MacAddr6;
use openscq30_i18n::Translate;
use openscq30_lib::{OpenSCQ30Session, storage::PairedDevice};

use crate::{fl, handle_soft_error, utils::coalesce_result};

pub struct DeviceSelectionModel {
    paired_devices: Vec<PairedDevice>,
    /// Mac addresses of devices that are currently connected to the bluetooth adapter. None until the first
    /// successful refresh.
    connected_devices: Option<HashSet<MacAddr6>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConnectToDevice(usize),
    RemoveDevice(usize),
    AddDevice,
    SetPairedDevices(Vec<PairedDevice>),
    SetConnectedDevices(Option<HashSet<MacAddr6>>),
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
        let model = Self {
            paired_devices: Vec::new(),
            connected_devices: None,
        };
        (
            model,
            Task::batch([
                Self::refresh_paired_devices(session.clone()),
                Self::refresh_connected_devices(session),
            ]),
        )
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

    pub fn refresh_connected_devices(session: Arc<OpenSCQ30Session>) -> Task<Message> {
        Task::future(async move {
            // This runs periodically, so failures (such as bluetooth being turned off) are not surfaced to the user.
            // The connection status is just hidden instead.
            match session.list_connected_devices().await {
                Ok(connected_devices) => Message::SetConnectedDevices(Some(
                    connected_devices
                        .into_iter()
                        .map(|descriptor| descriptor.mac_address)
                        .collect(),
                )),
                Err(err) => {
                    tracing::debug!("failed to list connected devices: {err:?}");
                    Message::SetConnectedDevices(None)
                }
            }
        })
    }

    pub fn view(&self) -> Element<'_, Message> {
        widget::scrollable(
            widget::column(
                iter::once(
                    widget::row![
                        widget::text::title2(fl!("select-device")).width(Length::Fill),
                        widget::button::standard(fl!("add-device")).on_press(Message::AddDevice),
                    ]
                    .align_y(alignment::Vertical::Center)
                    .into(),
                )
                .chain(self.items()),
            )
            .padding([0, 10])
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
                widget::row![
                    widget::column![
                        widget::text::heading(device.model.translate()),
                        widget::text::body(device.model.to_string()),
                        widget::text::body(device.mac_address.to_string()),
                    ]
                    .push_maybe(
                        device
                            .is_demo
                            .then(|| fl!("demo-mode").to_uppercase())
                            .map(widget::text::body),
                    )
                    .width(Length::Fill),
                    self.connection_status(device),
                    widget::space().width(Length::Fixed(16f32)),
                    widget::button::destructive(fl!("remove"))
                        .on_press(Message::RemoveDevice(index)),
                    widget::space().width(Length::Fixed(6f32)),
                    widget::button::suggested(fl!("connect"))
                        .on_press(Message::ConnectToDevice(index)),
                ]
                .align_y(alignment::Vertical::Center)
                .apply(widget::container)
                .width(Length::Fill)
                .padding(16)
                .class(cosmic::style::Container::List)
                .into()
            })
    }

    fn connection_status(&self, device: &PairedDevice) -> Element<'_, Message> {
        let Some(connected_devices) = &self.connected_devices else {
            return widget::space().into();
        };
        if device.is_demo {
            return widget::space().into();
        }
        if connected_devices.contains(&device.mac_address) {
            widget::text::body(fl!("bluetooth-connected"))
                .class(cosmic::theme::Text::Accent)
                .into()
        } else {
            widget::text::body(fl!("bluetooth-not-connected")).into()
        }
    }

    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ConnectToDevice(index) => {
                return Action::ConnectToDevice(self.paired_devices[index]);
            }
            Message::RemoveDevice(index) => {
                return Action::RemoveDevice(self.paired_devices[index]);
            }
            Message::AddDevice => return Action::AddDevice,
            Message::SetPairedDevices(paired_devices) => self.paired_devices = paired_devices,
            Message::SetConnectedDevices(connected_devices) => {
                self.connected_devices = connected_devices;
            }
            Message::Warning(message) => return Action::Warning(message),
        }
        Action::None
    }
}
