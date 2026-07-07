use crate::devices::soundcore::common::macros::sound_mode_enum;

sound_mode_enum!(
    pub enum TransparencyMode {
        FullyTransparent = 0,
        VocalMode = 1,
    }
);
