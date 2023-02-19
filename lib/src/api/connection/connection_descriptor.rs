pub trait ConnectionDescriptor {
    fn name(&self) -> &String;
    fn mac_address(&self) -> &String;
}
