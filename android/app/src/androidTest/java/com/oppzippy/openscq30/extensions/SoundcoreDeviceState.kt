package com.oppzippy.openscq30.extensions

import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.extensions.resources.toEqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.Battery
import com.oppzippy.openscq30.lib.wrapper.DeviceFeatures
import com.oppzippy.openscq30.lib.wrapper.DeviceState
import com.oppzippy.openscq30.lib.wrapper.SingleBattery

fun DeviceState.Companion.empty(): DeviceState = DeviceState(
    deviceFeatures = DeviceFeatures(
        availableSoundModes = null,
        hasHearId = false,
        numEqualizerChannels = 0,
        numEqualizerBands = 0,
        hasDynamicRangeCompression = false,
        hasButtonConfiguration = false,
        hasWearDetection = false,
        hasTouchTone = false,
        hasAutoPowerOff = false,
        dynamicRangeCompressionMinFirmwareVersion = null,
    ),
    serialNumber = null,
    firmwareVersion = null,
    equalizerConfiguration = PresetEqualizerProfile.SoundcoreSignature.toEqualizerConfiguration(),
    soundModes = null,
    soundModesTypeTwo = null,
    battery = Battery.Single(SingleBattery(false, 0u)),
    ageRange = null,
    gender = null,
    buttonConfiguration = null,
    hearId = null,
    ambientSoundModeCycle = null,
)
