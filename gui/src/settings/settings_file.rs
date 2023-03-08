use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    sync::RwLock,
};

use anyhow::Context;

use super::settings_state::SettingsState;

#[derive(Debug)]
pub struct SettingsFile {
    settings_directory_path: PathBuf,
    state: RwLock<SettingsState>,
}

impl SettingsFile {
    pub fn new(path: PathBuf) -> Self {
        Self {
            settings_directory_path: path,
            state: RwLock::new(SettingsState::default()),
        }
    }

    pub fn load(&self) -> anyhow::Result<()> {
        let mut file = self.get_file(Mode::READ).context("get config file")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .context("read from config file")?;
        let settings =
            toml::from_str::<SettingsState>(&buffer).context("parse toml config file")?;

        self.edit(|state| *state = settings)
            .context("update state")?;
        Ok(())
    }

    pub fn edit<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut SettingsState),
    {
        let mut state = self
            .state
            .write()
            .map_err(|err| anyhow::anyhow!("failed to write rwlock: {err}"))?;
        f(&mut state);
        self.save(&state)?;
        Ok(())
    }

    fn save(&self, state: &SettingsState) -> anyhow::Result<()> {
        let mut file = self.get_file(Mode::WRITE)?;
        let toml_string = toml::to_string(state)?;
        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }

    pub fn get<F, T>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(&SettingsState) -> T,
    {
        let state = self
            .state
            .read()
            .map_err(|err| anyhow::anyhow!("failed to read from rwlock: {err}"))?;
        Ok(f(&*state))
    }

    fn get_file(&self, mode: Mode) -> anyhow::Result<File> {
        let dir = self.settings_directory_path.parent().with_context(|| {
            format!(
                "settings file has no parent directory: {}",
                self.settings_directory_path.to_string_lossy()
            )
        })?;
        fs::create_dir_all(dir)
            .with_context(|| format!("create directories in path {}", dir.to_string_lossy()))?;

        let mut options = OpenOptions::new();
        let options = match mode {
            Mode::READ => options.read(true),
            Mode::WRITE => options.write(true).create(true).truncate(true),
        };
        let file = options
            .open(&self.settings_directory_path)
            .with_context(|| {
                format!(
                    "open file {} for {:?}",
                    self.settings_directory_path.to_string_lossy(),
                    mode,
                )
            })?;
        Ok(file)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    READ,
    WRITE,
}
