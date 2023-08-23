use openscq30_lib::packets::structures::Gender as LibGender;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Gender {
    inner: LibGender,
}

impl Gender {
    #[generate_interface(constructor)]
    pub fn new(gender: u8) -> Gender {
        Self {
            inner: LibGender(gender),
        }
    }
}

impl From<LibGender> for Gender {
    fn from(inner: LibGender) -> Self {
        Self { inner }
    }
}
impl From<Gender> for LibGender {
    fn from(value: Gender) -> Self {
        value.inner
    }
}
