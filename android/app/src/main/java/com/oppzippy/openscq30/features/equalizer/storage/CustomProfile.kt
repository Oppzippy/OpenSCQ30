package com.oppzippy.openscq30.features.equalizer.storage

import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey

@Entity(
    tableName = "equalizer_custom_profile",
    indices = [
        Index(value = ["values"], unique = true),
    ],
)
data class CustomProfile(
    @PrimaryKey val name: String,
    val values: List<Byte>,
)
