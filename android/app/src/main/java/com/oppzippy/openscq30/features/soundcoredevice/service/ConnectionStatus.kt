package com.oppzippy.openscq30.features.soundcoredevice.service

import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDevice
import kotlinx.coroutines.Job

sealed class ConnectionStatus {
    data object AwaitingConnection : ConnectionStatus()
    class Connecting(val macAddress: String, val job: Job) : ConnectionStatus()
    class Connected(val device: SoundcoreDevice) : ConnectionStatus()
    data object Disconnected : ConnectionStatus()
}
