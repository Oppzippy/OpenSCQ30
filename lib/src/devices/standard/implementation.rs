mod ambient_sound_mode_cycle;
mod custom_button_model;
mod equalizer_configuration;
mod hear_id;
mod packet_handlers;
mod sound_modes;
mod sound_modes_type_two;

use std::collections::HashMap;

pub use ambient_sound_mode_cycle::*;
pub use custom_button_model::*;
pub use equalizer_configuration::*;
pub use hear_id::*;
pub use packet_handlers::*;
pub use sound_modes::*;
pub use sound_modes_type_two::*;

use super::{state::DeviceState, structures::*};
use crate::soundcore_device::device::device_implementation::DeviceImplementation;

pub struct StandardImplementation {
    initializer: Box<dyn Fn(&[u8]) -> crate::Result<DeviceState> + Send + Sync>,
}

impl StandardImplementation {
    pub fn new(
        initializer: Box<dyn Fn(&[u8]) -> crate::Result<DeviceState> + Send + Sync>,
    ) -> Self {
        Self { initializer }
    }
}

/// Creates a standard implementation instance using the inbound packet type's take function.
/// This is a macro rather than a generic function so that we don't have to box the returned function.
#[macro_export]
macro_rules! standard_implementation {
    ($inbound_packet_type:ident) => {
        || {
            ::std::sync::Arc::new(
                $crate::devices::standard::implementation::StandardImplementation::new(
                    ::std::boxed::Box::new(|input| {
                        $inbound_packet_type::take::<::nom::error::VerboseError<_>>(input)
                            .map(|(_, packet)| {
                                $crate::devices::standard::packets::inbound::state_update_packet::StateUpdatePacket::from(
                                    packet,
                                )
                                .into()
                            })
                            .map_err(|err| $crate::Error::ParseError {
                                message: format!("{err:?}"),
                            })
                    }),
                ),
            )
        }
    };
}

impl DeviceImplementation for StandardImplementation {
    fn packet_handlers(
        &self,
    ) -> HashMap<Command, Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        packet_handlers()
    }

    fn initialize(&self, packet: &[u8]) -> crate::Result<DeviceState> {
        (self.initializer)(packet)
    }

    fn set_sound_modes(
        &self,
        state: DeviceState,
        sound_modes: SoundModes,
    ) -> crate::Result<crate::soundcore_device::device::soundcore_command::CommandResponse> {
        set_sound_modes(state, sound_modes)
    }

    fn set_sound_modes_type_two(
        &self,
        state: DeviceState,
        sound_modes: SoundModesTypeTwo,
    ) -> crate::Result<crate::soundcore_device::device::soundcore_command::CommandResponse> {
        set_sound_modes_type_two(state, sound_modes)
    }

    fn set_ambient_sound_mode_cycle(
        &self,
        state: DeviceState,
        cycle: AmbientSoundModeCycle,
    ) -> crate::Result<crate::soundcore_device::device::soundcore_command::CommandResponse> {
        set_ambient_sound_mode_cycle(state, cycle)
    }

    fn set_equalizer_configuration(
        &self,
        state: DeviceState,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<crate::soundcore_device::device::soundcore_command::CommandResponse> {
        set_equalizer_configuration(state, equalizer_configuration)
    }

    fn set_hear_id(
        &self,
        state: DeviceState,
        hear_id: HearId,
    ) -> crate::Result<crate::soundcore_device::device::soundcore_command::CommandResponse> {
        set_hear_id(state, hear_id)
    }

    fn set_custom_button_model(
        &self,
        state: DeviceState,
        custom_button_model: CustomButtonModel,
    ) -> crate::Result<crate::soundcore_device::device::soundcore_command::CommandResponse> {
        set_custom_button_model(state, custom_button_model)
    }
}
