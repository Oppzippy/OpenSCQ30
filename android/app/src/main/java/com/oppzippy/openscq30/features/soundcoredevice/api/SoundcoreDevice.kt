package com.oppzippy.openscq30.features.soundcoredevice.api

import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import kotlinx.coroutines.flow.Flow

interface SoundcoreDevice {
    val state: SoundcoreDeviceState
    val stateFlow: Flow<SoundcoreDeviceState>
    val isDisconnected: Flow<Boolean>
    val name: String
    val macAddress: String
    fun setSoundMode(
        newAmbientSoundMode: AmbientSoundMode,
        newNoiseCancelingMode: NoiseCancelingMode,
    )
    fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration)
    fun destroy()
}
