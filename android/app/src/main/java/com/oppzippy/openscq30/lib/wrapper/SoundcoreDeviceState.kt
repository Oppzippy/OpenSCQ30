package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.bindings.AgeRange
import com.oppzippy.openscq30.lib.bindings.CustomHearId
import com.oppzippy.openscq30.lib.bindings.DeviceProfile
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.FirmwareVersion
import com.oppzippy.openscq30.lib.bindings.Gender
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.StateUpdatePacket
import kotlin.jvm.optionals.getOrNull

data class SoundcoreDeviceState(
    val deviceProfile: DeviceProfile,
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
    val firmwareVersion: FirmwareVersion?,
    val serialNumber: String?,
) {
    companion object // used for static extension methods in tests

    fun supportsDynamicRangeCompression(): Boolean {
        if (deviceProfile.hasDynamicRangeCompression()) {
            val minFirmwareVersion =
                deviceProfile.dynamicRangeCompressionMinFirmwareVersion().getOrNull() ?: return true
            if (firmwareVersion == null) {
                return false
            }
            return firmwareVersion.compare(minFirmwareVersion) >= 0
        }
        return false
    }
}

fun StateUpdatePacket.toSoundcoreDeviceState(): SoundcoreDeviceState {
    return SoundcoreDeviceState(
        deviceProfile = deviceProfile(),
        equalizerConfiguration = equalizerConfiguration(),
        soundModes = soundModes().getOrNull(),
        firmwareVersion = firmwareVersion().getOrNull(),
        serialNumber = serialNumber().getOrNull(),
        isLeftBatteryCharging = isLeftBatteryCharging,
        isRightBatteryCharging = isRightBatteryCharging,
        leftBatteryLevel = leftBatteryLevel(),
        rightBatteryLevel = rightBatteryLevel(),
        ageRange = ageRange().getOrNull(),
        gender = gender().getOrNull(),
        customHearId = customHearId().getOrNull(),
    )
}
