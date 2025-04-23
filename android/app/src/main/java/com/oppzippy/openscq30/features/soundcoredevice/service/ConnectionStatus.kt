package com.oppzippy.openscq30.features.soundcoredevice.service

import kotlinx.coroutines.Job

sealed class ConnectionStatus {
    data object AwaitingConnection : ConnectionStatus()
    class Connecting(val macAddress: String, val job: Job) : ConnectionStatus()
    class Connected(val deviceManager: DeviceConnectionManager) : ConnectionStatus()
    data object Disconnected : ConnectionStatus()
}
