package com.oppzippy.openscq30.features.soundcoredevice.demo

import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.lib.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

class DemoSoundcoreDevice(
    override val name: String,
    override val macAddress: String,
) : SoundcoreDevice {
    private val _stateFlow = MutableStateFlow(
        SoundcoreDeviceState(
            AmbientSoundMode.Normal,
            NoiseCancelingMode.Indoor,
            EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
        ),
    )
    override val stateFlow: Flow<SoundcoreDeviceState> = _stateFlow.asStateFlow()
    override val isDisconnected = MutableStateFlow(false).asStateFlow()
    override val state: SoundcoreDeviceState
        get() {
            return _stateFlow.value
        }

    override fun setSoundMode(
        newAmbientSoundMode: AmbientSoundMode,
        newNoiseCancelingMode: NoiseCancelingMode,
    ) {
        Log.i(
            "DemoSoundcoreDevice",
            "set ambient sound mode to $newAmbientSoundMode and noise canceling mode to $newNoiseCancelingMode",
        )
        _stateFlow.value = _stateFlow.value.withAmbientSoundMode(newAmbientSoundMode)
            .withNoiseCancelingMode(newNoiseCancelingMode)
    }

    override fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        Log.i(
            "DemoSoundcoreDevice",
            "set equalizer configuration to $equalizerConfiguration",
        )
        _stateFlow.value = _stateFlow.value.withEqualizerConfiguration(equalizerConfiguration)
    }

    override fun destroy() {}
}
