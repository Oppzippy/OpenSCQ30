package com.oppzippy.openscq30.features.soundcoredevice.demo

import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.lib.*
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow

class DemoSoundcoreDevice(
    override val name: String,
    override val macAddress: String,
) : SoundcoreDevice {
    private val _stateFlow = MutableStateFlow(
        SoundcoreDeviceState(
            AmbientSoundMode.Normal,
            NoiseCancelingMode.Indoor,
            EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature)
        )
    )
    override val stateFlow: Flow<SoundcoreDeviceState> = _stateFlow
    override val state: SoundcoreDeviceState
        get() {
            return _stateFlow.value
        }

    override fun setSoundMode(
        newAmbientSoundMode: AmbientSoundMode, newNoiseCancelingMode: NoiseCancelingMode
    ) {
        _stateFlow.value = _stateFlow.value.withAmbientSoundMode(newAmbientSoundMode)
            .withNoiseCancelingMode(newNoiseCancelingMode)
    }

    override fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        _stateFlow.value = _stateFlow.value.withEqualizerConfiguration(equalizerConfiguration)
    }

    override fun destroy() {
    }
}