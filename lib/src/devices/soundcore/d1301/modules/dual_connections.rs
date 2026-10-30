//! The D1301 supports the standard dual connections commands (0x0b family), but unlike other models
//! it neither reports whether dual connections is enabled nor remembers it after powering off. To
//! make up for that, the last value set through openscq30 is stored and reapplied on connect.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{device, settings::SettingId},
    devices::{
        DeviceModel,
        soundcore::{
            common::{
                device::SoundcoreDeviceBuilder,
                modules::ModuleCollection,
                packet,
                state_modifier::StateModifier,
                structures::{DualConnections, DualConnectionsDevice},
            },
            d1301::state::D1301State,
        },
    },
    storage::OpenSCQ30Database,
};

/// The D1301 does not report whether dual connections is enabled, but if more than one device is
/// connected, it must be.
pub fn initial_dual_connections(devices: Vec<DualConnectionsDevice>) -> DualConnections {
    DualConnections {
        is_enabled: devices.iter().filter(|device| device.is_connected).count() > 1,
        devices,
    }
}

impl SoundcoreDeviceBuilder<D1301State> {
    pub async fn d1301_dual_connections(&mut self) {
        let database = self.database();
        let device_model = self.device_model();

        let saved = match database
            .fetch_device_model_setting(device_model, SettingId::DualConnections)
            .await
        {
            Ok(value) => value.and_then(|value| value.as_bool()),
            Err(err) => {
                tracing::warn!("failed to fetch saved dual connections setting: {err:?}");
                None
            }
        };
        let is_enabled = Has::<DualConnections>::get(&*self.state_sender().borrow()).is_enabled;
        if saved == Some(true) && !is_enabled {
            match self
                .packet_io_controller()
                .send_with_response(&packet::outbound::set_dual_connections_enabled(true))
                .await
            {
                Ok(_) => self.state_sender().send_modify(|state| {
                    Has::<DualConnections>::get_mut(state).is_enabled = true;
                }),
                Err(err) => tracing::warn!("failed to reapply dual connections: {err:?}"),
            }
        }

        self.dual_connections();
        self.module_collection()
            .add_d1301_dual_connections_persistence(database, device_model, saved);
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<DualConnections> + Clone + Send + Sync,
{
    /// Must be added after the dual connections module so that only values that were successfully
    /// sent to the device get saved.
    fn add_d1301_dual_connections_persistence(
        &mut self,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        saved: Option<bool>,
    ) {
        self.state_modifiers
            .push(Box::new(PersistDualConnectionsStateModifier {
                database,
                device_model,
                saved: Mutex::new(saved),
            }));
    }
}

struct PersistDualConnectionsStateModifier {
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
    /// Compared against the database's value rather than the device's state, since the state can
    /// already be enabled (two devices connected) without that ever having been saved.
    saved: Mutex<Option<bool>>,
}

#[async_trait]
impl<StateT> StateModifier<StateT> for PersistDualConnectionsStateModifier
where
    StateT: Has<DualConnections> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        let is_enabled = target_state.get().is_enabled;
        // Only save what the dual connections module actually applied to the device.
        if state_sender.borrow().get().is_enabled != is_enabled
            || *self.saved.lock().unwrap() == Some(is_enabled)
        {
            return Ok(());
        }
        // Failing to save shouldn't fail setting the value on the device, which already happened.
        match self
            .database
            .upsert_device_model_setting(
                self.device_model,
                SettingId::DualConnections,
                is_enabled.into(),
            )
            .await
        {
            Ok(()) => *self.saved.lock().unwrap() = Some(is_enabled),
            Err(err) => tracing::warn!("failed to save dual connections setting: {err:?}"),
        }
        Ok(())
    }
}
