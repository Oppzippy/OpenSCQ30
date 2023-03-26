pub trait ConnectionDescriptor {
    fn name(&self) -> &str;
    fn mac_address(&self) -> &str;
}
