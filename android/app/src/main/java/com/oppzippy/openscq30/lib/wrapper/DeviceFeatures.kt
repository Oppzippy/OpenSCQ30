package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.DeviceFeatures as ProtobufDeviceFeatures
import com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType as ProtobufNoiseCancelingModeType
import com.oppzippy.openscq30.lib.protobuf.SoundModeProfile as ProtobufSoundModeProfile
import com.oppzippy.openscq30.lib.protobuf.TransparencyModeType as ProtobufTransparencyModeType
import com.oppzippy.openscq30.lib.protobuf.deviceFeatures
import com.oppzippy.openscq30.lib.protobuf.dynamicRangeCompressionMinFirmwareVersionOrNull
import com.oppzippy.openscq30.lib.protobuf.soundModeOrNull
import com.oppzippy.openscq30.lib.protobuf.soundModeProfile

data class DeviceFeatures(
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
    fun toProtobuf(): ProtobufDeviceFeatures = deviceFeatures {
        this@DeviceFeatures.soundMode?.let { soundMode = it.toProtobuf() }
        hasHearId = this@DeviceFeatures.hasHearId
        numEqualizerChannels = this@DeviceFeatures.numEqualizerChannels
        numEqualizerBands = this@DeviceFeatures.numEqualizerBands
        hasDynamicRangeCompression = this@DeviceFeatures.hasDynamicRangeCompression
        hasCustomButtonModel = this@DeviceFeatures.hasCustomButtonModel
        hasWearDetection = this@DeviceFeatures.hasWearDetection
        hasTouchTone = this@DeviceFeatures.hasTouchTone
        hasAutoPowerOff = this@DeviceFeatures.hasAutoPowerOff
        this@DeviceFeatures.dynamicRangeCompressionMinFirmwareVersion?.let {
            dynamicRangeCompressionMinFirmwareVersion = it.toProtobuf()
        }
    }
}

fun ProtobufDeviceFeatures.toKotlin(): DeviceFeatures = DeviceFeatures(
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

data class SoundModeProfile(
    val noiseCancelingModeType: NoiseCancelingModeType,
    val transparencyModeType: TransparencyModeType,
) {
    fun toProtobuf(): ProtobufSoundModeProfile = soundModeProfile {
        noiseCancelingModeType = this@SoundModeProfile.noiseCancelingModeType.toProtobuf()
        transparencyModeType = this@SoundModeProfile.transparencyModeType.toProtobuf()
    }
}

fun ProtobufSoundModeProfile.toKotlin(): SoundModeProfile = SoundModeProfile(
    noiseCancelingModeType = noiseCancelingModeType.toKotlin(),
    transparencyModeType = transparencyModeType.toKotlin(),
)

enum class NoiseCancelingModeType {
    None,
    Basic,
    Custom,
    ;

    fun toProtobuf(): ProtobufNoiseCancelingModeType = when (this) {
        None -> ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_NONE
        Basic -> ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_BASIC
        Custom -> ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_CUSTOM
    }
}

fun ProtobufNoiseCancelingModeType.toKotlin(): NoiseCancelingModeType = when (this) {
    ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_NONE -> NoiseCancelingModeType.None
    ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_BASIC -> NoiseCancelingModeType.Basic
    ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_CUSTOM -> NoiseCancelingModeType.Custom
}

enum class TransparencyModeType {
    Basic,
    Custom,
    ;

    fun toProtobuf(): ProtobufTransparencyModeType = when (this) {
        Basic -> ProtobufTransparencyModeType.TRANSPARENCY_MODE_BASIC
        Custom -> ProtobufTransparencyModeType.TRANSPARENCY_MODE_CUSTOM
    }
}

fun ProtobufTransparencyModeType.toKotlin(): TransparencyModeType = when (this) {
    ProtobufTransparencyModeType.TRANSPARENCY_MODE_BASIC -> TransparencyModeType.Basic
    ProtobufTransparencyModeType.TRANSPARENCY_MODE_CUSTOM -> TransparencyModeType.Custom
}
