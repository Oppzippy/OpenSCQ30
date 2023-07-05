package com.oppzippy.openscq30.ui.equalizer.models

import com.oppzippy.openscq30.lib.VolumeAdjustments

class EqualizerLine(private val values: List<Byte>) {
    fun draw(width: Float, height: Float, padding: Float): List<Pair<Float, Float>> {
        val widthWithoutPadding = width - padding * 2
        val heightWithoutPadding = height - padding * 2
        val minVolume = VolumeAdjustments.minVolume()
        val maxVolume = VolumeAdjustments.maxVolume()
        val range = maxVolume - minVolume

        val points = values.mapIndexed { index, value ->
            val normalizedX = index.toFloat() / values.size.toFloat()
            val x = normalizedX * widthWithoutPadding + padding
            val normalizedY = 1F - ((value - minVolume) / range.toFloat())
            val y = normalizedY * heightWithoutPadding + padding
            Pair(x, y)
        }
        return points
    }
}
