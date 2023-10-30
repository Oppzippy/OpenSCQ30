package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.bluetooth.BluetoothGatt
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import kotlinx.coroutines.CoroutineScope

interface SoundcoreDeviceFactory {
    fun createSoundcoreDevice(
        gatt: BluetoothGatt,
        callbackHandler: SoundcoreDeviceCallbackHandler,
        scope: CoroutineScope,
        deviceState: SoundcoreDeviceState,
    ): SoundcoreDevice
}
