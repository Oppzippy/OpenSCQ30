package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.FirmwareVersion
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.StateUpdatePacket
import kotlin.jvm.optionals.getOrNull

data class SoundcoreDeviceState(
    val featureFlags: Int,
    val leftBatteryLevel: Short,
    val rightBatteryLevel: Short,
    val isLeftBatteryCharging: Boolean,
    val isRightBatteryCharging: Boolean,
    val equalizerConfiguration: EqualizerConfiguration,
    val soundModes: SoundModes?,
//    val ageRange: AgeRange?,
//    val customHearId: HearId?,
//    val customButtonModel: CustomButtonModel?,
    val leftFirmwareVersion: FirmwareVersion?,
    val rightFirmwareVersion: FirmwareVersion?,
    val serialNumber: String?,
) {
    companion object // used for static extension methods in tests
}

fun StateUpdatePacket.toSoundcoreDeviceState(): SoundcoreDeviceState {
    return SoundcoreDeviceState(
        featureFlags = featureFlags(),
        equalizerConfiguration = equalizerConfiguration(),
        soundModes = soundModes().getOrNull(),
        leftFirmwareVersion = firmwareVersion().getOrNull(),
        rightFirmwareVersion = null,
        serialNumber = serialNumber().getOrNull(),
        isLeftBatteryCharging = false, // TODO
        isRightBatteryCharging = false, // TODO
        leftBatteryLevel = 0, // TODO
        rightBatteryLevel = 0, // TODO
    )
}
