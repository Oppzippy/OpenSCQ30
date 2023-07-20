package com.oppzippy.openscq30.features.soundcoredevice.api

import com.oppzippy.openscq30.libbindings.AmbientSoundMode
import com.oppzippy.openscq30.libbindings.EqualizerConfiguration
import com.oppzippy.openscq30.libbindings.NoiseCancelingMode
import com.oppzippy.openscq30.libbindings.SoundcoreDeviceState
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
