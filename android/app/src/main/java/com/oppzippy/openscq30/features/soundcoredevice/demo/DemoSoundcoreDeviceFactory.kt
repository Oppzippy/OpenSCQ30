package com.oppzippy.openscq30.features.soundcoredevice.demo

import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import kotlinx.coroutines.CoroutineScope

class DemoSoundcoreDeviceFactory : SoundcoreDeviceFactory {
    override suspend fun createSoundcoreDevice(macAddress: String, scope: CoroutineScope): SoundcoreDevice {
        return DemoSoundcoreDevice("Demo Q30", "00:00:00:00:00:00")
    }
}
