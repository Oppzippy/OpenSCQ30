use macaddr::MacAddr6;

pub trait ConnectionDescriptor {
    fn name(&self) -> &str;
    fn mac_address(&self) -> MacAddr6;
}
