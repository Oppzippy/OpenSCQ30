use std::{collections::HashMap, path::PathBuf, time::Duration};

use anyhow::anyhow;
use clap::{ArgMatches, Command, arg, value_parser};
use dirs::config_dir;
use macaddr::MacAddr6;
use openscq30_lib::adaptive_eq::{
    self, AdaptiveEqConfig, apply_preset_profile, resolve_profile_for_track,
    select_best_media_track,
};
use serde::Serialize;

use crate::openscq30_session;

pub fn build_subcommand() -> Command {
    Command::new("adaptive-eq")
        .about("Automatically switch Soundcore equalizer preset profiles based on YouTube Music / MPRIS media playback")
        .arg(
            arg!(-a --"mac-address" <MAC_ADDRESS> "Device's mac address (if omitted, uses the first paired device)")
                .value_parser(value_parser!(MacAddr6)),
        )
        .arg(arg!(--once "Detect current song and apply matching profile once, then exit"))
        .arg(arg!(--"dry-run" "Detect current song, genre, and target profile without connecting to headphones"))
        .arg(arg!(--"all-players" "Monitor all MPRIS media players, not just browsers / YouTube Music"))
        .arg(arg!(--"no-online" "Disable online genre lookup (iTunes / Deezer / MusicBrainz) and use only local rules/keywords"))
        .arg(arg!(--"default-profile" <PROFILE> "Override fallback equalizer preset (e.g. SoundcoreSignature, Flat, Pop)"))
        .arg(
            arg!(-i --interval <MS> "Polling interval in milliseconds (default: 2000)")
                .value_parser(value_parser!(u64)),
        )
        .arg(arg!(-j --json "Output events as JSON"))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AdaptiveCliEvent {
    player: String,
    playback_status: String,
    raw_artist: String,
    raw_title: String,
    clean_artist: String,
    clean_title: String,
    detected_genre: String,
    detection_source: String,
    target_profile: String,
    applied: bool,
}

fn default_config_dir() -> PathBuf {
    config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("openscq30")
}

pub async fn handle(matches: &ArgMatches, explicit_mac: Option<MacAddr6>) -> anyhow::Result<()> {
    let cfg_dir = default_config_dir();
    let mut config = AdaptiveEqConfig::load_or_default(&cfg_dir);

    if matches.get_flag("all-players") {
        config.yt_music_or_browser_only = false;
    }
    if matches.get_flag("no-online") {
        config.online_lookup_enabled = false;
    }
    if let Some(default_prof) = matches.get_one::<String>("default-profile") {
        config.default_profile = default_prof.to_owned();
    }
    if let Some(interval_ms) = matches.get_one::<u64>("interval") {
        config.poll_interval_ms = (*interval_ms).max(500);
    }

    let once = matches.get_flag("once");
    let dry_run = matches.get_flag("dry-run");
    let json = matches.get_flag("json");

    let device = if dry_run {
        None
    } else {
        let session = openscq30_session().await?;
        let mac_address = match explicit_mac
            .or_else(|| matches.get_one::<MacAddr6>("mac-address").copied())
        {
            Some(mac) => mac,
            None => {
                let paired = session.paired_devices().await?;
                paired
                    .first()
                    .map(|d| d.mac_address)
                    .ok_or_else(|| {
                        anyhow!(
                            "No paired Soundcore devices found. Pair your Q30 in the GUI or with `openscq30 paired-devices add`, or pass `--mac-address`."
                        )
                    })?
            }
        };
        if !json {
            println!("Connecting to Soundcore device ({mac_address})...");
        }
        Some(session.connect(mac_address).await?)
    };

    let mut online_cache: HashMap<String, Option<(String, String)>> = HashMap::new();
    let mut last_track_key: Option<String> = None;

    if !once && !json {
        println!(
            "Adaptive EQ started (YT Music / Browser only: {}, Online Genre Lookup: {}, Default Profile: {}). Listening for tracks...",
            config.yt_music_or_browser_only, config.online_lookup_enabled, config.default_profile
        );
    }

    loop {
        let track = select_best_media_track(
            config.yt_music_or_browser_only,
            config.only_when_playing && !once,
        )
        .await;

        if let Some(track) = track {
            let key = track.track_key();
            if once || last_track_key.as_deref() != Some(key.as_str()) {
                let resolution =
                    resolve_profile_for_track(&track, &config, &mut online_cache).await;
                let mut applied = false;
                if let Some(dev) = &device {
                    applied =
                        apply_preset_profile(dev.as_ref(), &resolution.target_profile).await?;
                }
                last_track_key = Some(key);

                if json {
                    let event = AdaptiveCliEvent {
                        player: track.player_short_name,
                        playback_status: track.playback_status,
                        raw_artist: track.raw_artist,
                        raw_title: track.raw_title,
                        clean_artist: track.clean_artist,
                        clean_title: track.clean_title,
                        detected_genre: resolution.detected_genre,
                        detection_source: resolution.detection_source,
                        target_profile: resolution.target_profile,
                        applied,
                    };
                    println!("{}", serde_json::to_string_pretty(&event)?);
                } else {
                    println!(
                        "[{}] {} | Genre: {} ({}) -> EQ Profile: {}{}",
                        track.player_short_name,
                        track.display_title(),
                        resolution.detected_genre,
                        resolution.detection_source,
                        resolution.target_profile,
                        if dry_run {
                            " [Dry-Run]"
                        } else if applied {
                            " [Applied]"
                        } else {
                            " [Already Active]"
                        }
                    );
                }
            }
        } else if once {
            if json {
                println!("null");
            } else {
                println!("No active YouTube Music / MPRIS media player found.");
            }
        } else {
            last_track_key = None;
        }

        if once {
            break;
        }

        tokio::time::sleep(Duration::from_millis(config.poll_interval_ms)).await;
    }

    // Use adaptive_eq module reference to avoid unused import warnings if any
    let _ = adaptive_eq::STANDARD_PRESET_PROFILES;

    Ok(())
}
