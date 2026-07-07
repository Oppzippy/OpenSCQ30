use crate::devices::soundcore::common::macros::sound_mode_enum;

sound_mode_enum!(
    pub enum AmbientSoundMode {
        NoiseCanceling = 0,
        Transparency = 1,
        Normal = 2,
    }
);
