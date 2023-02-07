package com.oppzippy.openscq30.features.soundcoredevice.api

import kotlinx.coroutines.CoroutineScope

interface SoundcoreDeviceFactory {
    suspend fun createSoundcoreDevice(
        macAddress: String,
        scope: CoroutineScope,
    ): SoundcoreDevice?
}