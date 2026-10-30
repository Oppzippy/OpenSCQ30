use std::sync::Arc;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{
        device,
        settings::{CategoryId, Setting, SettingId, Value},
    },
    devices::soundcore::common::{
        modules::ModuleCollection,
        packet::{
            self, PacketIOController,
            inbound::{FromPacketBody, TryToPacket},
            parsing::take_bool,
        },
        packet_manager::PacketHandler,
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        state_modifier::StateModifier,
        structures::{Ldac, SurroundSound},
    },
};

const LDAC_SET_COMMAND: packet::Command = packet::Command([0x01, 0xFF]);
const LDAC_UPDATE_COMMAND: packet::Command = packet::Command([0x01, 0x7F]);

impl<T> ModuleCollection<T>
where
    T: Has<SurroundSound> + Has<Ldac> + Clone + Send + Sync + 'static,
{
    /// Adds the LDAC and 3D sound settings along with the logic to keep them mutually exclusive.
    pub fn add_d1402_ldac_and_three_d_sound(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, LdacAndThreeDSoundSettingHandler);
        self.packet_handlers
            .set_handler(LDAC_UPDATE_COMMAND, Box::new(LdacPacketHandler));
        self.state_modifiers
            .push(Box::new(LdacAndThreeDSoundStateModifier { packet_io }));
    }
}

pub struct LdacAndThreeDSoundSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for LdacAndThreeDSoundSettingHandler
where
    T: Has<SurroundSound> + Has<Ldac> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::Ldac, SettingId::SurroundSound]
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let value = match setting_id {
            SettingId::Ldac => {
                let ldac: &Ldac = state.get();
                ldac.0
            }
            SettingId::SurroundSound => {
                let three_d_sound: &SurroundSound = state.get();
                three_d_sound.0
            }
            _ => return None,
        };
        Some(Setting::Toggle { value })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let enabled = value.try_as_bool()?;
        match setting_id {
            SettingId::Ldac => {
                let ldac: &mut Ldac = state.get_mut();
                ldac.0 = enabled;
            }
            SettingId::SurroundSound => {
                let three_d_sound: &mut SurroundSound = state.get_mut();
                three_d_sound.0 = enabled;
            }
            _ => return Err(SettingHandlerError::DoesNotExist),
        }
        Ok(())
    }
}

struct LdacPacketHandler;

#[async_trait]
impl<T> PacketHandler<T> for LdacPacketHandler
where
    T: Has<Ldac> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let update: LdacUpdate = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            let ldac: &mut Ldac = state.get_mut();
            let modified = ldac.0 != update.0;
            ldac.0 = update.0;
            modified
        });
        Ok(())
    }
}

struct LdacUpdate(bool);

impl FromPacketBody for LdacUpdate {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("ldac update", all_consuming(map(take_bool, Self))).parse_complete(input)
    }
}

/// LDAC and 3D sound can't both be on. Turning 3D sound on makes the device drop LDAC and briefly
/// restart the Bluetooth audio connection, and turning LDAC on turns 3D sound off. The official app
/// shows the same reconnect, so it's device behavior. The device doesn't send an update for the
/// flag it turns off, so it's reflected in the local state here. When both are requested on at
/// once, 3D sound wins and LDAC isn't sent, avoiding a needless extra Bluetooth audio restart.
struct LdacAndThreeDSoundStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl LdacAndThreeDSoundStateModifier {
    async fn send_flag(&self, command: packet::Command, enabled: bool) -> device::Result<()> {
        self.packet_io
            .send_with_response(&packet::Outbound::new(command, vec![enabled.into()]))
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<T> StateModifier<T> for LdacAndThreeDSoundStateModifier
where
    T: Has<SurroundSound> + Has<Ldac> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let (current_ldac, current_three_d_sound) = {
            let state = state_sender.borrow();
            let ldac: &Ldac = state.get();
            let three_d_sound: &SurroundSound = state.get();
            (*ldac, *three_d_sound)
        };
        let target_ldac: &Ldac = target_state.get();
        let target_three_d_sound: &SurroundSound = target_state.get();
        if *target_ldac == current_ldac && *target_three_d_sound == current_three_d_sound {
            return Ok(());
        }

        if target_three_d_sound.0 && !current_three_d_sound.0 {
            self.send_flag(packet::outbound::SET_SURROUND_SOUND_COMMAND, true)
                .await?;
            state_sender.send_modify(|state| {
                *state.get_mut() = SurroundSound(true);
                *state.get_mut() = Ldac(false);
            });
        } else if target_ldac.0 && !current_ldac.0 {
            self.send_flag(LDAC_SET_COMMAND, true).await?;
            state_sender.send_modify(|state| {
                *state.get_mut() = Ldac(true);
                *state.get_mut() = SurroundSound(false);
            });
        } else {
            if current_three_d_sound.0 && !target_three_d_sound.0 {
                self.send_flag(packet::outbound::SET_SURROUND_SOUND_COMMAND, false)
                    .await?;
                state_sender.send_modify(|state| *state.get_mut() = SurroundSound(false));
            }
            if current_ldac.0 && !target_ldac.0 {
                self.send_flag(LDAC_SET_COMMAND, false).await?;
                state_sender.send_modify(|state| *state.get_mut() = Ldac(false));
            }
        }
        Ok(())
    }
}
