package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.bindings.AgeRange
import com.oppzippy.openscq30.lib.bindings.CustomHearId
import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.FirmwareVersion
import com.oppzippy.openscq30.lib.bindings.Gender
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.StateUpdatePacket
import kotlin.jvm.optionals.getOrNull

data class SoundcoreDeviceState(
    val featureFlags: DeviceFeatureFlags,
    val leftBatteryLevel: Short,
    val rightBatteryLevel: Short,
    val isLeftBatteryCharging: Boolean,
    val isRightBatteryCharging: Boolean,
    val equalizerConfiguration: EqualizerConfiguration,
    val soundModes: SoundModes?,
    val ageRange: AgeRange?,
    val gender: Gender?,
    val customHearId: CustomHearId?,
//    val customButtonModel: CustomButtonModel?,
    val leftFirmwareVersion: FirmwareVersion?,
    val rightFirmwareVersion: FirmwareVersion?,
    val serialNumber: String?,
    val dynamicRangeCompressionMinFirmwareVersion: FirmwareVersion?,
) {
    companion object // used for static extension methods in tests

    fun supportsDynamicRangeCompression(): Boolean {
        if (featureFlags.contains(DeviceFeatureFlags.dynamicRangeCompression())) {
            if (leftFirmwareVersion == null || dynamicRangeCompressionMinFirmwareVersion == null) {
                return false
            }
            return if (rightFirmwareVersion == null) {
                leftFirmwareVersion.compare(dynamicRangeCompressionMinFirmwareVersion) >= 0
            } else {
                leftFirmwareVersion.compare(dynamicRangeCompressionMinFirmwareVersion) >= 0 &&
                    rightFirmwareVersion.compare(dynamicRangeCompressionMinFirmwareVersion) >= 0
            }
        }
        return false
    }
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
        ageRange = ageRange().getOrNull(),
        gender = gender().getOrNull(),
        dynamicRangeCompressionMinFirmwareVersion = dynamicRangeCompressionMinFirmwareVersion().getOrNull(),
        customHearId = customHearId().getOrNull(),
    )
}
