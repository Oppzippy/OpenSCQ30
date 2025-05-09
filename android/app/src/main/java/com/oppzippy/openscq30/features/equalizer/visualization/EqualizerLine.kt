package com.oppzippy.openscq30.features.equalizer.visualization

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Paint

class EqualizerLine(private val values: List<Short>, private val minVolume: Short, private val maxVolume: Short) {
    fun drawBitmap(
        bitmap: Bitmap,
        xOffset: Float = 0F,
        yOffset: Float = 0F,
        width: Float = bitmap.width.toFloat(),
        height: Float = bitmap.height.toFloat(),
        padding: Float = 4F,
    ) {
        val canvas = Canvas(bitmap)
        val points = points(width, height, padding)
        val lineCoordinates = points.flatMapIndexed { index, coordinates ->
            val scaledCoordinates = Pair(coordinates.first + xOffset, coordinates.second + yOffset)
            if (index == 0 || index == points.size - 1) {
                scaledCoordinates.toList()
            } else {
                listOf(
                    scaledCoordinates.first,
                    scaledCoordinates.second,
                    scaledCoordinates.first,
                    scaledCoordinates.second,
                )
            }
        }
        canvas.drawLines(
            lineCoordinates.toFloatArray(),
            Paint(Paint.ANTI_ALIAS_FLAG).apply {
                strokeWidth = bitmap.height * 0.05F
                color = 0xFF777777.toInt()
                strokeCap = Paint.Cap.ROUND
                strokeJoin = Paint.Join.ROUND
            },
        )
    }

    private fun points(width: Float, height: Float, padding: Float): List<Pair<Float, Float>> {
        val widthWithoutPadding = width - padding * 2
        val heightWithoutPadding = height - padding * 2
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
