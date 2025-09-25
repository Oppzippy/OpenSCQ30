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
        let mut profiles = profiles
            .into_iter()
            .map(|(name, profile)| LegacyProfileInfo {
                name: name.to_owned(),
                values: profile.volume_offsets.to_owned(),
                visualization: EqualizerLine::new(-120, 135, profile.volume_offsets),
            })
            .collect::<Vec<_>>();
        profiles.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        Self { profiles }
    }

    pub fn view(&self) -> Element<'_, Message> {
        widget::scrollable(widget::column().padding(10).spacing(10).extend(
            self.profiles.iter().map(|profile| {
                widget::row()
                    .padding(10)
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
                            .push(widget::vertical_space().height(4))
                            .push(
                                widget::responsive(|size| {
                                    profile
                                        .visualization
                                        .view()
                                        .apply(widget::container)
                                        .width(size.width)
                                        .height(40)
                                        .into()
                                })
                                .apply(widget::container)
                                // responsive wants to fill all available height, but that is not desirable
                                // it will even crash due to being a child of scrollable, so constrain its height
                                // by wrapping in a container
                                .height(40),
                            ),
                    )
                    .push(
                        widget::button::standard(fl!("migrate")).on_press(Message::Migrate(
                            profile.name.to_owned(),
                            profile.values.to_owned(),
                        )),
                    )
                    .apply(widget::container)
                    .class(cosmic::style::Container::Card)
                    .into()
            }),
        ))
        .into()
    }
}
