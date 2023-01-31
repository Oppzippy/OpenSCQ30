package com.oppzippy.openscq30.features.soundcoredevice

import dagger.hilt.android.scopes.ActivityRetainedScoped
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import java.io.Closeable
import javax.inject.Inject

@ActivityRetainedScoped
class SoundcoreDeviceBox @Inject constructor(private val factory: SoundcoreDeviceFactory): Closeable {
    private val _device: MutableStateFlow<SoundcoreDevice?> = MutableStateFlow(null)
    val device = _device.asStateFlow()

    suspend fun setDevice(macAddress: String, scope: CoroutineScope) {
        val newDevice = factory.createSoundcoreDevice(macAddress, scope)
        swapDevice(newDevice)
    }

    @Synchronized
    private fun swapDevice(newDevice: SoundcoreDevice?) {
        val oldDevice = _device.value
        _device.value = newDevice
        oldDevice?.destroy()
    }

    override fun close() {
        _device.value?.destroy()
    }
}