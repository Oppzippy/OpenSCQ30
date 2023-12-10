package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.HearId.HearIdCase
import com.oppzippy.openscq30.lib.protobuf.basicHearId
import com.oppzippy.openscq30.lib.protobuf.customHearId
import com.oppzippy.openscq30.lib.protobuf.hearId

sealed class HearId {
    class Basic(val basicHearId: BasicHearId) : HearId()
    class Custom(val customHearId: CustomHearId) : HearId()

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.HearId {
        return when (this) {
            is Basic -> hearId { basic = basicHearId.toProtobuf() }
            is Custom -> hearId { custom = customHearId.toProtobuf() }
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.HearId.toKotlin(): HearId {
    return when (hearIdCase) {
        HearIdCase.BASIC -> HearId.Basic(basic.toKotlin())
        HearIdCase.CUSTOM -> HearId.Custom(custom.toKotlin())
        HearIdCase.HEARID_NOT_SET -> TODO()
        null -> TODO()
    }
}

data class BasicHearId(
    val isEnabled: Boolean,
    val volumeAdjustments: StereoVolumeAdjustments,
    val time: Int,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.BasicHearId {
        return basicHearId {
            isEnabled = this@BasicHearId.isEnabled
            volumeAdjustments = this@BasicHearId.volumeAdjustments.toProtobuf()
            time = this@BasicHearId.time
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.BasicHearId.toKotlin(): BasicHearId {
    return BasicHearId(
        isEnabled = isEnabled,
        volumeAdjustments = volumeAdjustments.toKotlin(),
        time = time,
    )
}

data class CustomHearId(
    val isEnabled: Boolean,
    val volumeAdjustments: StereoVolumeAdjustments,
    val time: Int,
    val hearIdType: UByte,
    val hearIdMusicType: UByte,
    val customVolumeAdjustments: StereoVolumeAdjustments?,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.CustomHearId {
        return customHearId {
            isEnabled = this@CustomHearId.isEnabled
            volumeAdjustments = this@CustomHearId.volumeAdjustments.toProtobuf()
            time = this@CustomHearId.time
            hearIdType = this@CustomHearId.hearIdType.toInt()
            hearIdMusicType = this@CustomHearId.hearIdMusicType.toInt()
            this@CustomHearId.customVolumeAdjustments?.let {
                customVolumeAdjustments = it.toProtobuf()
            }
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.CustomHearId.toKotlin(): CustomHearId {
    return CustomHearId(
        isEnabled = isEnabled,
        volumeAdjustments = volumeAdjustments.toKotlin(),
        time = time,
        hearIdType = hearIdType.toUByte(),
        hearIdMusicType = hearIdMusicType.toUByte(),
        customVolumeAdjustments = if (hasCustomVolumeAdjustments()) customVolumeAdjustments.toKotlin() else null,
    )
}
