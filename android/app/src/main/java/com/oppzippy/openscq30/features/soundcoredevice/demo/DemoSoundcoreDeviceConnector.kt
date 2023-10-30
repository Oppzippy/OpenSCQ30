package com.oppzippy.openscq30.features.soundcoredevice.demo

import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import kotlinx.coroutines.CoroutineScope

class DemoSoundcoreDeviceConnector : SoundcoreDeviceConnector {
    override suspend fun connectToSoundcoreDevice(
        macAddress: String,
        scope: CoroutineScope,
    ): SoundcoreDevice {
        return DemoSoundcoreDevice("Demo Q30", "00:00:00:00:00:00")
    }
}
