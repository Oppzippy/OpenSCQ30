use mockall::mock;
use openscq30_lib::api::device::DeviceDescriptor;

mock! {
    #[derive(Debug)]
    pub Descriptor {}
    impl DeviceDescriptor for Descriptor {
        fn name(&self) -> &String;
        fn mac_address(&self) -> &String;
    }
}
