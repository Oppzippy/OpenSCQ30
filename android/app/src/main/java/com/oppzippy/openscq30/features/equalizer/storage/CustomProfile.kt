package com.oppzippy.openscq30.features.equalizer.storage

import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey
import com.oppzippy.openscq30.lib.bindings.VolumeAdjustments

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
    fun getVolumeAdjustments(): VolumeAdjustments {
        return VolumeAdjustments(
            doubleArrayOf(
                band100,
                band200,
                band400,
                band800,
                band1600,
                band3200,
                band6400,
                band12800,
            ),
        )
    }
}

fun VolumeAdjustments.toCustomProfile(name: String): CustomProfile {
    val adjustments = adjustments()
    return CustomProfile(
        name,
        adjustments[0],
        adjustments[1],
        adjustments[2],
        adjustments[3],
        adjustments[4],
        adjustments[5],
        adjustments[6],
        adjustments[7],
    )
}
