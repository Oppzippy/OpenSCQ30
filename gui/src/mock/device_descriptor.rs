use macaddr::MacAddr6;
use mockall::mock;
use openscq30_lib::api::device::DeviceDescriptor;

mock! {
    #[derive(Debug)]
    pub Descriptor {}
    impl DeviceDescriptor for Descriptor {
        fn name(&self) -> &str;
        fn mac_address(&self) -> MacAddr6;
    }
}
