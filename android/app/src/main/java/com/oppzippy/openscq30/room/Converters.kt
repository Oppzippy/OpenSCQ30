package com.oppzippy.openscq30.room

import androidx.room.TypeConverter

class Converters {
    @TypeConverter
    fun fromByteList(bytes: List<Byte>): ByteArray {
        return bytes.toByteArray()
    }

    @TypeConverter
    fun toByteList(bytes: ByteArray): List<Byte> {
        return bytes.asList()
    }
}