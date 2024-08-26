package com.oppzippy.openscq30.features.soundcoredevice.api

import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDevice
import kotlinx.coroutines.CoroutineScope

interface SoundcoreDeviceConnector {
    suspend fun connectToSoundcoreDevice(macAddress: String, coroutineScope: CoroutineScope): SoundcoreDevice?
}
