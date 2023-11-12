use btleplug::api::BDAddr;
use macaddr::MacAddr6;

// We can't implement From<BDAddr> for MacAddr6 since both BDAddr and MacAddr6
// are from external crates.
pub trait IntoMacAddr {
    fn into_mac_addr(self) -> MacAddr6;
}

impl IntoMacAddr for BDAddr {
    fn into_mac_addr(self) -> MacAddr6 {
        self.into_inner().into()
    }
}

pub trait IntoBDAddr {
    fn into_bd_addr(self) -> BDAddr;
}

impl IntoBDAddr for MacAddr6 {
    fn into_bd_addr(self) -> BDAddr {
        self.into_array().into()
    }
}
