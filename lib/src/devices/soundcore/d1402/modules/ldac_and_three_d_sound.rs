use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        modules::ModuleCollection,
        packet::PacketIOController,
        state_modifier::StateModifier,
        structures::{Ldac, SurroundSound},
    },
};

impl<T> ModuleCollection<T>
where
    T: Has<SurroundSound> + Has<Ldac> + Clone + Send + Sync + 'static,
{
    /// Adds the LDAC and 3D sound flags along with the logic to keep them mutually exclusive.
    pub fn add_d1402_ldac_and_three_d_sound(&mut self, packet_io: Arc<PacketIOController>) {
        let previous = Arc::new(Mutex::new(None));
        // This has to come before the flags, since afterwards the transitions are no longer visible
        self.state_modifiers
            .push(Box::new(RememberPreviousStateModifier {
                previous: previous.clone(),
            }));
        self.add_ldac(packet_io.clone());
        self.add_surround_sound(packet_io);
        self.state_modifiers
            .push(Box::new(MutuallyExclusiveStateModifier { previous }));
    }
}

type PreviousState = Arc<Mutex<Option<(Ldac, SurroundSound)>>>;

struct RememberPreviousStateModifier {
    previous: PreviousState,
}

#[async_trait]
impl<T> StateModifier<T> for RememberPreviousStateModifier
where
    T: Has<SurroundSound> + Has<Ldac> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        _target_state: &T,
    ) -> device::Result<()> {
        let state = state_sender.borrow();
        *self.previous.lock().unwrap() = Some((*state.get(), *state.get()));
        Ok(())
    }
}

/// Turning 3D sound on makes the device drop LDAC and briefly restart the Bluetooth audio
/// connection, and turning LDAC on while 3D sound is on turns 3D sound off. The official app shows
/// the same reconnect, so it's device behavior. The device doesn't send an update, so reflect it
/// in the local state. If both are turned on at once, the flags are applied in order, so 3D sound
/// wins.
struct MutuallyExclusiveStateModifier {
    previous: PreviousState,
}

#[async_trait]
impl<T> StateModifier<T> for MutuallyExclusiveStateModifier
where
    T: Has<SurroundSound> + Has<Ldac> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let Some((previous_ldac, previous_three_d_sound)) = *self.previous.lock().unwrap() else {
            return Ok(());
        };
        let target_ldac: &Ldac = target_state.get();
        let target_three_d_sound: &SurroundSound = target_state.get();
        if target_three_d_sound.0 && !previous_three_d_sound.0 {
            state_sender.send_modify(|state| *state.get_mut() = Ldac(false));
        } else if target_ldac.0 && !previous_ldac.0 {
            state_sender.send_modify(|state| *state.get_mut() = SurroundSound(false));
        }
        Ok(())
    }
}
