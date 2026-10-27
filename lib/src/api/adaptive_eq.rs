use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::api::{
    device::{self, OpenSCQ30Device},
    settings::{Setting, SettingId, Value},
};

/// All 22 standard built-in Soundcore equalizer preset profile identifiers.
pub const STANDARD_PRESET_PROFILES: &[&str] = &[
    "SoundcoreSignature",
    "Acoustic",
    "BassBooster",
    "BassReducer",
    "Classical",
    "Podcast",
    "Dance",
    "Deep",
    "Electronic",
    "Flat",
    "HipHop",
    "Jazz",
    "Latin",
    "Lounge",
    "Piano",
    "Pop",
    "RnB",
    "Rock",
    "SmallSpeakers",
    "SpokenWord",
    "TrebleBooster",
    "TrebleReducer",
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTrackInfo {
    pub player_bus_name: String,
    pub player_short_name: String,
    pub playback_status: String,
    pub raw_title: String,
    pub raw_artist: String,
    pub raw_album: String,
    pub clean_artist: String,
    pub clean_title: String,
    pub mpris_genres: Vec<String>,
    pub url: String,
    pub is_browser_or_yt_music: bool,
}

impl MediaTrackInfo {
    pub fn track_key(&self) -> String {
        format!(
            "{}::{}::{}",
            self.player_short_name,
            self.raw_artist.trim(),
            self.raw_title.trim()
        )
    }

    pub fn display_title(&self) -> String {
        if !self.clean_artist.is_empty() && !self.clean_title.is_empty() {
            format!("{} - {}", self.clean_artist, self.clean_title)
        } else if !self.raw_artist.is_empty() {
            format!("{} - {}", self.raw_artist, self.raw_title)
        } else {
            self.raw_title.clone()
        }
    }

    pub fn is_playing(&self) -> bool {
        self.playback_status.eq_ignore_ascii_case("Playing")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RuleMatchType {
    Artist,
    Keyword,
    Genre,
}

impl RuleMatchType {
    pub const ALL: &[Self] = &[Self::Artist, Self::Keyword, Self::Genre];

    pub fn label(self) -> &'static str {
        match self {
            Self::Artist => "Artist",
            Self::Keyword => "Title / Keyword",
            Self::Genre => "Genre",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveRule {
    pub match_type: RuleMatchType,
    pub pattern: String,
    pub target_profile: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreCategoryMapping {
    pub category_id: String,
    pub display_name: String,
    pub keywords: Vec<String>,
    pub target_profile: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveEqConfig {
    pub enabled: bool,
    pub yt_music_or_browser_only: bool,
    pub online_lookup_enabled: bool,
    pub only_when_playing: bool,
    pub poll_interval_ms: u64,
    pub default_profile: String,
    pub custom_rules: Vec<AdaptiveRule>,
    pub genre_mappings: Vec<GenreCategoryMapping>,
}

impl Default for AdaptiveEqConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            yt_music_or_browser_only: true,
            online_lookup_enabled: true,
            only_when_playing: true,
            poll_interval_ms: 2000,
            default_profile: "SoundcoreSignature".to_string(),
            custom_rules: Vec::new(),
            genre_mappings: default_genre_mappings(),
        }
    }
}

impl AdaptiveEqConfig {
    pub fn config_file_path(config_dir: &Path) -> PathBuf {
        config_dir.join("adaptive_eq.json")
    }

    pub fn load_or_default(config_dir: &Path) -> Self {
        let path = Self::config_file_path(config_dir);
        match std::fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<Self>(&content) {
                Ok(mut parsed) => {
                    let defaults = default_genre_mappings();
                    if parsed.genre_mappings.is_empty() {
                        parsed.genre_mappings = defaults;
                    } else {
                        for mapping in &mut parsed.genre_mappings {
                            if let Some(def) = defaults
                                .iter()
                                .find(|d| d.category_id == mapping.category_id)
                            {
                                mapping.display_name = def.display_name.clone();
                            }
                        }
                    }
                    if parsed.poll_interval_ms < 500 {
                        parsed.poll_interval_ms = 2000;
                    }
                    parsed
                }
                Err(err) => {
                    tracing::warn!("Failed to parse adaptive_eq.json ({err:?}), using defaults");
                    Self::default()
                }
            },
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self, config_dir: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(config_dir)?;
        let path = Self::config_file_path(config_dir);
        let content = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(path, content)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolutionResult {
    pub detected_genre: String,
    pub detection_source: String,
    pub target_profile: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveHistoryEntry {
    pub track_display: String,
    pub detected_genre: String,
    pub detection_source: String,
    pub applied_profile: String,
}

pub fn default_genre_mappings() -> Vec<GenreCategoryMapping> {
    vec![
        GenreCategoryMapping {
            category_id: "rock_metal".to_string(),
            display_name: "Rock / Metal / Alternative".to_string(),
            keywords: vec![
                "rock",
                "hard rock",
                "metal",
                "heavy metal",
                "metalcore",
                "post-hardcore",
                "deathcore",
                "nu metal",
                "thrash",
                "punk",
                "pop punk",
                "grunge",
                "alternative",
                "indie rock",
                "anadolu rock",
                "turkish rock",
                "prog-rock",
                "progressive rock",
                "hardcore",
                "emo",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Rock".to_string(),
        },
        GenreCategoryMapping {
            category_id: "hiphop_rap".to_string(),
            display_name: "Hip-Hop / Rap / Trap".to_string(),
            keywords: vec![
                "hip-hop",
                "hip hop",
                "rap",
                "trap",
                "drill",
                "grime",
                "old school rap",
                "gangsta rap",
                "turkish hip-hop",
                "turkce rap",
                "türkçe rap",
                "underground rap",
                "alternative rap",
                "boom bap",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "HipHop".to_string(),
        },
        GenreCategoryMapping {
            category_id: "pop".to_string(),
            display_name: "Pop / K-Pop / Synthpop".to_string(),
            keywords: vec![
                "pop",
                "turkish pop",
                "türkçe pop",
                "k-pop",
                "j-pop",
                "synthpop",
                "dance pop",
                "indie pop",
                "teen pop",
                "electropop",
                "britpop",
                "adult contemporary",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Pop".to_string(),
        },
        GenreCategoryMapping {
            category_id: "electronic".to_string(),
            display_name: "Electronic / EDM / Techno".to_string(),
            keywords: vec![
                "electronic",
                "electronica",
                "edm",
                "techno",
                "trance",
                "psytrance",
                "drum & bass",
                "drum and bass",
                "dnb",
                "dubstep",
                "synthwave",
                "retrowave",
                "cyberpunk",
                "electro",
                "idm",
                "ambient",
                "industrial",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Electronic".to_string(),
        },
        GenreCategoryMapping {
            category_id: "dance_house".to_string(),
            display_name: "Dance / House / Reggae / Club".to_string(),
            keywords: vec![
                "dance",
                "house",
                "tech house",
                "progressive house",
                "disco",
                "nu-disco",
                "eurodance",
                "club",
                "uk garage",
                "garage",
                "breakbeat",
                "reggae",
                "dancehall",
                "roots reggae",
                "dub",
                "ska",
                "afrobeats",
                "afrobeat",
                "afro-beat",
                "afro-pop",
                "amapiano",
                "fitness & workout",
                "exercise",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Dance".to_string(),
        },
        GenreCategoryMapping {
            category_id: "bass_phonk".to_string(),
            display_name: "Bass / Phonk / Hardstyle".to_string(),
            keywords: vec![
                "phonk",
                "drift phonk",
                "bass boost",
                "bass boosted",
                "sub bass",
                "hardstyle",
                "hardbass",
                "big room",
                "808",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "BassBooster".to_string(),
        },
        GenreCategoryMapping {
            category_id: "rnb_soul".to_string(),
            display_name: "R&B / Soul / Funk".to_string(),
            keywords: vec![
                "r&b",
                "rnb",
                "soul",
                "neo-soul",
                "funk",
                "motown",
                "contemporary r&b",
                "disco/soul",
                "gospel",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "RnB".to_string(),
        },
        GenreCategoryMapping {
            category_id: "classical_score".to_string(),
            display_name: "Classical / Orchestra / Soundtrack".to_string(),
            keywords: vec![
                "classical",
                "klasik",
                "orchestra",
                "orchestral",
                "symphony",
                "concerto",
                "opera",
                "baroque",
                "chamber music",
                "soundtrack",
                "film score",
                "original score",
                "ost",
                "sanat",
                "türk sanat",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Classical".to_string(),
        },
        GenreCategoryMapping {
            category_id: "jazz_blues".to_string(),
            display_name: "Jazz / Blues / Swing".to_string(),
            keywords: vec![
                "jazz",
                "caz",
                "blues",
                "swing",
                "bebop",
                "smooth jazz",
                "vocal jazz",
                "fusion",
                "big band",
                "bossa",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Jazz".to_string(),
        },
        GenreCategoryMapping {
            category_id: "acoustic_folk".to_string(),
            display_name: "Acoustic / Folk / Country".to_string(),
            keywords: vec![
                "acoustic",
                "akustik",
                "unplugged",
                "folk",
                "singer/songwriter",
                "singer-songwriter",
                "country",
                "americana",
                "bluegrass",
                "halk",
                "türkü",
                "turku",
                "özgün",
                "arabesque",
                "arabesk",
                "fantezi",
                "world",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Acoustic".to_string(),
        },
        GenreCategoryMapping {
            category_id: "latin".to_string(),
            display_name: "Latin / Reggaeton / Salsa".to_string(),
            keywords: vec![
                "latin",
                "latino",
                "reggaeton",
                "salsa",
                "bachata",
                "samba",
                "flamenco",
                "tango",
                "cumbia",
                "merengue",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Latin".to_string(),
        },
        GenreCategoryMapping {
            category_id: "piano".to_string(),
            display_name: "Piano / Instrumental / New Age".to_string(),
            keywords: vec!["piano", "piyano", "solo piano", "new age", "neoclassical"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            target_profile: "Piano".to_string(),
        },
        GenreCategoryMapping {
            category_id: "lounge_lofi".to_string(),
            display_name: "Lounge / Lo-Fi / Chillout".to_string(),
            keywords: vec![
                "lounge",
                "lo-fi",
                "lofi",
                "chillout",
                "chillhop",
                "downtempo",
                "easy listening",
                "chill",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Lounge".to_string(),
        },
        GenreCategoryMapping {
            category_id: "deep".to_string(),
            display_name: "Deep House / Minimal / Deep".to_string(),
            keywords: vec!["deep house", "deep", "minimal", "melodic techno"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            target_profile: "Deep".to_string(),
        },
        GenreCategoryMapping {
            category_id: "podcast".to_string(),
            display_name: "Podcast / Talk / Comedy".to_string(),
            keywords: vec![
                "podcast",
                "söyleşi",
                "röportaj",
                "interview",
                "talk show",
                "comedy",
                "stand-up",
                "standup",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "Podcast".to_string(),
        },
        GenreCategoryMapping {
            category_id: "spoken_word".to_string(),
            display_name: "Audiobook / Spoken Word".to_string(),
            keywords: vec![
                "spoken word",
                "audiobook",
                "sesli kitap",
                "speech",
                "lecture",
                "ders",
                "news",
                "haber",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            target_profile: "SpokenWord".to_string(),
        },
    ]
}

/// Returns `(options, localized_options)` for the device's `PresetEqualizerProfile` setting,
/// falling back to `STANDARD_PRESET_PROFILES` if the device does not expose options.
pub fn device_preset_profiles(device: &dyn OpenSCQ30Device) -> (Vec<String>, Vec<String>) {
    if let Some(Setting::PresetEqualizerProfileSelect { select, .. }) =
        device.setting(&SettingId::PresetEqualizerProfile)
        && !select.options.is_empty()
    {
        let opts = select.options.iter().map(ToString::to_string).collect();
        let loc = if select.localized_options.len() == select.options.len() {
            select.localized_options
        } else {
            select.options.iter().map(ToString::to_string).collect()
        };
        return (opts, loc);
    }

    let std_opts: Vec<String> = STANDARD_PRESET_PROFILES
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    (std_opts.clone(), std_opts)
}

pub fn current_device_preset_profile(device: &dyn OpenSCQ30Device) -> Option<String> {
    if let Some(Setting::PresetEqualizerProfileSelect { value, .. }) =
        device.setting(&SettingId::PresetEqualizerProfile)
    {
        value.map(|v| v.into_owned())
    } else {
        None
    }
}

/// Applies the target built-in equalizer preset to the connected device if it is not already active.
/// Returns `Ok(true)` if the preset was changed, or `Ok(false)` if it was already set.
pub async fn apply_preset_profile(
    device: &(dyn OpenSCQ30Device + Send + Sync),
    profile_name: &str,
) -> device::Result<bool> {
    let (available_options, _) = device_preset_profiles(device);
    let canonical_profile = available_options
        .iter()
        .find(|opt| opt.eq_ignore_ascii_case(profile_name))
        .cloned()
        .unwrap_or_else(|| profile_name.to_string());

    if let Some(current) = current_device_preset_profile(device)
        && current.eq_ignore_ascii_case(&canonical_profile)
    {
        return Ok(false);
    }

    device
        .set_setting_values(vec![(
            SettingId::PresetEqualizerProfile,
            Value::OptionalString(Some(Cow::Owned(canonical_profile))),
        )])
        .await?;

    Ok(true)
}

/// Cleans YouTube / YouTube Music raw artist and title into `(clean_artist, clean_title)`.
/// Handles cases where `raw_artist` is a channel/label (e.g. `"Epitaph Records"` or `"Artist - Topic"`)
/// and `raw_title` is `'Falling In Reverse - "God Is A Weapon (feat. Marilyn Manson)"'`.
pub fn parse_yt_music_metadata(raw_artist: &str, raw_title: &str) -> (String, String) {
    let cleaned_raw_artist = strip_artist_suffixes(raw_artist);
    let stripped_title = strip_youtube_title_noise(raw_title);

    // Check if the title contains "Artist - Track" separator
    for sep in [" - ", " – ", " — ", " | "] {
        if let Some((left, right)) = stripped_title.split_once(sep) {
            let left_clean = strip_artist_suffixes(left);
            let right_clean = strip_feat_and_quotes(right);
            if !left_clean.is_empty() && !right_clean.is_empty() {
                return (left_clean, right_clean);
            }
        }
    }

    let clean_title = strip_feat_and_quotes(&stripped_title);
    (cleaned_raw_artist, clean_title)
}

fn strip_artist_suffixes(artist: &str) -> String {
    let mut s = artist.trim().to_string();
    for suffix in [" - Topic", " - Konu", "VEVO", " Official", " Resmi"] {
        if s.len() > suffix.len() && s.to_lowercase().ends_with(&suffix.to_lowercase()) {
            s.truncate(s.len() - suffix.len());
            s = s.trim().to_string();
        }
    }
    s
}

fn strip_youtube_title_noise(title: &str) -> String {
    let mut s = title.trim().to_string();
    let noise_tags = [
        "(official music video)",
        "[official music video]",
        "(official video)",
        "[official video]",
        "(official audio)",
        "[official audio]",
        "(official lyric video)",
        "[official lyric video]",
        "(lyric video)",
        "[lyric video]",
        "(lyrics)",
        "[lyrics]",
        "(visualizer)",
        "[visualizer]",
        "(audio)",
        "[audio]",
        "(hd)",
        "[hd]",
        "(4k)",
        "[4k]",
        "(hq)",
        "[hq]",
        "(resmi video)",
        "(resmi klip)",
        "[resmi klip]",
        "(video klip)",
    ];

    loop {
        let lower = s.to_lowercase();
        let mut changed = false;
        for tag in noise_tags {
            if let Some(idx) = lower.find(tag) {
                s.replace_range(idx..idx + tag.len(), "");
                s = s.trim().to_string();
                changed = true;
                break;
            }
        }
        if !changed {
            break;
        }
    }
    s
}

fn strip_feat_and_quotes(track: &str) -> String {
    let mut s = track.trim().to_string();

    // Remove (feat. ...), (ft. ...), [feat. ...], [ft. ...]
    for marker in ["(feat.", "(ft.", "[feat.", "[ft.", "(with "] {
        let lower = s.to_lowercase();
        if let Some(start) = lower.find(marker) {
            let closing = if marker.starts_with('(') { ')' } else { ']' };
            if let Some(end_rel) = s[start..].find(closing) {
                s.replace_range(start..=start + end_rel, "");
                s = s.trim().to_string();
            } else {
                s.truncate(start);
                s = s.trim().to_string();
            }
        }
    }

    // Strip surrounding quotes if present
    let trim_chars: &[char] = &['"', '\'', '“', '”', '«', '»'];
    s = s.trim_matches(trim_chars).trim().to_string();

    // Also handle case where feat was outside inner quotes, e.g. `"Song" (feat. X)`
    s = s.trim_matches(trim_chars).trim().to_string();
    s
}

/// Queries D-Bus MPRIS (`org.mpris.MediaPlayer2.*`) on Linux for all active media players.
pub async fn fetch_active_media_tracks() -> Vec<MediaTrackInfo> {
    tokio::task::spawn_blocking(|| {
        let Ok(list_output) = Command::new("dbus-send")
            .args([
                "--session",
                "--dest=org.freedesktop.DBus",
                "--type=method_call",
                "--print-reply",
                "/org/freedesktop/DBus",
                "org.freedesktop.DBus.ListNames",
            ])
            .output()
        else {
            return Vec::new();
        };

        if !list_output.status.success() {
            return Vec::new();
        }

        let stdout = String::from_utf8_lossy(&list_output.stdout);
        let player_bus_names = parse_mpris_bus_names(&stdout);

        let mut tracks = Vec::new();
        for bus_name in player_bus_names {
            let Ok(props_output) = Command::new("dbus-send")
                .args([
                    "--session",
                    "--print-reply",
                    &format!("--dest={bus_name}"),
                    "/org/mpris/MediaPlayer2",
                    "org.freedesktop.DBus.Properties.GetAll",
                    "string:org.mpris.MediaPlayer2.Player",
                ])
                .output()
            else {
                continue;
            };

            if !props_output.status.success() {
                continue;
            }

            let props_stdout = String::from_utf8_lossy(&props_output.stdout);
            if let Some(track) = parse_dbus_get_all_output(&bus_name, &props_stdout) {
                tracks.push(track);
            }
        }

        tracks
    })
    .await
    .unwrap_or_default()
}

/// Selects the best active track (prioritizing playing browser/YT Music tracks).
pub async fn select_best_media_track(
    yt_music_or_browser_only: bool,
    only_when_playing: bool,
) -> Option<MediaTrackInfo> {
    let tracks = fetch_active_media_tracks().await;
    select_best_from_list(tracks, yt_music_or_browser_only, only_when_playing)
}

pub fn select_best_from_list(
    tracks: Vec<MediaTrackInfo>,
    yt_music_or_browser_only: bool,
    only_when_playing: bool,
) -> Option<MediaTrackInfo> {
    let mut candidates: Vec<MediaTrackInfo> = tracks
        .into_iter()
        .filter(|t| !t.raw_title.trim().is_empty())
        .filter(|t| !yt_music_or_browser_only || t.is_browser_or_yt_music)
        .filter(|t| !only_when_playing || t.is_playing())
        .collect();

    // Sort so that:
    // 1. Playing comes before Paused
    // 2. Browser / YT Music comes before other players
    candidates.sort_by_key(|t| (!t.is_playing(), !t.is_browser_or_yt_music));
    candidates.into_iter().next()
}

pub fn parse_mpris_bus_names(dbus_list_stdout: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in dbus_list_stdout.lines() {
        if let Some(val) = extract_dbus_string_line(line)
            && val.starts_with("org.mpris.MediaPlayer2.")
        {
            names.push(val);
        }
    }
    names
}

pub fn parse_dbus_get_all_output(bus_name: &str, stdout: &str) -> Option<MediaTrackInfo> {
    let mut playback_status = String::new();
    let mut raw_title = String::new();
    let mut raw_album = String::new();
    let mut url = String::new();
    let mut artists: Vec<String> = Vec::new();
    let mut genres: Vec<String> = Vec::new();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum FieldState {
        None,
        PlaybackStatus,
        Title,
        Album,
        Url,
        Artist,
        Genre,
    }

    let mut state = FieldState::None;
    let mut in_array = false;

    for line in stdout.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("dict entry(") {
            state = FieldState::None;
            in_array = false;
            continue;
        }

        if state == FieldState::None {
            if let Some(key) = extract_dbus_string_line(trimmed) {
                state = match key.as_str() {
                    "PlaybackStatus" => FieldState::PlaybackStatus,
                    "xesam:title" => FieldState::Title,
                    "xesam:album" => FieldState::Album,
                    "xesam:url" => FieldState::Url,
                    "xesam:artist" => FieldState::Artist,
                    "xesam:genre" => FieldState::Genre,
                    _ => FieldState::None,
                };
            }
            continue;
        }

        if trimmed.contains("array [") {
            in_array = true;
            continue;
        }
        if in_array && trimmed == "]" {
            in_array = false;
            state = FieldState::None;
            continue;
        }

        if let Some(val) = extract_dbus_string_line(trimmed) {
            match state {
                FieldState::PlaybackStatus => {
                    playback_status = val;
                    state = FieldState::None;
                }
                FieldState::Title => {
                    raw_title = val;
                    state = FieldState::None;
                }
                FieldState::Album => {
                    raw_album = val;
                    state = FieldState::None;
                }
                FieldState::Url => {
                    url = val;
                    state = FieldState::None;
                }
                FieldState::Artist => {
                    if !val.trim().is_empty() {
                        artists.push(val);
                    }
                    if !in_array {
                        state = FieldState::None;
                    }
                }
                FieldState::Genre => {
                    if !val.trim().is_empty() {
                        genres.push(val);
                    }
                    if !in_array {
                        state = FieldState::None;
                    }
                }
                FieldState::None => (),
            }
        }
    }

    if raw_title.trim().is_empty() && playback_status.is_empty() {
        return None;
    }

    let raw_artist = artists.join(", ");
    let (clean_artist, clean_title) = parse_yt_music_metadata(&raw_artist, &raw_title);

    let short_raw = bus_name
        .strip_prefix("org.mpris.MediaPlayer2.")
        .unwrap_or(bus_name);
    let player_short_name = short_raw
        .split(".instance")
        .next()
        .unwrap_or(short_raw)
        .to_string();

    let lower_bus = player_short_name.to_lowercase();
    let lower_url = url.to_lowercase();
    let is_browser_or_yt_music = [
        "chromium",
        "chrome",
        "google-chrome",
        "firefox",
        "brave",
        "vivaldi",
        "opera",
        "msedge",
        "edge",
        "zen",
        "thorium",
        "librewolf",
        "waterfox",
        "floorp",
        "epiphany",
        "youtube",
        "ytmusic",
        "pear",
        "plasma-browser-integration",
    ]
    .iter()
    .any(|kw| lower_bus.contains(kw))
        || lower_url.contains("youtube.com")
        || lower_url.contains("youtu.be");

    Some(MediaTrackInfo {
        player_bus_name: bus_name.to_string(),
        player_short_name,
        playback_status,
        raw_title,
        raw_artist,
        raw_album,
        clean_artist,
        clean_title,
        mpris_genres: genres,
        url,
        is_browser_or_yt_music,
    })
}

fn extract_dbus_string_line(line: &str) -> Option<String> {
    let marker = "string \"";
    let start = line.find(marker)? + marker.len();
    let end = line.rfind('"')?;
    if end >= start {
        Some(line[start..end].to_string())
    } else {
        None
    }
}

/// Resolves the best Soundcore EQ preset profile for the given `MediaTrackInfo` using:
/// 1. User custom rules
/// 2. Direct title/version keywords (Acoustic, Unplugged, Piano, Podcast, Audiobook, Bass Boost/Phonk)
/// 3. MPRIS genre metadata (if present)
/// 4. Online music APIs (iTunes Song, Deezer Track/Album, iTunes Artist, MusicBrainz Artist) with caching
/// 5. Broad keyword & record label fallback
/// 6. Configured default fallback profile
pub async fn resolve_profile_for_track(
    track: &MediaTrackInfo,
    config: &AdaptiveEqConfig,
    online_cache: &mut HashMap<String, Option<(String, String)>>,
) -> ResolutionResult {
    // 1. Check user custom rules first
    if let Some(res) = match_custom_rules(track, &config.custom_rules, None) {
        return res;
    }

    // 2. Check high-priority track version / content keywords in the title
    if let Some(res) = match_priority_title_keywords(track, config) {
        return res;
    }

    // 3. Check MPRIS metadata genres if provided by the player
    for genre in &track.mpris_genres {
        if let Some(res) = match_custom_rules(track, &config.custom_rules, Some(genre)) {
            return res;
        }
        if let Some(profile) = map_genre_to_profile(genre, &config.genre_mappings) {
            return ResolutionResult {
                detected_genre: genre.clone(),
                detection_source: "MPRIS Metadata".to_string(),
                target_profile: profile,
            };
        }
    }

    // 4. Online Genre Lookup (iTunes / Deezer / MusicBrainz)
    let mut unmapped_online_genre: Option<(String, String)> = None;
    if config.online_lookup_enabled {
        let cache_key = format!(
            "{}|{}",
            track.clean_artist.to_lowercase(),
            track.clean_title.to_lowercase()
        );
        let online_genre_info = if let Some(cached) = online_cache.get(&cache_key) {
            cached.clone()
        } else {
            let fetched = lookup_genre_online(track).await;
            online_cache.insert(cache_key, fetched.clone());
            fetched
        };

        if let Some((genre, source)) = online_genre_info {
            if let Some(res) = match_custom_rules(track, &config.custom_rules, Some(&genre)) {
                return res;
            }
            if let Some(profile) = map_genre_to_profile(&genre, &config.genre_mappings) {
                return ResolutionResult {
                    detected_genre: genre,
                    detection_source: source,
                    target_profile: profile,
                };
            }
            unmapped_online_genre = Some((genre, source));
        }
    }

    // 5. Broad keyword & record label fallback on artist/title/album
    if let Some(res) = match_broad_keywords_and_labels(track, config) {
        return res;
    }

    // 6. Default fallback profile
    if let Some((genre, source)) = unmapped_online_genre {
        ResolutionResult {
            detected_genre: genre,
            detection_source: format!("{source} (Default Fallback)"),
            target_profile: config.default_profile.clone(),
        }
    } else {
        ResolutionResult {
            detected_genre: "Unknown".to_string(),
            detection_source: "Default Profile".to_string(),
            target_profile: config.default_profile.clone(),
        }
    }
}

pub fn match_custom_rules(
    track: &MediaTrackInfo,
    rules: &[AdaptiveRule],
    external_genre: Option<&str>,
) -> Option<ResolutionResult> {
    let artist_haystack = format!(
        "{} {}",
        track.clean_artist.to_lowercase(),
        track.raw_artist.to_lowercase()
    );
    let keyword_haystack = format!(
        "{} {} {} {}",
        track.raw_title.to_lowercase(),
        track.clean_title.to_lowercase(),
        track.raw_artist.to_lowercase(),
        track.raw_album.to_lowercase()
    );

    for rule in rules {
        let pat = rule.pattern.trim().to_lowercase();
        if pat.is_empty() {
            continue;
        }
        match rule.match_type {
            RuleMatchType::Artist => {
                if artist_haystack.contains(&pat) {
                    return Some(ResolutionResult {
                        detected_genre: format!("Artist: {}", rule.pattern),
                        detection_source: "Custom Rule".to_string(),
                        target_profile: rule.target_profile.clone(),
                    });
                }
            }
            RuleMatchType::Keyword => {
                if keyword_haystack.contains(&pat) {
                    return Some(ResolutionResult {
                        detected_genre: format!("Keyword: {}", rule.pattern),
                        detection_source: "Custom Rule".to_string(),
                        target_profile: rule.target_profile.clone(),
                    });
                }
            }
            RuleMatchType::Genre => {
                if let Some(g) = external_genre
                    && g.to_lowercase().contains(&pat)
                {
                    return Some(ResolutionResult {
                        detected_genre: g.to_string(),
                        detection_source: "Custom Rule (Genre)".to_string(),
                        target_profile: rule.target_profile.clone(),
                    });
                }
                if track
                    .mpris_genres
                    .iter()
                    .any(|g| g.to_lowercase().contains(&pat))
                {
                    return Some(ResolutionResult {
                        detected_genre: rule.pattern.clone(),
                        detection_source: "Custom Rule (Genre)".to_string(),
                        target_profile: rule.target_profile.clone(),
                    });
                }
            }
        }
    }
    None
}

fn match_priority_title_keywords(
    track: &MediaTrackInfo,
    config: &AdaptiveEqConfig,
) -> Option<ResolutionResult> {
    let title_lower = track.raw_title.to_lowercase();
    let combined_lower = format!(
        "{} {}",
        track.raw_title.to_lowercase(),
        track.raw_artist.to_lowercase()
    );

    let priority_checks: &[(&[&str], &str, &str, bool)] = &[
        (
            &[
                "podcast",
                "söyleşi",
                "röportaj",
                "interview",
                "talk show",
                "stand-up",
                "standup",
            ],
            "podcast",
            "Podcast",
            true,
        ),
        (
            &["audiobook", "sesli kitap", "spoken word"],
            "spoken_word",
            "SpokenWord",
            true,
        ),
        (
            &["acoustic", "akustik", "unplugged"],
            "acoustic_folk",
            "Acoustic",
            false,
        ),
        (
            &["piano version", "solo piano", "piyano", "piano cover"],
            "piano",
            "Piano",
            false,
        ),
        (
            &["bass boosted", "bass boost", "extreme bass", "phonk"],
            "bass_phonk",
            "BassBooster",
            false,
        ),
        (
            &["lo-fi", "lofi", "chillhop"],
            "lounge_lofi",
            "Lounge",
            true,
        ),
    ];

    for (keywords, category_id, fallback_profile, check_artist_too) in priority_checks {
        let haystack = if *check_artist_too {
            &combined_lower
        } else {
            &title_lower
        };
        if let Some(matched_kw) = keywords.iter().find(|kw| haystack.contains(**kw)) {
            let profile = config
                .genre_mappings
                .iter()
                .find(|m| m.category_id == *category_id)
                .map_or_else(
                    || (*fallback_profile).to_string(),
                    |m| m.target_profile.clone(),
                );
            return Some(ResolutionResult {
                detected_genre: (*matched_kw).to_string(),
                detection_source: "Title Analysis".to_string(),
                target_profile: profile,
            });
        }
    }

    None
}

pub fn map_genre_to_profile(genre: &str, mappings: &[GenreCategoryMapping]) -> Option<String> {
    let genre_lower = genre.trim().to_lowercase();
    if genre_lower.is_empty() || genre_lower == "music" || genre_lower == "müzik" {
        return None;
    }

    // 1. Exact keyword match first
    for mapping in mappings {
        if mapping
            .keywords
            .iter()
            .any(|kw| kw.eq_ignore_ascii_case(&genre_lower))
        {
            return Some(mapping.target_profile.clone());
        }
    }

    // 2. Substring match (preferring longer keyword matches first)
    let mut best_match: Option<(usize, &str)> = None;
    for mapping in mappings {
        for kw in &mapping.keywords {
            let kw_lower = kw.to_lowercase();
            if genre_lower.contains(&kw_lower) {
                let len = kw_lower.len();
                if best_match.is_none_or(|(best_len, _)| len > best_len) {
                    best_match = Some((len, mapping.target_profile.as_str()));
                }
            }
        }
    }

    best_match.map(|(_, profile)| profile.to_string())
}

fn match_broad_keywords_and_labels(
    track: &MediaTrackInfo,
    config: &AdaptiveEqConfig,
) -> Option<ResolutionResult> {
    let combined = format!(
        "{} {} {}",
        track.raw_title.to_lowercase(),
        track.raw_artist.to_lowercase(),
        track.raw_album.to_lowercase()
    );

    // Check known record labels / channels often seen as `xesam:artist` on YouTube
    let label_hints: &[(&[&str], &str, &str)] = &[
        (
            &[
                "epitaph",
                "nuclear blast",
                "century media",
                "roadrunner",
                "sumerian",
                "fearless records",
                "rise records",
                "hopeless records",
                "napalm records",
                "metal blade",
                "fueled by ramen",
            ],
            "rock_metal",
            "Rock",
        ),
        (
            &[
                "spinnin",
                "monstercat",
                "armada",
                "ultra music",
                "revealed",
                "anjunabeats",
                "drumcode",
                "defected",
                "selected.",
                "ncs",
                "nocopyrightsounds",
            ],
            "electronic",
            "Electronic",
        ),
        (&["blue note", "verve"], "jazz_blues", "Jazz"),
        (
            &[
                "deutsche grammophon",
                "decca classics",
                "philharmonia",
                "symphony orchestra",
            ],
            "classical_score",
            "Classical",
        ),
    ];

    for (labels, cat_id, fallback) in label_hints {
        if let Some(label) = labels.iter().find(|l| combined.contains(**l)) {
            let profile = config
                .genre_mappings
                .iter()
                .find(|m| m.category_id == *cat_id)
                .map_or_else(|| (*fallback).to_string(), |m| m.target_profile.clone());
            return Some(ResolutionResult {
                detected_genre: format!("Channel/Label ({label})"),
                detection_source: "Channel Analysis".to_string(),
                target_profile: profile,
            });
        }
    }

    // Check if any genre keyword appears directly in the title/album
    if let Some(profile) = map_genre_to_profile(&combined, &config.genre_mappings) {
        return Some(ResolutionResult {
            detected_genre: "Keyword".to_string(),
            detection_source: "Title/Artist Analysis".to_string(),
            target_profile: profile,
        });
    }

    None
}

async fn lookup_genre_online(track: &MediaTrackInfo) -> Option<(String, String)> {
    let artist = if !track.clean_artist.is_empty() {
        track.clean_artist.as_str()
    } else {
        track.raw_artist.as_str()
    };
    let title = if !track.clean_title.is_empty() {
        track.clean_title.as_str()
    } else {
        track.raw_title.as_str()
    };

    // 1. Try iTunes Song Search
    if !artist.is_empty()
        && !title.is_empty()
        && let Some(genre) = query_itunes_song_genre(artist, &track.raw_artist, title).await
    {
        return Some((genre, "iTunes API (Track)".to_string()));
    }

    // 2. Try Deezer Track + Album Genre Search
    if !artist.is_empty()
        && !title.is_empty()
        && let Some(genre) = query_deezer_track_genre(artist, &track.raw_artist, title).await
    {
        return Some((genre, "Deezer API".to_string()));
    }

    // 3. Try iTunes Artist Genre Search (very reliable when song is new or YouTube-only)
    if !artist.is_empty()
        && let Some(genre) = query_itunes_artist_genre(artist).await
    {
        return Some((genre, "iTunes API (Artist)".to_string()));
    }

    if !track.raw_artist.is_empty()
        && !track.raw_artist.eq_ignore_ascii_case(artist)
        && let Some(genre) = query_itunes_artist_genre(&track.raw_artist).await
    {
        return Some((genre, "iTunes API (Artist)".to_string()));
    }

    // 4. Fallback to MusicBrainz Artist Tag Search
    if !artist.is_empty()
        && let Some(genre) = query_musicbrainz_artist_tag(artist).await
    {
        return Some((genre, "MusicBrainz API".to_string()));
    }

    None
}

fn artists_match(query_artist: &str, raw_artist: &str, candidate_artist: &str) -> bool {
    let cand = candidate_artist.trim().to_lowercase();
    if cand.is_empty() {
        return false;
    }
    for target in [query_artist, raw_artist] {
        let t = target.trim().to_lowercase();
        if !t.is_empty() && (cand.contains(&t) || t.contains(&cand)) {
            return true;
        }
    }
    false
}

fn is_useful_genre(genre: &str) -> bool {
    let g = genre.trim().to_lowercase();
    !g.is_empty() && g != "music" && g != "müzik" && g != "unknown"
}

async fn curl_get_json(url: &str) -> Option<serde_json::Value> {
    let url = url.to_string();
    tokio::task::spawn_blocking(move || {
        let output = Command::new("curl")
            .args([
                "-s",
                "-L",
                "--max-time",
                "4",
                "-A",
                "OpenSCQ30-AdaptiveEQ/2.13",
                &url,
            ])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        serde_json::from_slice(&output.stdout).ok()
    })
    .await
    .ok()
    .flatten()
}

fn url_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len() * 2);
    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{b:02X}")),
        }
    }
    encoded
}

async fn query_itunes_song_genre(
    clean_artist: &str,
    raw_artist: &str,
    clean_title: &str,
) -> Option<String> {
    let term = url_encode(&format!("{clean_artist} {clean_title}"));
    let url =
        format!("https://itunes.apple.com/search?term={term}&media=music&entity=song&limit=5");
    let json = curl_get_json(&url).await?;
    let results = json.get("results")?.as_array()?;

    for item in results {
        let candidate_artist = item
            .get("artistName")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let genre = item
            .get("primaryGenreName")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        if artists_match(clean_artist, raw_artist, candidate_artist) && is_useful_genre(genre) {
            return Some(genre.to_string());
        }
    }
    None
}

async fn query_itunes_artist_genre(artist: &str) -> Option<String> {
    let term = url_encode(artist);
    let url = format!("https://itunes.apple.com/search?term={term}&entity=musicArtist&limit=3");
    let json = curl_get_json(&url).await?;
    let results = json.get("results")?.as_array()?;

    for item in results {
        let candidate_artist = item
            .get("artistName")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let genre = item
            .get("primaryGenreName")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        if artists_match(artist, artist, candidate_artist) && is_useful_genre(genre) {
            return Some(genre.to_string());
        }
    }
    None
}

async fn query_deezer_track_genre(
    clean_artist: &str,
    raw_artist: &str,
    clean_title: &str,
) -> Option<String> {
    let q = url_encode(&format!("{clean_artist} {clean_title}"));
    let search_url = format!("https://api.deezer.com/search?q={q}&limit=3");
    let json = curl_get_json(&search_url).await?;
    let data = json.get("data")?.as_array()?;

    for item in data {
        let candidate_artist = item
            .get("artist")
            .and_then(|a| a.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or_default();
        if !artists_match(clean_artist, raw_artist, candidate_artist) {
            continue;
        }
        let Some(album_id) = item
            .get("album")
            .and_then(|al| al.get("id"))
            .and_then(serde_json::Value::as_u64)
        else {
            continue;
        };

        let album_url = format!("https://api.deezer.com/album/{album_id}");
        if let Some(album_json) = curl_get_json(&album_url).await
            && let Some(genres) = album_json
                .get("genres")
                .and_then(|g| g.get("data"))
                .and_then(|d| d.as_array())
        {
            for g in genres {
                if let Some(name) = g.get("name").and_then(|n| n.as_str())
                    && is_useful_genre(name)
                {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

async fn query_musicbrainz_artist_tag(artist: &str) -> Option<String> {
    let q = url_encode(&format!("artist:\"{artist}\""));
    let url = format!("https://musicbrainz.org/ws/2/artist/?query={q}&fmt=json&limit=1");
    let json = curl_get_json(&url).await?;
    let artists = json.get("artists")?.as_array()?;
    let first = artists.first()?;
    let tags = first.get("tags")?.as_array()?;

    tags.iter()
        .filter_map(|tag| {
            let name = tag.get("name")?.as_str()?;
            let count = tag
                .get("count")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(0);
            is_useful_genre(name).then_some((count, name.to_string()))
        })
        .max_by_key(|(count, _)| *count)
        .map(|(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_yt_music_title_with_label_channel_and_quotes() {
        let (artist, title) = parse_yt_music_metadata(
            "Epitaph Records",
            r#"Falling In Reverse - "God Is A Weapon (feat. Marilyn Manson)""#,
        );
        assert_eq!(artist, "Falling In Reverse");
        assert_eq!(title, "God Is A Weapon");

        let (artist2, title2) = parse_yt_music_metadata(
            "Epitaph Records",
            r#"Falling In Reverse - "Joseph" (feat. Corey Taylor and Serj Tankian)"#,
        );
        assert_eq!(artist2, "Falling In Reverse");
        assert_eq!(title2, "Joseph");
    }

    #[test]
    fn parses_topic_channel_and_official_video_suffix() {
        let (artist, title) =
            parse_yt_music_metadata("Metallica - Topic", "Nothing Else Matters (Official Audio)");
        assert_eq!(artist, "Metallica");
        assert_eq!(title, "Nothing Else Matters");
    }

    #[test]
    fn parses_dbus_get_all_chromium_output() {
        let sample = r#"method return time=1793098224.541579 sender=:1.344 -> destination=:1.504 serial=1917 reply_serial=2
   array [
      dict entry(
         string "Metadata"
         variant             array [
               dict entry(
                  string "xesam:album"
                  variant                      string ""
               )
               dict entry(
                  string "xesam:artist"
                  variant                      array [
                        string "Epitaph Records"
                     ]
               )
               dict entry(
                  string "xesam:title"
                  variant                      string "Falling In Reverse - "Joseph" (feat. Corey Taylor and Serj Tankian)"
               )
            ]
      )
      dict entry(
         string "PlaybackStatus"
         variant             string "Playing"
      )
   ]"#;

        let info =
            parse_dbus_get_all_output("org.mpris.MediaPlayer2.chromium.instance1221877", sample)
                .unwrap();
        assert_eq!(info.player_short_name, "chromium");
        assert!(info.is_browser_or_yt_music);
        assert!(info.is_playing());
        assert_eq!(info.raw_artist, "Epitaph Records");
        assert_eq!(info.clean_artist, "Falling In Reverse");
        assert_eq!(info.clean_title, "Joseph");
    }

    #[test]
    fn maps_genres_and_turkish_genres_correctly() {
        let mappings = default_genre_mappings();
        assert_eq!(
            map_genre_to_profile("Hard Rock", &mappings).as_deref(),
            Some("Rock")
        );
        assert_eq!(
            map_genre_to_profile("Turkish Rock", &mappings).as_deref(),
            Some("Rock")
        );
        assert_eq!(
            map_genre_to_profile("Turkish Hip-Hop/Rap", &mappings).as_deref(),
            Some("HipHop")
        );
        assert_eq!(
            map_genre_to_profile("Arabesque", &mappings).as_deref(),
            Some("Acoustic")
        );
        assert_eq!(
            map_genre_to_profile("Classical", &mappings).as_deref(),
            Some("Classical")
        );
        assert_eq!(
            map_genre_to_profile("Sanat", &mappings).as_deref(),
            Some("Classical")
        );
        assert_eq!(
            map_genre_to_profile("Electronic", &mappings).as_deref(),
            Some("Electronic")
        );
    }
}
