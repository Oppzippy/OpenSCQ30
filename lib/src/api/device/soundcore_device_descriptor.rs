pub trait SoundcoreDeviceDescriptor {
    fn name(&self) -> &String;
    fn mac_address(&self) -> &String;
}
