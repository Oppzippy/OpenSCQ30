pub trait SoundcoreDeviceConnectionDescriptor {
    fn name(&self) -> &String;
    fn mac_address(&self) -> &String;
}
