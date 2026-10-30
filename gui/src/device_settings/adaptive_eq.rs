use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use cosmic::{
    Element, Task,
    iced::{Length, alignment, core::text::Wrapping},
    widget,
};
use openscq30_lib::adaptive_eq::{
    self, AdaptiveEqConfig, AdaptiveHistoryEntry, AdaptiveRule, GenreCategoryMapping,
    MediaTrackInfo, ResolutionResult, RuleMatchType, default_genre_mappings,
    device_preset_profiles,
};

use crate::{app::DebugOpenSCQ30Device, fl};

fn localize_match_type(match_type: RuleMatchType) -> String {
    match match_type {
        RuleMatchType::Artist => fl!("adaptive-eq-match-artist"),
        RuleMatchType::Keyword => fl!("adaptive-eq-match-keyword"),
        RuleMatchType::Genre => fl!("adaptive-eq-match-genre"),
    }
}

fn localize_genre_category(mapping: &GenreCategoryMapping) -> String {
    match mapping.category_id.as_str() {
        "rock_metal" => fl!("adaptive-eq-cat-rock-metal"),
        "hiphop_rap" => fl!("adaptive-eq-cat-hiphop-rap"),
        "pop" => fl!("adaptive-eq-cat-pop"),
        "electronic" => fl!("adaptive-eq-cat-electronic"),
        "dance_house" => fl!("adaptive-eq-cat-dance-house"),
        "bass_phonk" => fl!("adaptive-eq-cat-bass-phonk"),
        "rnb_soul" => fl!("adaptive-eq-cat-rnb-soul"),
        "classical_score" => fl!("adaptive-eq-cat-classical-score"),
        "jazz_blues" => fl!("adaptive-eq-cat-jazz-blues"),
        "acoustic_folk" => fl!("adaptive-eq-cat-acoustic-folk"),
        "latin" => fl!("adaptive-eq-cat-latin"),
        "piano" => fl!("adaptive-eq-cat-piano"),
        "lounge_lofi" => fl!("adaptive-eq-cat-lounge-lofi"),
        "deep" => fl!("adaptive-eq-cat-deep"),
        "podcast" => fl!("adaptive-eq-cat-podcast"),
        "spoken_word" => fl!("adaptive-eq-cat-spoken-word"),
        _ => mapping.display_name.clone(),
    }
}

fn localize_detected_genre(genre: &str) -> String {
    match genre {
        "Unknown" => fl!("adaptive-eq-genre-unknown"),
        "Keyword" => fl!("adaptive-eq-genre-keyword"),
        _ => genre.to_string(),
    }
}

fn localize_detection_source(source: &str) -> String {
    match source {
        "Default Profile" => fl!("adaptive-eq-source-default"),
        "Custom Rule" => fl!("adaptive-eq-source-custom-rule"),
        "Custom Rule (Genre)" => fl!("adaptive-eq-source-custom-rule-genre"),
        "Title Analysis" => fl!("adaptive-eq-source-title-analysis"),
        "Channel Analysis" => fl!("adaptive-eq-source-channel-analysis"),
        "Title/Artist Analysis" => fl!("adaptive-eq-source-title-artist-analysis"),
        "iTunes API (Track)" => fl!("adaptive-eq-source-itunes-track"),
        "iTunes API (Artist)" => fl!("adaptive-eq-source-itunes-artist"),
        _ => source.to_string(),
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PollTick,
    ForceRefreshNow,
    MediaStateUpdated {
        generation: u64,
        track: Option<MediaTrackInfo>,
        resolution: Option<(ResolutionResult, HashMap<String, Option<(String, String)>>)>,
        applied: bool,
        error: Option<String>,
    },
    ToggleEnabled(bool),
    ToggleYtOnly(bool),
    ToggleOnlineLookup(bool),
    ToggleOnlyPlaying(bool),
    SetDefaultProfile(usize),
    SetNewRuleMatchType(usize),
    SetNewRulePattern(String),
    SetNewRuleProfile(usize),
    AddCustomRule,
    RemoveCustomRule(usize),
    SetGenreMappingProfile(usize, usize),
    ResetGenreMappings,
}

pub enum Action {
    None,
    Task(Task<Message>),
    RefreshDeviceSettings,
    Warning(String),
}

pub struct AdaptiveEqModel {
    device: DebugOpenSCQ30Device,
    config_dir: PathBuf,
    config: AdaptiveEqConfig,
    profile_ids: Vec<String>,
    localized_profiles: Vec<String>,
    match_type_labels: Vec<String>,
    new_rule_match_type_idx: usize,
    new_rule_pattern: String,
    new_rule_profile_idx: usize,
    current_track: Option<MediaTrackInfo>,
    current_resolution: Option<ResolutionResult>,
    last_track_key: Option<String>,
    online_cache: HashMap<String, Option<(String, String)>>,
    history: VecDeque<AdaptiveHistoryEntry>,
    poll_in_flight: bool,
    poll_generation: u64,
    active_generation: Arc<AtomicU64>,
    enabled_flag: Arc<AtomicBool>,
}

impl AdaptiveEqModel {
    pub fn new(device: DebugOpenSCQ30Device, config_dir: PathBuf) -> (Self, Task<Message>) {
        let config = AdaptiveEqConfig::load_or_default(&config_dir);
        let (profile_ids, localized_profiles) = device_preset_profiles(device.as_ref());
        let match_type_labels = RuleMatchType::ALL
            .iter()
            .map(|t| localize_match_type(*t))
            .collect();

        let default_rock_idx = profile_ids.iter().position(|p| p == "Rock").unwrap_or(0);
        let enabled_flag = Arc::new(AtomicBool::new(config.enabled));
        let active_generation = Arc::new(AtomicU64::new(0));

        let mut model = Self {
            device,
            config_dir,
            config,
            profile_ids,
            localized_profiles,
            match_type_labels,
            new_rule_match_type_idx: 0,
            new_rule_pattern: String::new(),
            new_rule_profile_idx: default_rock_idx,
            current_track: None,
            current_resolution: None,
            last_track_key: None,
            online_cache: HashMap::new(),
            history: VecDeque::with_capacity(12),
            poll_in_flight: false,
            poll_generation: 0,
            active_generation,
            enabled_flag,
        };

        let initial_task = if model.config.enabled {
            model.trigger_poll(true)
        } else {
            Task::none()
        };
        (model, initial_task)
    }

    pub fn refresh_device_profiles(&mut self) {
        let (profile_ids, localized_profiles) = device_preset_profiles(self.device.as_ref());
        self.profile_ids = profile_ids;
        self.localized_profiles = localized_profiles;
        self.match_type_labels = RuleMatchType::ALL
            .iter()
            .map(|t| localize_match_type(*t))
            .collect();
    }

    fn localize_profile(&self, profile_id: &str) -> String {
        self.profile_ids
            .iter()
            .position(|id| id.eq_ignore_ascii_case(profile_id))
            .and_then(|idx| self.localized_profiles.get(idx))
            .cloned()
            .unwrap_or_else(|| profile_id.to_string())
    }

    fn save_config(&self) {
        if let Err(err) = self.config.save(&self.config_dir) {
            tracing::warn!("Failed to save adaptive_eq.json: {err:?}");
        }
    }

    fn invalidate_in_flight(&mut self) {
        self.poll_generation = self.poll_generation.wrapping_add(1);
        self.active_generation
            .store(self.poll_generation, Ordering::SeqCst);
        self.poll_in_flight = false;
    }

    fn trigger_poll(&mut self, force: bool) -> Task<Message> {
        if !self.config.enabled && !force {
            return Task::none();
        }
        if self.poll_in_flight && !force {
            return Task::none();
        }
        self.poll_generation = self.poll_generation.wrapping_add(1);
        let generation = self.poll_generation;
        self.active_generation.store(generation, Ordering::SeqCst);
        self.poll_in_flight = true;

        let device = self.device.0.clone();
        let config = self.config.clone();
        let active_generation = self.active_generation.clone();
        let enabled_flag = self.enabled_flag.clone();
        let last_track_key = if force {
            None
        } else {
            self.last_track_key.clone()
        };
        let mut cache = self.online_cache.clone();

        Task::future(async move {
            let track = adaptive_eq::select_best_media_track(
                config.yt_music_or_browser_only,
                config.only_when_playing,
            )
            .await;

            let Some(track) = track else {
                return Message::MediaStateUpdated {
                    generation,
                    track: None,
                    resolution: None,
                    applied: false,
                    error: None,
                };
            };

            let key = track.track_key();
            if last_track_key.as_deref() == Some(key.as_str()) {
                return Message::MediaStateUpdated {
                    generation,
                    track: Some(track),
                    resolution: None,
                    applied: false,
                    error: None,
                };
            }

            let resolution =
                adaptive_eq::resolve_profile_for_track(&track, &config, &mut cache).await;

            let mut applied = false;
            let mut error = None;
            let still_current_and_enabled = config.enabled
                && enabled_flag.load(Ordering::SeqCst)
                && active_generation.load(Ordering::SeqCst) == generation;

            if still_current_and_enabled {
                match adaptive_eq::apply_preset_profile(device.as_ref(), &resolution.target_profile)
                    .await
                {
                    Ok(changed) => applied = changed,
                    Err(err) => error = Some(format!("{err:#}")),
                }
            }

            Message::MediaStateUpdated {
                generation,
                track: Some(track),
                resolution: Some((resolution, cache)),
                applied,
                error,
            }
        })
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::PollTick => Action::Task(self.trigger_poll(false)),
            Message::ForceRefreshNow => Action::Task(self.trigger_poll(true)),
            Message::MediaStateUpdated {
                generation,
                track,
                resolution,
                applied,
                error,
            } => {
                if let Some((_, ref updated_cache)) = resolution {
                    self.online_cache.extend(updated_cache.clone());
                }
                if generation != self.poll_generation {
                    return Action::None;
                }

                self.poll_in_flight = false;
                if track.is_none() {
                    self.last_track_key = None;
                }
                self.current_track = track;

                if let Some((res, _)) = resolution {
                    if error.is_none()
                        && let Some(t) = &self.current_track
                    {
                        self.last_track_key = Some(t.track_key());
                        let display = t.display_title();
                        let should_push = self.history.front().is_none_or(|front| {
                            front.track_display != display
                                || front.applied_profile != res.target_profile
                        });
                        if should_push {
                            if self.history.len() >= 12 {
                                self.history.pop_back();
                            }
                            self.history.push_front(AdaptiveHistoryEntry {
                                track_display: display,
                                detected_genre: res.detected_genre.clone(),
                                detection_source: res.detection_source.clone(),
                                applied_profile: res.target_profile.clone(),
                            });
                        }
                    } else if error.is_some() {
                        // Allow retry on the next poll tick if Bluetooth profile application failed
                        self.last_track_key = None;
                    }
                    self.current_resolution = Some(res);
                }

                if let Some(err) = error {
                    Action::Warning(err)
                } else if applied {
                    Action::RefreshDeviceSettings
                } else {
                    Action::None
                }
            }
            Message::ToggleEnabled(enabled) => {
                self.config.enabled = enabled;
                self.enabled_flag.store(enabled, Ordering::SeqCst);
                self.save_config();
                if enabled {
                    Action::Task(self.trigger_poll(true))
                } else {
                    self.invalidate_in_flight();
                    Action::None
                }
            }
            Message::ToggleYtOnly(val) => {
                self.config.yt_music_or_browser_only = val;
                self.save_config();
                Action::Task(self.trigger_poll(true))
            }
            Message::ToggleOnlineLookup(val) => {
                self.config.online_lookup_enabled = val;
                self.save_config();
                Action::Task(self.trigger_poll(true))
            }
            Message::ToggleOnlyPlaying(val) => {
                self.config.only_when_playing = val;
                self.save_config();
                Action::Task(self.trigger_poll(true))
            }
            Message::SetDefaultProfile(idx) => {
                if let Some(profile_id) = self.profile_ids.get(idx) {
                    self.config.default_profile = profile_id.clone();
                    self.save_config();
                    return Action::Task(self.trigger_poll(true));
                }
                Action::None
            }
            Message::SetNewRuleMatchType(idx) => {
                if idx < RuleMatchType::ALL.len() {
                    self.new_rule_match_type_idx = idx;
                }
                Action::None
            }
            Message::SetNewRulePattern(pattern) => {
                self.new_rule_pattern = pattern;
                Action::None
            }
            Message::SetNewRuleProfile(idx) => {
                if idx < self.profile_ids.len() {
                    self.new_rule_profile_idx = idx;
                }
                Action::None
            }
            Message::AddCustomRule => {
                let pattern = self.new_rule_pattern.trim().to_string();
                if pattern.is_empty() {
                    return Action::None;
                }
                let match_type = RuleMatchType::ALL
                    .get(self.new_rule_match_type_idx)
                    .copied()
                    .unwrap_or(RuleMatchType::Artist);
                let target_profile = self
                    .profile_ids
                    .get(self.new_rule_profile_idx)
                    .cloned()
                    .unwrap_or_else(|| "Rock".to_string());

                self.config.custom_rules.push(AdaptiveRule {
                    match_type,
                    pattern,
                    target_profile,
                });
                self.new_rule_pattern.clear();
                self.save_config();
                Action::Task(self.trigger_poll(true))
            }
            Message::RemoveCustomRule(idx) => {
                if idx < self.config.custom_rules.len() {
                    self.config.custom_rules.remove(idx);
                    self.save_config();
                    return Action::Task(self.trigger_poll(true));
                }
                Action::None
            }
            Message::SetGenreMappingProfile(mapping_idx, profile_idx) => {
                if let Some(mapping) = self.config.genre_mappings.get_mut(mapping_idx)
                    && let Some(profile_id) = self.profile_ids.get(profile_idx)
                {
                    mapping.target_profile = profile_id.clone();
                    self.save_config();
                    return Action::Task(self.trigger_poll(true));
                }
                Action::None
            }
            Message::ResetGenreMappings => {
                self.config.genre_mappings = default_genre_mappings();
                self.save_config();
                Action::Task(self.trigger_poll(true))
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let default_profile_idx = self
            .profile_ids
            .iter()
            .position(|p| p.eq_ignore_ascii_case(&self.config.default_profile));

        let player_text = match &self.current_track {
            Some(t) => format!("{} ({})", t.player_short_name, t.playback_status),
            None => fl!("adaptive-eq-waiting"),
        };

        let now_playing_text = match &self.current_track {
            Some(t) => t.display_title(),
            None => "-".to_string(),
        };

        let genre_text = match &self.current_resolution {
            Some(res) => format!(
                "{} ({})",
                localize_detected_genre(&res.detected_genre),
                localize_detection_source(&res.detection_source)
            ),
            None => "-".to_string(),
        };

        let applied_profile_text = match &self.current_resolution {
            Some(res) => self.localize_profile(&res.target_profile),
            None => "-".to_string(),
        };

        let status_section = widget::settings::section()
            .title(fl!("adaptive-eq-live-status"))
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-enabled"))
                    .toggler(self.config.enabled, Message::ToggleEnabled),
            )
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-yt-only"))
                    .toggler(self.config.yt_music_or_browser_only, Message::ToggleYtOnly),
            )
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-online-lookup")).toggler(
                    self.config.online_lookup_enabled,
                    Message::ToggleOnlineLookup,
                ),
            )
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-only-playing"))
                    .toggler(self.config.only_when_playing, Message::ToggleOnlyPlaying),
            )
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-default-profile")).flex_control(
                    widget::dropdown(
                        &self.localized_profiles,
                        default_profile_idx,
                        Message::SetDefaultProfile,
                    )
                    .width(Length::Fill),
                ),
            )
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-player"))
                    .flex_control(widget::text(player_text).wrapping(Wrapping::WordOrGlyph)),
            )
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-now-playing"))
                    .flex_control(widget::text(now_playing_text).wrapping(Wrapping::WordOrGlyph)),
            )
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-detected-genre"))
                    .flex_control(widget::text(genre_text).wrapping(Wrapping::WordOrGlyph)),
            )
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-applied-profile")).flex_control(
                    widget::text(applied_profile_text).wrapping(Wrapping::WordOrGlyph),
                ),
            )
            .add(
                widget::settings::item::builder(fl!("refresh")).flex_control(
                    widget::button::suggested(fl!("adaptive-eq-refresh-now"))
                        .on_press(Message::ForceRefreshNow),
                ),
            );

        // Section 2: Custom Rules
        let mut custom_rules_section = widget::settings::section()
            .title(fl!("adaptive-eq-custom-rules"))
            .add(
                widget::settings::item::builder(fl!("adaptive-eq-add-rule")).flex_control(
                    widget::row![
                        widget::dropdown(
                            &self.match_type_labels,
                            Some(self.new_rule_match_type_idx),
                            Message::SetNewRuleMatchType,
                        ),
                        widget::text_input(
                            fl!("adaptive-eq-rule-pattern-placeholder"),
                            &self.new_rule_pattern,
                        )
                        .on_input(Message::SetNewRulePattern)
                        .on_submit(|_| Message::AddCustomRule)
                        .width(Length::Fill),
                        widget::dropdown(
                            &self.localized_profiles,
                            Some(self.new_rule_profile_idx),
                            Message::SetNewRuleProfile,
                        ),
                        widget::button::suggested(fl!("create")).on_press(Message::AddCustomRule),
                    ]
                    .spacing(8)
                    .align_y(alignment::Vertical::Center),
                ),
            );

        for (idx, rule) in self.config.custom_rules.iter().enumerate() {
            let label = format!(
                "[{}] {}",
                localize_match_type(rule.match_type),
                rule.pattern
            );
            let target_loc = self.localize_profile(&rule.target_profile);
            custom_rules_section = custom_rules_section.add(
                widget::settings::item::builder(label).flex_control(
                    widget::row![
                        widget::text(target_loc),
                        widget::button::destructive(fl!("delete"))
                            .on_press(Message::RemoveCustomRule(idx)),
                    ]
                    .spacing(8)
                    .align_y(alignment::Vertical::Center),
                ),
            );
        }

        // Section 3: Genre Mappings
        let mut mappings_section =
            widget::settings::section().title(fl!("adaptive-eq-genre-mappings"));
        for (idx, mapping) in self.config.genre_mappings.iter().enumerate() {
            let selected_idx = self
                .profile_ids
                .iter()
                .position(|p| p.eq_ignore_ascii_case(&mapping.target_profile));
            let category_label = localize_genre_category(mapping);
            mappings_section = mappings_section.add(
                widget::settings::item::builder(category_label).flex_control(
                    widget::dropdown(&self.localized_profiles, selected_idx, move |profile_idx| {
                        Message::SetGenreMappingProfile(idx, profile_idx)
                    })
                    .width(Length::Fill),
                ),
            );
        }
        mappings_section = mappings_section.add(
            widget::settings::item::builder(fl!("default")).flex_control(
                widget::button::standard(fl!("adaptive-eq-reset-mappings"))
                    .on_press(Message::ResetGenreMappings),
            ),
        );

        let mut content = widget::column![status_section, custom_rules_section, mappings_section]
            .spacing(16)
            .width(Length::Fill);

        // Section 4: Recent History
        if !self.history.is_empty() {
            let mut history_section = widget::settings::section().title(fl!("adaptive-eq-history"));
            for entry in &self.history {
                let right_summary = format!(
                    "{} ({}) → {}",
                    localize_detected_genre(&entry.detected_genre),
                    localize_detection_source(&entry.detection_source),
                    self.localize_profile(&entry.applied_profile)
                );
                history_section = history_section.add(
                    widget::settings::item::builder(entry.track_display.as_str())
                        .flex_control(widget::text(right_summary).wrapping(Wrapping::WordOrGlyph)),
                );
            }
            content = content.push(history_section);
        }

        widget::scrollable(content).into()
    }
}
