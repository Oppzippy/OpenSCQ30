package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.deviceProfile
import com.oppzippy.openscq30.lib.protobuf.dynamicRangeCompressionMinFirmwareVersionOrNull
import com.oppzippy.openscq30.lib.protobuf.soundModeOrNull
import com.oppzippy.openscq30.lib.protobuf.soundModeProfile

data class DeviceProfile(
    val soundMode: SoundModeProfile?,
    val hasHearId: Boolean,
    val numEqualizerChannels: Int,
    val numEqualizerBands: Int,
    val hasDynamicRangeCompression: Boolean,
    val hasCustomButtonModel: Boolean,
    val hasWearDetection: Boolean,
    val hasTouchTone: Boolean,
    val hasAutoPowerOff: Boolean,
    val dynamicRangeCompressionMinFirmwareVersion: FirmwareVersion?,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.DeviceProfile {
        return deviceProfile {
            this@DeviceProfile.soundMode?.let { soundMode = it.toProtobuf() }
            hasHearId = this@DeviceProfile.hasHearId
            numEqualizerChannels = this@DeviceProfile.numEqualizerChannels
            numEqualizerBands = this@DeviceProfile.numEqualizerBands
            hasDynamicRangeCompression = this@DeviceProfile.hasDynamicRangeCompression
            hasCustomButtonModel = this@DeviceProfile.hasCustomButtonModel
            hasWearDetection = this@DeviceProfile.hasWearDetection
            hasTouchTone = this@DeviceProfile.hasTouchTone
            hasAutoPowerOff = this@DeviceProfile.hasAutoPowerOff
            this@DeviceProfile.dynamicRangeCompressionMinFirmwareVersion?.let {
                dynamicRangeCompressionMinFirmwareVersion = it.toProtobuf()
            }
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.DeviceProfile.toKotlin(): DeviceProfile {
    return DeviceProfile(
        soundMode = soundModeOrNull?.toKotlin(),
        hasHearId = hasHearId,
        numEqualizerChannels = numEqualizerChannels,
        numEqualizerBands = numEqualizerBands,
        hasDynamicRangeCompression = hasDynamicRangeCompression,
        hasCustomButtonModel = hasCustomButtonModel,
        hasWearDetection = hasWearDetection,
        hasTouchTone = hasTouchTone,
        hasAutoPowerOff = hasAutoPowerOff,
        dynamicRangeCompressionMinFirmwareVersion = dynamicRangeCompressionMinFirmwareVersionOrNull?.toKotlin(),
    )
}

data class SoundModeProfile(
    val noiseCancelingModeType: NoiseCancelingModeType,
    val transparencyModeType: TransparencyModeType,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.SoundModeProfile {
        return soundModeProfile {
            noiseCancelingModeType = this@SoundModeProfile.noiseCancelingModeType.toProtobuf()
            transparencyModeType = this@SoundModeProfile.transparencyModeType.toProtobuf()
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.SoundModeProfile.toKotlin(): SoundModeProfile {
    return SoundModeProfile(
        noiseCancelingModeType = noiseCancelingModeType.toKotlin(),
        transparencyModeType = transparencyModeType.toKotlin(),
    )
}

enum class NoiseCancelingModeType {
    None,
    Basic,
    Custom,
    ;

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType {
        return when (this) {
            None -> com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType.NOISE_CANCELING_MODE_NONE
            Basic -> com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType.NOISE_CANCELING_MODE_BASIC
            Custom -> com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType.NOISE_CANCELING_MODE_CUSTOM
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType.toKotlin(): NoiseCancelingModeType {
    return when (this) {
        com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType.NOISE_CANCELING_MODE_NONE -> NoiseCancelingModeType.None
        com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType.NOISE_CANCELING_MODE_BASIC -> NoiseCancelingModeType.Basic
        com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType.NOISE_CANCELING_MODE_CUSTOM -> NoiseCancelingModeType.Custom
    }
}

enum class TransparencyModeType {
    Basic,
    Custom,
    ;

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.TransparencyModeType {
        return when (this) {
            Basic -> com.oppzippy.openscq30.lib.protobuf.TransparencyModeType.TRANSPARENCY_MODE_BASIC
            Custom -> com.oppzippy.openscq30.lib.protobuf.TransparencyModeType.TRANSPARENCY_MODE_CUSTOM
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.TransparencyModeType.toKotlin(): TransparencyModeType {
    return when (this) {
        com.oppzippy.openscq30.lib.protobuf.TransparencyModeType.TRANSPARENCY_MODE_BASIC -> TransparencyModeType.Basic
        com.oppzippy.openscq30.lib.protobuf.TransparencyModeType.TRANSPARENCY_MODE_CUSTOM -> TransparencyModeType.Custom
    }
}
