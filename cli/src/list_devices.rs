use openscq30_lib::api::device::DeviceDescriptor;

pub fn list_devices(descriptors: &[impl DeviceDescriptor]) {
    println!(
        "{}",
        descriptors
            .iter()
            .map(|descriptor| descriptor.mac_address().to_owned())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
