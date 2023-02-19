pub trait DeviceDescriptor {
    fn name(&self) -> &String;
    fn mac_address(&self) -> &String;
}
