package com.oppzippy.openscq30.features.equalizer.storage

import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey

@Entity(
    tableName = "custom_equalizer_profile",
    indices = [
        Index(
            value = ["band100", "band200", "band400", "band800", "band1600", "band3200", "band6400", "band12800"],
            unique = true,
        ),
    ],
)
data class CustomProfile(
    @PrimaryKey val name: String,
    val band100: Double,
    val band200: Double,
    val band400: Double,
    val band800: Double,
    val band1600: Double,
    val band3200: Double,
    val band6400: Double,
    val band12800: Double,
) {
    fun getVolumeAdjustments(): List<Double> {
        return listOf(
            band100,
            band200,
            band400,
            band800,
            band1600,
            band3200,
            band6400,
            band12800,
        )
    }
}

fun List<Double>.toCustomProfile(name: String): CustomProfile {
    return CustomProfile(
        name,
        this[0],
        this[1],
        this[2],
        this[3],
        this[4],
        this[5],
        this[6],
        this[7],
    )
}
