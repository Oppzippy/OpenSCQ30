package com.oppzippy.openscq30.features.soundcoredevice.demo

import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.lib.bindings.AgeRange
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.FirmwareVersion
import com.oppzippy.openscq30.lib.bindings.Gender
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

class DemoSoundcoreDevice(
    override val name: String,
    override val macAddress: String,
) : SoundcoreDevice {
    private val _stateFlow = MutableStateFlow(
        SoundcoreDeviceState(
            featureFlags = -1, // all features on
            soundModes = SoundModes(
                AmbientSoundMode.Normal,
                NoiseCancelingMode.Indoor,
                TransparencyMode.VocalMode,
                CustomNoiseCanceling(0),
            ),
            equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
            leftFirmwareVersion = FirmwareVersion(1, 0),
            rightFirmwareVersion = FirmwareVersion(1, 1),
            serialNumber = "0000000000000000",
            leftBatteryLevel = 4,
            rightBatteryLevel = 3,
            isLeftBatteryCharging = false,
            isRightBatteryCharging = true,
            ageRange = AgeRange(1),
            gender = Gender(2),
            customHearId = null,
            dynamicRangeCompressionMinFirmwareVersion = null,
        ),
    )
    override val stateFlow: Flow<SoundcoreDeviceState> = _stateFlow.asStateFlow()
    override val isDisconnected = MutableStateFlow(false).asStateFlow()
    override val state: SoundcoreDeviceState
        get() {
            return _stateFlow.value
        }

    override fun setSoundModes(newSoundModes: SoundModes) {
        Log.i(
            "DemoSoundcoreDevice",
            "set ambient sound mode to ${newSoundModes.ambientSoundMode()} and noise canceling mode to ${newSoundModes.noiseCancelingMode()}",
        )
        _stateFlow.value = _stateFlow.value.copy(soundModes = newSoundModes)
    }

    override fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        Log.i(
            "DemoSoundcoreDevice",
            "set equalizer configuration to $equalizerConfiguration",
        )
        _stateFlow.value = _stateFlow.value.copy(equalizerConfiguration = equalizerConfiguration)
    }

    override fun destroy() {}
}
