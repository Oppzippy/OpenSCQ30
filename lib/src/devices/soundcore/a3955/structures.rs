mod sound_modes;

use openscq30_i18n_macros::Translate;
pub use sound_modes::*;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::common;

common::structures::flag!(AncPersonalizedToEarCanal);

#[derive(
    Debug,
    Default,
    Eq,
    PartialEq,
    Clone,
    Copy,
    EnumIter,
    EnumString,
    IntoStaticStr,
    FromRepr,
    Translate,
)]
#[repr(u8)]
pub enum ImmersiveExperience {
    #[default]
    Disabled = 0,
    GamingMode = 1,
    MovieMode = 2,
}
