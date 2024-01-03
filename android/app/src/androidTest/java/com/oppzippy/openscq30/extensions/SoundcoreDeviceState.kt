package com.oppzippy.openscq30.extensions

import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.extensions.resources.toEqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.Battery
import com.oppzippy.openscq30.lib.wrapper.DeviceProfile
import com.oppzippy.openscq30.lib.wrapper.DeviceState
import com.oppzippy.openscq30.lib.wrapper.SingleBattery

fun DeviceState.Companion.empty(): DeviceState {
    return DeviceState(
        deviceProfile = DeviceProfile(
            soundMode = null,
            hasHearId = false,
            numEqualizerChannels = 0,
            numEqualizerBands = 0,
            hasDynamicRangeCompression = false,
            hasCustomButtonModel = false,
            hasWearDetection = false,
            hasTouchTone = false,
            hasAutoPowerOff = false,
            dynamicRangeCompressionMinFirmwareVersion = null,
        ),
        serialNumber = null,
        firmwareVersion = null,
        equalizerConfiguration = PresetEqualizerProfile.SoundcoreSignature.toEqualizerConfiguration(),
        soundModes = null,
        battery = Battery.Single(SingleBattery(false, 0u)),
        ageRange = null,
        gender = null,
        customButtonModel = null,
        hearId = null,
        ambientSoundModeCycle = null,
    )
}
