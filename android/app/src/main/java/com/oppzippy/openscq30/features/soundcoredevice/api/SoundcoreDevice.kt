package com.oppzippy.openscq30.features.soundcoredevice.api

import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import kotlinx.coroutines.flow.Flow

interface SoundcoreDevice {
    val state: SoundcoreDeviceState
    val stateFlow: Flow<SoundcoreDeviceState>
    val isDisconnected: Flow<Boolean>
    val name: String
    val macAddress: String
    fun setSoundModes(newSoundModes: SoundModes)

    fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration)
    fun destroy()
}
