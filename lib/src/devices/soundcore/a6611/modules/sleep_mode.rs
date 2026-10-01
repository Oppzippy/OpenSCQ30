use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};
use tokio::sync::watch;

use crate::{
    api::{
        device,
        settings::{CategoryId, Setting, SettingId, Value},
    },
    devices::soundcore::{
        a6611::{
            packets::{inbound::BLUETOOTH_MODE_UPDATE_COMMAND, outbound::set_bluetooth_mode},
            structures::BluetoothMode,
        },
        common::{
            modules::ModuleCollection,
            packet::{self, PacketIOController},
            packet_manager::PacketHandler,
            settings_manager::{SettingHandler, SettingHandlerResult},
            state_modifier::StateModifier,
        },
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SleepModeSetting {
        SleepMode,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<BluetoothMode> + Clone + Send + Sync,
{
    pub fn add_a6611_sleep_mode(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, SleepModeSettingHandler);
        self.state_modifiers
            .push(Box::new(SleepModeStateModifier { packet_io }));
        self.packet_handlers.set_handler(
            BLUETOOTH_MODE_UPDATE_COMMAND,
            Box::new(BluetoothModeUpdateHandler),
        );
    }
}

struct SleepModeSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for SleepModeSettingHandler
where
    T: Has<BluetoothMode> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SleepModeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let _: SleepModeSetting = (*setting_id).try_into().ok()?;
        let bluetooth_mode: &BluetoothMode = state.get();
        Some(Setting::Toggle {
            value: !bluetooth_mode.0,
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let is_sleep_mode = value.try_as_bool()?;
        let bluetooth_mode: &mut BluetoothMode = state.get_mut();
        bluetooth_mode.0 = !is_sleep_mode;
        Ok(())
    }
}

struct SleepModeStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for SleepModeStateModifier
where
    T: Has<BluetoothMode> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let current: BluetoothMode = *state_sender.borrow().get();
        let target: BluetoothMode = *target_state.get();
        if current != target {
            self.packet_io
                .send_with_response(&set_bluetooth_mode(target))
                .await?;
            state_sender.send_modify(|state| *state.get_mut() = target);
        }
        Ok(())
    }
}

struct BluetoothModeUpdateHandler;

#[async_trait]
impl<T> PacketHandler<T> for BluetoothModeUpdateHandler
where
    T: Has<BluetoothMode> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        if let Some(&value) = packet.body.first() {
            let reported = BluetoothMode(value != 0);
            state.send_if_modified(|state| {
                let current: &mut BluetoothMode = state.get_mut();
                let modified = *current != reported;
                *current = reported;
                modified
            });
        }
        Ok(())
    }
}
