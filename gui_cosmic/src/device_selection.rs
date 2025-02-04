use cosmic::{
    iced::{alignment, Length, Pixels},
    iced_core::text::LineHeight,
    widget, Apply, Element,
};
use macaddr::MacAddr6;
use openscq30_storage::{OpenSCQ30Database, PairedDevice};

use crate::fl;

pub struct DeviceSelectionModel {
    database: OpenSCQ30Database,
    paired_devices: Vec<PairedDevice>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConnectToDevice(usize),
    RemoveDevice(usize),
    AddDevice,
}

pub enum Action {
    ConnectToDevice(PairedDevice),
    RemoveDevice(MacAddr6),
    AddDevice,
}

impl DeviceSelectionModel {
    pub fn new(database: OpenSCQ30Database) -> Self {
        let mut model = DeviceSelectionModel {
            database,
            paired_devices: Vec::new(),
        };
        model.refresh_paired_devices();
        model
    }

    pub fn refresh_paired_devices(&mut self) {
        self.paired_devices = self
            .database
            .fetch_paired_devices()
            .expect("TODO error handling");
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
                Action::ConnectToDevice(self.paired_devices[index].clone())
            }
            Message::RemoveDevice(index) => {
                Action::RemoveDevice(self.paired_devices[index].mac_address)
            }
            Message::AddDevice => Action::AddDevice,
        }
    }
}
