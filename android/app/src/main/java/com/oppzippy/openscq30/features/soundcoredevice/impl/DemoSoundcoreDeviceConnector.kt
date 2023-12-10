package com.oppzippy.openscq30.features.soundcoredevice.impl

import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.lib.bindings.newDemoSoundcoreDevice
import kotlinx.coroutines.CoroutineScope
import java.util.UUID

class DemoSoundcoreDeviceConnector : SoundcoreDeviceConnector {
    @Throws(SecurityException::class)
    override suspend fun connectToSoundcoreDevice(
        macAddress: String,
        coroutineScope: CoroutineScope,
    ): SoundcoreDevice {
        val nativeDevice = newDemoSoundcoreDevice("Demo Device", macAddress)
        return SoundcoreDevice(
            name = nativeDevice.name(),
            macAddress = nativeDevice.macAddress(),
            // SoundcoreDeviceCallbackHandler should have already received a packet by the time we reach
            // this point, so serviceUuid should never be null
            bleServiceUuid = UUID(0, 0),
            cleanUp = {},
            nativeDevice = nativeDevice,
            coroutineScope = coroutineScope,
            initialState = nativeDevice.state(),
        )
    }
}
