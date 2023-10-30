package com.oppzippy.openscq30.features.soundcoredevice.api

import kotlinx.coroutines.CoroutineScope

interface SoundcoreDeviceConnector {
    suspend fun connectToSoundcoreDevice(
        macAddress: String,
        scope: CoroutineScope,
    ): SoundcoreDevice?
}
