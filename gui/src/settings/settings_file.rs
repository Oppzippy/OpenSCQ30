use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    sync::RwLock,
};

use anyhow::Context;
use serde::{de::DeserializeOwned, Serialize};
use tempfile::NamedTempFile;

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
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            settings_file_path: path.into(),
            state: RwLock::new(SettingsStateType::default()),
        }
    }

    pub fn load(&self) -> anyhow::Result<()> {
        let buffer = self.read_file()?;
        let settings =
            toml::from_str::<SettingsStateType>(&buffer).context("parse toml config file")?;

        let mut state = self
            .state
            .write()
            .map_err(|err| anyhow::anyhow!("failed to write rwlock: {err}"))?;
        *state = settings;
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
        let toml_string = toml::to_string(state).context("serializing as toml")?;
        self.write_file(&toml_string)?;
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

    fn read_file(&self) -> anyhow::Result<String> {
        let mut options = OpenOptions::new();
        options.read(true);
        let mut file = options.open(&self.settings_file_path).with_context(|| {
            format!(
                "open file {} for reading",
                self.settings_file_path.to_string_lossy(),
            )
        })?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .context("reading settings file")?;
        Ok(buffer)
    }

    fn write_file(&self, content: &str) -> anyhow::Result<()> {
        let dir = self.settings_file_path.parent().with_context(|| {
            format!(
                "settings file has no parent directory: {}",
                self.settings_file_path.to_string_lossy()
            )
        })?;
        fs::create_dir_all(dir)
            .with_context(|| format!("create directories in path {}", dir.to_string_lossy()))?;

        let permissions = match self.settings_file_path.metadata() {
            Ok(metadata) => Some(metadata.permissions()),
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    tracing::warn!(
                        "failed to retrieve settings file permissions for {}: {err:?}",
                        self.settings_file_path.to_string_lossy()
                    );
                }
                None
            }
        };

        let mut file = NamedTempFile::new_in(&dir).with_context(|| {
            format!(
                "create temp file in same dir as settings file to write to before persisting: {}",
                dir.to_string_lossy()
            )
        })?;
        if let Some(permissions) = permissions {
            if let Err(err) = fs::set_permissions(file.path(), permissions) {
                tracing::warn!(
                    "failed to set settings file permissions for {}, continuing anyway: {err:?}",
                    file.path().to_string_lossy()
                );
            }
        }
        file.write_all(content.as_bytes())?;
        file.persist(&self.settings_file_path).with_context(|| {
            format!(
                "persisting temporary file to {}",
                &self.settings_file_path.to_string_lossy(),
            )
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SettingsFile;
    use serde::{Deserialize, Serialize};
    use std::fs::{self, Permissions};
    #[cfg(target_family = "unix")]
    use std::os::unix::{
        self,
        fs::{MetadataExt, PermissionsExt},
    };
    use tempfile::tempdir;

    #[derive(Serialize, Deserialize, Clone, Default)]
    struct TestConfig {
        pub number: i32,
    }

    #[test]
    fn it_creates_file_and_parent_dirs() {
        let file_dir = tempdir().unwrap();
        let file_path = file_dir.path().join("a").join("b").join("config.toml");
        let settings_file = SettingsFile::<TestConfig>::new(&file_path);
        settings_file.save(&TestConfig { number: 0 }).unwrap();

        assert!(file_path.is_file(), "file should have been created");
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn it_retains_file_permissions() {
        let file_dir = tempdir().unwrap();
        let file_path = file_dir.path().join("config.toml");
        fs::write(&file_path, "number = 0").unwrap();
        fs::set_permissions(&file_path, Permissions::from_mode(0o707)).unwrap();
        let settings_file = SettingsFile::<TestConfig>::new(&file_path);
        settings_file.save(&TestConfig { number: 0 }).unwrap();

        assert_eq!(file_path.metadata().unwrap().mode() & 0o777, 0o707);
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn it_overwrites_symlinks() {
        let file_dir = tempdir().unwrap();
        let link_dir = tempdir().unwrap();
        let file_path = file_dir.path().join("config.toml");
        let link_path = link_dir.path().join("config.toml");
        fs::write(&file_path, "number = 0").unwrap();
        unix::fs::symlink(&file_path, &link_path).unwrap();

        let settings_file = SettingsFile::<TestConfig>::new(&link_path);
        settings_file.save(&TestConfig { number: 1 }).unwrap();
        settings_file.load().unwrap();

        assert!(link_path.is_file(), "symlink should now be a file");
        assert!(
            !link_path.is_symlink(),
            "symlink should no longer be a symlink"
        );
        assert!(file_path.is_file(), "file should still exist");
        assert!(!file_path.is_symlink(), "file should not be a symlink");
        assert_eq!(
            1,
            settings_file.get(|config| config.number).unwrap(),
            "file should have been written to"
        );
    }
}
