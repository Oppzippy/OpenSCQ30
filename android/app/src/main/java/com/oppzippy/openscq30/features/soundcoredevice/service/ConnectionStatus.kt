package com.oppzippy.openscq30.features.soundcoredevice.service

sealed class ConnectionStatus {
    data object AwaitingConnection : ConnectionStatus()
    class Connecting(val macAddress: String) : ConnectionStatus()
    class Connected(val deviceManager: DeviceConnectionManager) : ConnectionStatus()
    data object Disconnected : ConnectionStatus()
}
