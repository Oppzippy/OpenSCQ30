use std::collections::HashMap;

use cosmic::{
    Apply, Element,
    iced::alignment::{Horizontal, Vertical},
    widget::{self},
};

use crate::{equalizer_line::EqualizerLine, fl, openscq30_v1_migration::LegacyEqualizerProfile};

struct LegacyProfileInfo {
    name: String,
    values: Vec<i16>,
    visualization: EqualizerLine<Message>,
}
pub struct LegacyMigrationModel {
    profiles: Vec<LegacyProfileInfo>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Migrate(String, Vec<i16>),
}

impl LegacyMigrationModel {
    pub fn new(profiles: HashMap<String, LegacyEqualizerProfile>) -> Self {
        let profiles = profiles
            .into_iter()
            .map(|(name, profile)| LegacyProfileInfo {
                name: name.to_owned(),
                values: profile.volume_offsets.to_owned(),
                visualization: EqualizerLine::new(-120, 135, profile.volume_offsets),
            })
            .collect::<Vec<_>>();

        Self { profiles }
    }

    pub fn view(&self) -> Element<'_, Message> {
        widget::scrollable(
            widget::column()
                .padding(10)
                .extend(self.profiles.iter().map(|profile| {
                    widget::row()
                        .spacing(20)
                        .align_y(Vertical::Center)
                        .push(
                            widget::column()
                                .align_x(Horizontal::Center)
                                .push(widget::text::heading(&profile.name))
                                .push(widget::text(format!(
                                    "{:?}",
                                    profile
                                        .values
                                        .iter()
                                        .map(|v| *v as f32 / 10f32)
                                        .collect::<Vec<_>>()
                                )))
                                .push(
                                    profile
                                        .visualization
                                        .view()
                                        .apply(widget::container)
                                        .width(400)
                                        .height(40),
                                ),
                        )
                        .push(
                            widget::button::standard(fl!("migrate")).on_press(Message::Migrate(
                                profile.name.to_owned(),
                                profile.values.to_owned(),
                            )),
                        )
                        .into()
                })),
        )
        .into()
    }
}
