package com.oppzippy.openscq30.room

import androidx.room.TypeConverter
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling

class Converters {
    @TypeConverter
    fun fromByteList(bytes: List<Byte>): ByteArray {
        return bytes.toByteArray()
    }

    @TypeConverter
    fun toByteList(bytes: ByteArray): List<Byte> {
        return bytes.asList()
    }

    @TypeConverter
    fun fromCustomNoiseCanceling(customNoiseCanceling: CustomNoiseCanceling): Short {
        return customNoiseCanceling.value()
    }

    @TypeConverter
    fun toCustomNoiseCanceling(value: Short): CustomNoiseCanceling {
        return CustomNoiseCanceling(value)
    }
}
