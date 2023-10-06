use std::rc::Rc;

use anyhow::Context;

use super::{Config, SettingsFile, State};

#[derive(Debug, Clone)]
pub struct Settings {
    pub state: Rc<SettingsFile<State>>,
    pub config: Rc<SettingsFile<Config>>,
}

impl Settings {
    pub fn load(&self) -> anyhow::Result<()> {
        // We don't want to stop if one fails, so handle errors after everything is done
        let config_result = self.config.load().with_context(|| "Failed to load config");
        let state_result = self.state.load().with_context(|| "Failed to load state");
        config_result?;
        state_result?;
        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        #[cfg(not(target_os = "linux"))]
        let subdir_name: String = crate::APPLICATION_ID.application.to_string();
        #[cfg(target_os = "linux")]
        let subdir_name: String = crate::APPLICATION_ID.application.to_lowercase();

        let state_dir = dirs::state_dir()
            .or_else(dirs::data_local_dir)
            .expect("failed to find suitable directory for state")
            .join(&subdir_name);
        let config_dir = dirs::config_dir()
            .expect("failed to find suitable directory for config")
            .join(&subdir_name);

        Self {
            state: Rc::new(SettingsFile::new(state_dir.join("state.toml"))),
            config: Rc::new(SettingsFile::new(config_dir.join("config.toml"))),
        }
    }
}
