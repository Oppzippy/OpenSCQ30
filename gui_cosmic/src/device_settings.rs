use cosmic::{
    iced::{alignment, Length},
    widget::{self, nav_bar},
    Element, Task,
};
use openscq30_lib::api::settings::{CategoryId, Setting, SettingId, Value};

use crate::{app::DebugOpenSCQ30Device, fl, handle_soft_error, utils::coalesce_result};

#[derive(Debug, Clone)]
pub enum Message {
    SetSetting(SettingId<'static>, Value),
    SetEqualizerBand(SettingId<'static>, u8, i16),
    Refresh,
    Warning(String),
}
pub enum Action {
    Task(Task<Message>),
    Warning(String),
    None,
}

pub struct DeviceSettingsModel {
    device: DebugOpenSCQ30Device,
    nav_model: nav_bar::Model,
    settings: Vec<(SettingId<'static>, Setting)>,
}

impl DeviceSettingsModel {
    pub fn new(device: DebugOpenSCQ30Device) -> Self {
        let mut nav_model = nav_bar::Model::default();
        for category in device.categories() {
            nav_model
                .insert()
                .text(category.0)
                .data(category)
                .activate();
        }
        nav_model.activate_position(0);
        let mut model = Self {
            device,
            nav_model,
            settings: Vec::new(),
        };
        model.refresh();
        model
    }

    pub fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Message> {
        self.nav_model.activate(id);
        self.refresh();
        Task::none()
    }

    fn refresh(&mut self) {
        let Some(category_id) = self.nav_model.active_data::<CategoryId<'static>>() else {
            return;
        };
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

    pub fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    pub fn view(&self) -> Element<'_, Message> {
        let Some(category_id) = self.nav_model.active_data::<CategoryId<'static>>() else {
            return widget::row().into();
        };
        widget::column()
            .push(
                widget::text::title2(category_id.0)
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center),
            )
            .extend(self.settings.iter().map(|(setting_id, setting)| {
                let setting_id = setting_id.to_owned();
                match setting {
                    Setting::Toggle { value } => Element::from(
                        widget::toggler(*value)
                            .label(setting_id.0)
                            .width(Length::FillPortion(1))
                            .on_toggle(move |new_value| {
                                Message::SetSetting(setting_id, new_value.into())
                            }),
                    ),
                    Setting::I32Range { setting, value } => todo!(),
                    Setting::Select { setting, value } => with_label(
                        setting_id.0,
                        widget::dropdown(&setting.options, value.map(usize::from), move |index| {
                            Message::SetSetting(setting_id, (index as u16).into())
                        })
                        .width(Length::FillPortion(1)),
                    ),
                    Setting::OptionalSelect { setting, value } => with_label(
                        setting_id.0,
                        widget::dropdown(&setting.options, value.map(usize::from), move |index| {
                            Message::SetSetting(setting_id, Some(index as u16).into())
                        })
                        .width(Length::FillPortion(1)),
                    ),
                    Setting::MultiSelect { setting, value } => todo!(),
                    Setting::Equalizer {
                        setting,
                        values: value,
                    } => widget::responsive(move |size| {
                        if size.width < 700f32 || size.height < 300f32 {
                            widget::scrollable(widget::column().extend(
                                setting.band_hz.iter().cloned().enumerate().map(|(i, hz)| {
                                    widget::row()
                                        .width(Length::Fill)
                                        .align_y(alignment::Vertical::Center)
                                        .spacing(8)
                                        .push(widget::text::text(fl!("hz", hz = hz)))
                                        .push(widget::spin_button(
                                            {
                                                let divisor =
                                                    10i16.pow(setting.fraction_digits as u32);
                                                let db_integer_portion =
                                                    value[i as usize] / divisor;
                                                let db_decimal_portion =
                                                    (value[i as usize] % divisor).abs();
                                                let decimal_db = format!(
                                                    "{db_integer_portion}.{db_decimal_portion}"
                                                );
                                                fl!("db", db = decimal_db)
                                            },
                                            value[i as usize],
                                            1,
                                            setting.min,
                                            setting.max,
                                            move |band_value| {
                                                Message::SetEqualizerBand(
                                                    setting_id, i as u8, band_value,
                                                )
                                            },
                                        ))
                                        .push(widget::slider(
                                            setting.min..=setting.max,
                                            value
                                                .get(i as usize)
                                                .cloned()
                                                .unwrap_or((setting.min + setting.max) / 2),
                                            move |band_value| {
                                                Message::SetEqualizerBand(
                                                    setting_id, i as u8, band_value,
                                                )
                                            },
                                        ))
                                        .into()
                                }),
                            ))
                            .into()
                        } else {
                            widget::row()
                                .extend(setting.band_hz.iter().cloned().enumerate().map(
                                    |(i, hz)| {
                                        widget::column()
                                            .width(Length::Fill)
                                            .align_x(alignment::Horizontal::Center)
                                            .spacing(8)
                                            .push(widget::vertical_slider(
                                                setting.min..=setting.max,
                                                value
                                                    .get(i as usize)
                                                    .cloned()
                                                    .unwrap_or((setting.min + setting.max) / 2),
                                                move |band_value| {
                                                    Message::SetEqualizerBand(
                                                        setting_id, i as u8, band_value,
                                                    )
                                                },
                                            ))
                                            .push(widget::vertical_spin_button(
                                                {
                                                    let divisor =
                                                        10i16.pow(setting.fraction_digits as u32);
                                                    let db_integer_portion =
                                                        value[i as usize] / divisor;
                                                    let db_decimal_portion =
                                                        (value[i as usize] % divisor).abs();
                                                    let decimal_db = format!(
                                                        "{db_integer_portion}.{db_decimal_portion}"
                                                    );
                                                    fl!("db", db = decimal_db)
                                                },
                                                value[i as usize],
                                                1,
                                                setting.min,
                                                setting.max,
                                                move |band_value| {
                                                    Message::SetEqualizerBand(
                                                        setting_id, i as u8, band_value,
                                                    )
                                                },
                                            ))
                                            .push(widget::text::text(fl!("hz", hz = hz)))
                                            .into()
                                    },
                                ))
                                .into()
                        }
                    })
                    .into(),
                }
                .into()
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
                if let Some(Setting::Equalizer { setting: _, values }) =
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
            Message::Refresh => {
                self.refresh();
                Action::None
            }
            Message::Warning(message) => Action::Warning(message),
        }
    }
}

fn with_label<'a>(
    label: &'a str,
    element: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    widget::row()
        .align_y(alignment::Vertical::Center)
        .push(widget::text::text(label).width(Length::FillPortion(1)))
        .push(element.into())
        .into()
}
