package com.oppzippy.openscq30.features.equalizer.storage

import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey
import kotlin.math.roundToInt

private const val SCALE_FACTOR = 10.0

private fun scaleUpToInt(value: Double): Int = (value * SCALE_FACTOR).roundToInt()

private fun scaleDownToDouble(value: Int): Double = value.toDouble() / SCALE_FACTOR

@Entity(
    tableName = "custom_equalizer_profile",
    indices = [
        Index(
            name = "index_custom_equalizer_profile_bands",
            value = ["band100", "band200", "band400", "band800", "band1600", "band3200", "band6400", "band12800"],
            unique = true,
        ),
    ],
)
data class CustomProfile(
    @PrimaryKey val name: String,
    val band100: Int,
    val band200: Int,
    val band400: Int,
    val band800: Int,
    val band1600: Int,
    val band3200: Int,
    val band6400: Int,
    val band12800: Int,
) {
    @Deprecated(
        "Int constructor should be used instead. If you want doubles, make your own class rather than reusing this.",
    )
    constructor(
        name: String,
        band100: Double,
        band200: Double,
        band400: Double,
        band800: Double,
        band1600: Double,
        band3200: Double,
        band6400: Double,
        band12800: Double,
    ) : this(
        name,
        scaleUpToInt(band100),
        scaleUpToInt(band200),
        scaleUpToInt(band400),
        scaleUpToInt(band800),
        scaleUpToInt(band1600),
        scaleUpToInt(band3200),
        scaleUpToInt(band6400),
        scaleUpToInt(band12800),
    )

    fun getVolumeAdjustments(): List<Double> = listOf(
        scaleDownToDouble(band100),
        scaleDownToDouble(band200),
        scaleDownToDouble(band400),
        scaleDownToDouble(band800),
        scaleDownToDouble(band1600),
        scaleDownToDouble(band3200),
        scaleDownToDouble(band6400),
        scaleDownToDouble(band12800),
    )
}

fun List<Double>.toCustomProfile(name: String): CustomProfile = CustomProfile(
    name,
    scaleUpToInt(this[0]),
    scaleUpToInt(this[1]),
    scaleUpToInt(this[2]),
    scaleUpToInt(this[3]),
    scaleUpToInt(this[4]),
    scaleUpToInt(this[5]),
    scaleUpToInt(this[6]),
    scaleUpToInt(this[7]),
)
