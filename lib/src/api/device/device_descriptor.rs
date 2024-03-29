use macaddr::MacAddr6;

pub trait DeviceDescriptor {
    fn name(&self) -> &str;
    fn mac_address(&self) -> MacAddr6;
}
