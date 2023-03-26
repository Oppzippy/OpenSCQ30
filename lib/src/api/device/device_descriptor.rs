pub trait DeviceDescriptor {
    fn name(&self) -> &str;
    fn mac_address(&self) -> &str;
}
