use std::fmt;

#[derive(Clone, Copy)]
pub struct EqualizerProfileId(pub [u8; 2]);

impl fmt::Display for EqualizerProfileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:02x?} {:02x?}]", self.0[0], self.0[1])
    }
}

#[cfg(test)]
mod tests {
    use super::EqualizerProfileId;

    #[test]
    fn it_displays_using_hexidecimal() {
        let profile = EqualizerProfileId([0, 15]);
        assert_eq!("[00 0f]", profile.to_string());
    }
}
