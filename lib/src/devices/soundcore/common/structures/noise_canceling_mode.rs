use crate::devices::soundcore::common::macros::sound_mode_enum;

sound_mode_enum!(
    pub enum NoiseCancelingMode {
        Transport = 0,
        Outdoor = 1,
        Indoor = 2,
        Custom = 3,
    }
);
