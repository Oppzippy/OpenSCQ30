mod equalizer_profile;
mod migration;
mod paired_device;
mod quick_preset;
mod type_conversions;

use std::{
    mem,
    panic::Location,
    path::PathBuf,
    sync::{Arc, mpsc},
    thread,
};

use macaddr::MacAddr6;
use rusqlite::{Connection, ffi::SQLITE_CONSTRAINT_UNIQUE};
use thiserror::Error;
use tokio::sync::{Semaphore, oneshot};
use tracing::info_span;

use crate::{
    api::settings::SettingId, devices::DeviceModel, macros::impl_from_source_error_with_location,
};

pub use paired_device::PairedDevice;
pub use quick_preset::{QuickPreset, QuickPresetField};

// This needs to be Send + Sync, and rusqlite::Connection is not, so we have to spawn a new thread
// that owns the connection and communicate with it over a channel.
#[derive(Debug)]
pub struct OpenSCQ30Database {
    command_sender: mpsc::Sender<Command>,
    closed: Arc<Semaphore>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("already exists")]
    AlreadyExists { source: rusqlite::Error },
    #[error("not found")]
    NotFound {
        location: &'static Location<'static>,
    },
    #[error("io error")]
    IOError {
        source: std::io::Error,
        location: &'static Location<'static>,
    },
    #[error("sql error")]
    RusqliteError {
        source: rusqlite::Error,
        location: &'static Location<'static>,
    },
    #[error("sql error")]
    RusqliteFromSqlError {
        source: rusqlite::types::FromSqlError,
        location: &'static Location<'static>,
    },
    #[error("failed to deserialize json")]
    JsonError {
        source: serde_json::Error,
        location: &'static Location<'static>,
    },
    #[error("failed to parse value")]
    ParseError {
        source: strum::ParseError,
        location: &'static Location<'static>,
    },
}
pub type Result<T> = std::result::Result<T, Error>;

impl_from_source_error_with_location!(Error::JsonError(serde_json::Error));
impl_from_source_error_with_location!(Error::ParseError(strum::ParseError));
impl_from_source_error_with_location!(Error::RusqliteFromSqlError(rusqlite::types::FromSqlError));
impl_from_source_error_with_location!(Error::IOError(std::io::Error));

impl From<rusqlite::Error> for Error {
    #[track_caller]
    fn from(err: rusqlite::Error) -> Self {
        if let Some(sqlite_err) = err.sqlite_error()
            && sqlite_err.extended_code == SQLITE_CONSTRAINT_UNIQUE
        {
            return Self::AlreadyExists { source: err };
        }
        Self::RusqliteError {
            source: err,
            location: Location::caller(),
        }
    }
}

impl OpenSCQ30Database {
    pub async fn new_file(path: PathBuf) -> Result<Self> {
        if let Some(parent_dir) = path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        Self::new(|| Connection::open(path)).await
    }

    pub async fn new_in_memory() -> Result<Self> {
        Self::new(Connection::open_in_memory).await
    }

    async fn new(
        open_connection: impl FnOnce() -> rusqlite::Result<Connection> + Send + 'static,
    ) -> Result<Self> {
        let (init_error_sender, init_error_receiver) = oneshot::channel();
        let (command_sender, command_receiver) = mpsc::channel::<Command>();

        let closed = Arc::new(Semaphore::new(0));
        {
            let closed = closed.clone();
            thread::spawn(move || {
                let span = info_span!("OpenSCQ30Database");
                let _guard = span.enter();
                let mut connection = match open_connection() {
                    Ok(connection) => connection,
                    Err(err) => {
                        let _ = init_error_sender.send(err);
                        return;
                    }
                };
                if let Err(err) = connection.pragma_update(None, "foreign_keys", "ON") {
                    // foreign keys won't be checked, but we can proceed
                    tracing::warn!("failed to enable sqlite foreign key support: {err:?}");
                }
                match migration::migrate(&mut connection, migration::MIGRATIONS) {
                    Ok(()) => (),
                    Err(err) => {
                        let _ = init_error_sender.send(err);
                        return;
                    }
                };
                mem::drop(init_error_sender);
                command_receiver
                    .iter()
                    .for_each(|command| Self::handle_command(&mut connection, command));

                // clean up resources before allowing Self to be dropped
                tracing::trace!("cleaning up resources");
                mem::drop(connection);
                tracing::trace!("dropped sqlite connection, closing semaphore");
                closed.close();
            });
        }
        if let Ok(err) = init_error_receiver.await {
            return Err(err.into());
        };
        Ok(Self {
            command_sender,
            closed,
        })
    }
}

impl Drop for OpenSCQ30Database {
    fn drop(&mut self) {
        // in order to drop the sender, we need something to replace it with
        let (temp_sender, _) = mpsc::channel();
        self.command_sender = temp_sender;

        // wait for semaphore to close
        tracing::trace!("OpenSCQ30Database: waiting for closed semaphore to drop");
        let _ = futures::executor::block_on(self.closed.acquire());
        tracing::trace!("OpenSCQ30Database: dropping");
    }
}

macro_rules! commands {
    (
        $($handler:path => fn $fn_name:ident($($arg:ident: $arg_type:ty$(,)?)*) -> $return:ty;)*
    ) => {
        #[allow(non_camel_case_types)]
        enum Command {
            $(
                $fn_name {
                    $($arg: $arg_type,)*
                    result_sender: ::tokio::sync::oneshot::Sender<$return>,
                },
            )*
        }

        // Send errors are ignored since it's fine if the caller closes the receive channel, since that means
        // they are no longer .awaiting
        impl OpenSCQ30Database {
            fn handle_command(connection: &mut ::rusqlite::Connection, command: Command) {
                match command {
                    $(
                        Command::$fn_name { $($arg,)* result_sender } => {
                            let _ = result_sender.send($handler(connection, $($arg,)*));
                        }
                    )*
                }
            }

            $(
                pub async fn $fn_name(
                    &self, $($arg: $arg_type,)*
                ) -> $return {
                    let (result_sender, result_receiver) = ::tokio::sync::oneshot::channel();
                    self.command_sender
                        .send(Command::$fn_name { $($arg,)* result_sender })
                        .expect("receiver shouldn't be dropped until self is dropped");
                    result_receiver.await.expect("abort should be impossible")
                }
            )*
        }
    };
}

commands!(
    paired_device::fetch_all => fn fetch_all_paired_devices() -> Result<Vec<PairedDevice>>;
    paired_device::fetch => fn fetch_paired_device(mac_address: MacAddr6) -> Result<Option<PairedDevice>>;
    paired_device::insert => fn insert_paired_device(paired_device: PairedDevice) -> Result<()>;
    paired_device::upsert => fn upsert_paired_device(paired_device: PairedDevice) -> Result<()>;
    paired_device::delete => fn delete_paired_device(mac_address: MacAddr6) -> Result<()>;
    paired_device::update_last_connected => fn update_last_connected_device(mac_address: MacAddr6) -> Result<()>;
    paired_device::fetch_last_connected => fn fetch_last_connected_device() -> Result<Option<PairedDevice>>;
    quick_preset::fetch => fn fetch_quick_preset(
        model: DeviceModel,
        name: String,
    ) -> Result<QuickPreset>;
    quick_preset::fetch_all => fn fetch_all_quick_presets(
        model: DeviceModel,
    ) -> Result<Vec<QuickPreset>>;
    quick_preset::upsert => fn upsert_quick_preset(
        model: DeviceModel,
        quick_preset: QuickPreset,
    ) -> Result<()>;
    quick_preset::toggle_field => fn toggle_quick_preset_field(
        model: DeviceModel,
        name: String,
        setting_id: SettingId,
        is_enabled: bool,
    ) -> Result<()>;
    quick_preset::delete => fn delete_quick_preset(model: DeviceModel, name: String) -> Result<()>;
    equalizer_profile::fetch => fn fetch_equalizer_profile(
        model: DeviceModel,
        name: String,
    ) -> Result<Vec<i16>>;
    equalizer_profile::fetch_all => fn fetch_all_equalizer_profiles(
        model: DeviceModel,
    ) -> Result<Vec<(String, Vec<i16>)>>;
    equalizer_profile::upsert => fn upsert_equalizer_profile(
        model: DeviceModel,
        name: String,
        volume_adjustments: Vec<i16>,
    ) -> Result<()>;
    equalizer_profile::bulk_upsert => fn upsert_equalizer_profiles(
        model: DeviceModel,
        name: Vec<(String, Vec<i16>)>,
    ) -> Result<()>;
    equalizer_profile::delete => fn delete_equalizer_profile(model: DeviceModel, name: String) -> Result<()>;
);
