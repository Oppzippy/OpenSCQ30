use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

pub struct Config {
    path: PathBuf,
    inner: ConfigInner,
    write_lock: Arc<Mutex<()>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigInner {
    pub preferred_language: Option<String>,
}

impl Config {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let config_toml = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => String::new(),
                _ => return Err(anyhow::Error::from(err).context("loading config file")),
            },
        };
        let config_inner: ConfigInner = toml::from_str(&config_toml)?;
        Ok(Self {
            path,
            inner: config_inner,
            write_lock: Arc::new(Mutex::new(())),
        })
    }

    pub fn get(&self) -> &ConfigInner {
        &self.inner
    }

    #[must_use]
    pub fn modify(
        &mut self,
        modify: impl Fn(&mut ConfigInner),
    ) -> oneshot::Receiver<anyhow::Result<()>> {
        (modify)(&mut self.inner);
        self.save()
    }

    #[must_use]
    fn save(&self) -> oneshot::Receiver<anyhow::Result<()>> {
        let (sender, receiver) = oneshot::channel();
        let output = match toml::to_string_pretty(&self.inner) {
            Ok(output) => output,
            Err(err) => {
                sender
                    .send(Err(err).context("serializing config file"))
                    .expect("we own the receiver at this point, so the receiver is not closed");
                return receiver;
            }
        };

        let path = self.path.clone();
        let write_lock = self.write_lock.clone();
        thread::spawn(move || {
            // TODO ensure writes occur in the correct order
            let _lock = write_lock.lock().unwrap();
            tracing::debug!("saving config file: {output:?}");
            // TODO atomic write
            let result = fs::write(path, output);
            _ = sender.send(result.context("writing to config file"));
        });
        receiver
    }
}
