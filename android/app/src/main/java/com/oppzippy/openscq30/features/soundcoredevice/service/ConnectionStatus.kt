package com.oppzippy.openscq30.features.soundcoredevice.service

import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import kotlinx.coroutines.Job

sealed class ConnectionStatus {
    object AwaitingConnection : ConnectionStatus()
    class Connecting(val macAddress: String, val job: Job) : ConnectionStatus()
    class Connected(val device: SoundcoreDevice) : ConnectionStatus()
    object Disconnected : ConnectionStatus()
}
