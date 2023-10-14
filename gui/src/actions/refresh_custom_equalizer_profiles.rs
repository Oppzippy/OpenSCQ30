use anyhow::Context;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    objects::GlibCustomEqualizerProfile,
    settings::{Config, SettingsFile},
};

use super::StateUpdate;

pub fn refresh_custom_equalizer_profiles(
    state_update_sender: &UnboundedSender<StateUpdate>,
    settings_file: &SettingsFile<Config>,
) -> anyhow::Result<()> {
    settings_file
        .get(|settings| {
            let custom_profiles = settings
                .custom_profiles()
                .iter()
                .map(|(name, profile)| {
                    GlibCustomEqualizerProfile::new(name, profile.volume_adjustments())
                })
                .collect();
            state_update_sender
                .send(StateUpdate::SetCustomEqualizerProfiles(custom_profiles))
                .unwrap();
        })
        .context("refresh custom equalizer profiles")
}
