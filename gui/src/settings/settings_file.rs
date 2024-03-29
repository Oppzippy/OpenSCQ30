use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    sync::RwLock,
};

use anyhow::Context;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct SettingsFile<SettingsStateType> {
    settings_file_path: PathBuf,
    state: RwLock<SettingsStateType>,
}

impl<SettingsStateType> SettingsFile<SettingsStateType>
where
    SettingsStateType: Default,
    SettingsStateType: DeserializeOwned,
    SettingsStateType: Serialize,
{
    pub fn new(path: PathBuf) -> Self {
        Self {
            settings_file_path: path,
            state: RwLock::new(SettingsStateType::default()),
        }
    }

    pub fn load(&self) -> anyhow::Result<()> {
        let mut file = self.get_file(Mode::Read).context("get config file")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .context("read from config file")?;
        let settings =
            toml::from_str::<SettingsStateType>(&buffer).context("parse toml config file")?;

        self.edit(|state| *state = settings)
            .context("update state")?;
        Ok(())
    }

    pub fn edit<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut SettingsStateType),
    {
        let mut state = self
            .state
            .write()
            .map_err(|err| anyhow::anyhow!("failed to write rwlock: {err}"))?;
        f(&mut state);
        self.save(&state)?;
        Ok(())
    }

    fn save(&self, state: &SettingsStateType) -> anyhow::Result<()> {
        let mut file = self
            .get_file(Mode::Write)
            .context("open file for writing")?;
        let toml_string = toml::to_string(state).context("serializing as toml")?;
        file.write_all(toml_string.as_bytes())
            .context("write toml bytes to file")?;

        Ok(())
    }

    pub fn get<F, T>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(&SettingsStateType) -> T,
    {
        let state = self
            .state
            .read()
            .map_err(|err| anyhow::anyhow!("failed to read from rwlock: {err}"))?;
        Ok(f(&state))
    }

    fn get_file(&self, mode: Mode) -> anyhow::Result<File> {
        let dir = self.settings_file_path.parent().with_context(|| {
            format!(
                "settings file has no parent directory: {}",
                self.settings_file_path.to_string_lossy()
            )
        })?;
        fs::create_dir_all(dir)
            .with_context(|| format!("create directories in path {}", dir.to_string_lossy()))?;

        let mut options = OpenOptions::new();
        let options = match mode {
            Mode::Read => options.read(true),
            Mode::Write => options.write(true).create(true).truncate(true),
        };
        let file = options.open(&self.settings_file_path).with_context(|| {
            format!(
                "open file {} for {:?}",
                self.settings_file_path.to_string_lossy(),
                mode,
            )
        })?;
        Ok(file)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Read,
    Write,
}
