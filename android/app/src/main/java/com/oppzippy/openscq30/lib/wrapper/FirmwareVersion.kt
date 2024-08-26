package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.firmwareVersion

data class FirmwareVersion(val major: UByte, val minor: UByte) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.FirmwareVersion = firmwareVersion {
        major = this@FirmwareVersion.major.toInt()
        minor = this@FirmwareVersion.minor.toInt()
    }

    operator fun compareTo(other: FirmwareVersion): Int {
        val majorComparison = major.compareTo(other.major)
        return if (majorComparison != 0) {
            majorComparison
        } else {
            minor.compareTo(other.minor)
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.FirmwareVersion.toKotlin(): FirmwareVersion = FirmwareVersion(
    major = major.toUByte(),
    minor = minor.toUByte(),
)
