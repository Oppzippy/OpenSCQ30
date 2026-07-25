package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.PointMode
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import java.math.BigDecimal
import java.math.MathContext
import kotlin.math.absoluteValue

@Composable
fun ReadOnlyEqualizer(
    modifier: Modifier,
    color: Color,
    bands: List<UShort>,
    values: List<Short>,
    minValue: Short,
    maxValue: Short,
    fractionDigits: Short,
    drawHorizontalGuide: Boolean = true,
    strokeWidthDp: Dp = 2.dp,
) {
    val contentDescription = bands.take(values.size).map { bandHz ->
        if (bandHz < 1000.toUShort()) {
            stringResource(R.string.hz, bandHz.toInt())
        } else {
            stringResource(
                R.string.khz,
                BigDecimal(bandHz.toInt()).scaleByPowerOfTen(-3).round(MathContext(1)).toPlainString(),
            )
        }
    }.withIndex().joinToString(separator = ", ") { (index, bandHz) ->
        "$bandHz: ${BigDecimal(values[index].toInt()).scaleByPowerOfTen(-fractionDigits)}"
    }

    Canvas(modifier, contentDescription) {
        val strokeWidth = strokeWidthDp.toPx()
        val padding = Offset(strokeWidth, strokeWidth)
        val sizeWithoutPadding = Offset(
            x = size.width - padding.x * 2f,
            y = size.height - padding.y * 2f,
        )

        val range = (maxValue - minValue).toFloat()
        val zeroValueY = size.height - size.height * (minValue.toFloat().absoluteValue / range)

        if (drawHorizontalGuide) {
            listOf(strokeWidth, zeroValueY, size.height - strokeWidth).forEach { y ->
                drawLine(
                    color = color.copy(alpha = 0.3f),
                    start = Offset(0f, y),
                    end = Offset(size.width, y),
                    strokeWidth = strokeWidth,
                )
            }
        }

        val unscaledPoints = equalizerLinePoints(
            minValue = minValue,
            maxValue = maxValue,
            values = values,
        )
        val canvasSpacePoints = unscaledPoints.map { it * sizeWithoutPadding + padding }
        // line
        drawPoints(
            canvasSpacePoints.zipWithNext().flatMap { listOf(it.first, it.second) },
            pointMode = PointMode.Lines,
            color = color,
            cap = StrokeCap.Round,
            strokeWidth = strokeWidth,
        )
        // shading below the line
        drawPath(
            Path().apply {
                val leftEdgeX = canvasSpacePoints.first().x - strokeWidth
                val rightEdgeX = canvasSpacePoints.last().x + strokeWidth

                // top line
                moveTo(leftEdgeX, canvasSpacePoints.first().y)
                canvasSpacePoints.forEach { lineTo(it.x, it.y) }
                lineTo(rightEdgeX, canvasSpacePoints.last().y)

                // bottom line
                lineTo(rightEdgeX, size.height)
                lineTo(leftEdgeX, size.height)
                close()
            },
            color = color,
            alpha = 0.3f,
        )
    }
}

private fun equalizerLinePoints(minValue: Short, maxValue: Short, values: List<Short>): List<Offset> {
    val range = (maxValue - minValue).toFloat()
    return values.mapIndexed { index, value ->
        val x = index.toFloat() / (values.size - 1).toFloat()
        val y = (value - minValue).toFloat() / range
        Offset(x, 1 - y)
    }
}

private operator fun Offset.times(other: Offset): Offset = Offset(x * other.x, y * other.y)

@Preview(showBackground = true)
@Composable
private fun PreviewEqualizer() {
    val bands = listOf<UShort>(100u, 200u, 400u, 800u, 1600u, 3200u, 6400u, 12000u)
    OpenSCQ30Theme {
        Surface {
            ReadOnlyEqualizer(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(80.dp),
                color = MaterialTheme.colorScheme.onSurface,
                bands = bands,
                values = listOf(60, -60, 30, -30, 120, -120, 0, 0),
                minValue = -120,
                maxValue = 134,
                fractionDigits = 1,
            )
        }
    }
}
