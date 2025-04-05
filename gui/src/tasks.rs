async fn connect_to_paired_device(paired_device: PairedDevice) -> Result<Message, anyhow::Error> {
    let registry = paired_device
        .model
        .device_registry::<openscq30_lib::futures::TokioFutures>(
            Some(tokio::runtime::Handle::current()),
            true,
        )
        .await?;
    let device = registry.connect(paired_device.mac_address).await?;

    Ok(Message::ConnectToDeviceScreen(DebugOpenSCQ30Device(device)))
}
