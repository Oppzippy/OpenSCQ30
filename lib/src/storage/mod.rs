mod migration;
mod paired_device;
mod quick_preset;
mod type_conversions;

use std::{
    collections::HashMap,
    mem,
    panic::Location,
    path::PathBuf,
    sync::{mpsc, Arc},
    thread,
};

use macaddr::MacAddr6;
use rusqlite::{ffi::SQLITE_CONSTRAINT_UNIQUE, Connection};
use thiserror::Error;
use tokio::sync::{oneshot, Semaphore};
use tracing::info_span;

use crate::{
    api::settings::{self, SettingId},
    soundcore_device::device_model::DeviceModel,
};

// This needs to be Send + Sync, and rusqlite::Connection is not, so we have to spawn a new thread
// that owns the connection and communicate with it over a channel.
#[derive(Debug)]
pub struct OpenSCQ30Database {
    command_sender: mpsc::Sender<Command>,
    closed: Arc<Semaphore>,
}
#[derive(Clone, Debug)]
pub struct PairedDevice {
    pub name: String,
    pub mac_address: MacAddr6,
    pub model: DeviceModel,
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("sql error: {0:?}")]
    AlreadyExists(rusqlite::Error),
    #[error("sql error: {0:?}")]
    Other(rusqlite::Error),
    #[error("failed to deserialize json: {0:?}")]
    JsonError(serde_json::Error),
}

impl From<StorageError> for crate::Error {
    #[track_caller]
    fn from(value: StorageError) -> Self {
        Self::Other {
            source: Box::new(value),
            location: Location::caller(),
        }
    }
}

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        if let Some(sqlite_err) = err.sqlite_error() {
            if sqlite_err.extended_code == SQLITE_CONSTRAINT_UNIQUE {
                return StorageError::AlreadyExists(err);
            }
        }
        StorageError::Other(err)
    }
}

impl OpenSCQ30Database {
    pub async fn new(path: PathBuf) -> Result<Self, StorageError> {
        let (init_error_sender, init_error_receiver) = oneshot::channel();
        let (command_sender, command_receiver) = mpsc::channel::<Command>();

        let closed = Arc::new(Semaphore::new(0));
        {
            let closed = closed.clone();
            thread::spawn(move || {
                let span = info_span!("OpenSCQ30Database");
                let _guard = span.enter();
                let mut connection = match Connection::open(path) {
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
                closed.close()
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
    paired_device::fetch_all => fn fetch_all_paired_devices() -> Result<Vec<PairedDevice>, StorageError>;
    paired_device::fetch => fn fetch_paired_device(mac_address: MacAddr6) -> Result<Option<PairedDevice>, StorageError>;
    paired_device::insert => fn insert_paired_device(paired_device: PairedDevice) -> Result<(), StorageError>;
    paired_device::upsert => fn upsert_paired_device(paired_device: PairedDevice) -> Result<(), StorageError>;
    paired_device::delete => fn delete_paired_device(mac_address: MacAddr6) -> Result<(), StorageError>;
    quick_preset::fetch => fn fetch_quick_preset(
        model: DeviceModel,
        name: String,
    ) -> Result<HashMap<SettingId<'static>, settings::Value>, StorageError>;
    quick_preset::fetch_all => fn fetch_all_quick_presets(
        model: DeviceModel,
    ) -> Result<HashMap<String, HashMap<SettingId<'static>, settings::Value>>, StorageError>;
    quick_preset::upsert => fn upsert_quick_preset(
        model: DeviceModel,
        name: String,
        settings: HashMap<SettingId<'static>, settings::Value>,
    ) -> Result<(), StorageError>;
    quick_preset::delete => fn delete_quick_preset(model: DeviceModel, name: String) -> Result<(), StorageError>;
);
