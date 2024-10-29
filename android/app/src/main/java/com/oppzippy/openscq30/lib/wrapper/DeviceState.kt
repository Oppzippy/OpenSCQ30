package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.ambientSoundModeCycleOrNull
import com.oppzippy.openscq30.lib.protobuf.customButtonModelOrNull
import com.oppzippy.openscq30.lib.protobuf.deviceState
import com.oppzippy.openscq30.lib.protobuf.firmwareVersionOrNull
import com.oppzippy.openscq30.lib.protobuf.hearIdOrNull
import com.oppzippy.openscq30.lib.protobuf.soundModesOrNull
import com.oppzippy.openscq30.lib.protobuf.soundModesTypeTwoOrNull

data class DeviceState(
    val deviceFeatures: DeviceFeatures,
    val battery: Battery,
    val equalizerConfiguration: EqualizerConfiguration,
    val soundModes: SoundModes?,
    val soundModesTypeTwo: SoundModesTypeTwo?,
    val ageRange: UByte?,
    val gender: UByte?,
    val hearId: HearId?,
    val customButtonModel: CustomButtonModel?,
    val firmwareVersion: FirmwareVersion?,
    val serialNumber: String?,
    val ambientSoundModeCycle: AmbientSoundModeCycle?,
) {
    companion object // used for static extension methods in tests

    fun supportsDynamicRangeCompression(): Boolean {
        if (deviceFeatures.hasDynamicRangeCompression) {
            if (firmwareVersion == null) {
                return false
            }
            val minAllowedFirmwareVersion =
                deviceFeatures.dynamicRangeCompressionMinFirmwareVersion ?: return true
            return firmwareVersion >= minAllowedFirmwareVersion
        }
        return false
    }

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.DeviceState = deviceState {
        deviceFeatures = this@DeviceState.deviceFeatures.toProtobuf()
        battery = this@DeviceState.battery.toProtobuf()
        equalizerConfiguration = this@DeviceState.equalizerConfiguration.toProtobuf()
        this@DeviceState.soundModes?.let { soundModes = it.toProtobuf() }
        this@DeviceState.soundModesTypeTwo?.let { soundModesTypeTwo = it.toProtobuf() }
        this@DeviceState.ageRange?.let { ageRange = it.toInt() }
        this@DeviceState.gender?.let { gender = it.toInt() }
        this@DeviceState.hearId?.let { hearId = it.toProtobuf() }
        this@DeviceState.customButtonModel?.let { customButtonModel = it.toProtobuf() }
        this@DeviceState.firmwareVersion?.let { firmwareVersion = it.toProtobuf() }
        this@DeviceState.serialNumber?.let { serialNumber = it }
        this@DeviceState.ambientSoundModeCycle?.let { ambientSoundModeCycle = it.toProtobuf() }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.DeviceState.toKotlin(): DeviceState = DeviceState(
    deviceFeatures = deviceFeatures.toKotlin(),
    battery = battery.toKotlin(),
    equalizerConfiguration = equalizerConfiguration.toKotlin(),
    soundModes = soundModesOrNull?.toKotlin(),
    soundModesTypeTwo = soundModesTypeTwoOrNull?.toKotlin(),
    ageRange = if (hasAgeRange()) ageRange.toUByte() else null,
    gender = if (hasGender()) gender.toUByte() else null,
    hearId = hearIdOrNull?.toKotlin(),
    customButtonModel = customButtonModelOrNull?.toKotlin(),
    firmwareVersion = firmwareVersionOrNull?.toKotlin(),
    serialNumber = if (hasSerialNumber()) serialNumber else null,
    ambientSoundModeCycle = ambientSoundModeCycleOrNull?.toKotlin(),
)
