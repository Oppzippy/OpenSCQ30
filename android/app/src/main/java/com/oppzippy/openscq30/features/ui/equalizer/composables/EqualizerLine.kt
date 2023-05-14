package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.width
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.StrokeJoin
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.graphics.vector.PathNode
import androidx.compose.ui.graphics.vector.rememberVectorPainter
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun EqualizerLine(values: List<Byte>, width: Dp, height: Dp) {
    val padding = 4
    val widthWithoutPadding = width.value - padding * 2
    val heightWithoutPadding = height.value - padding * 2
    val minValue = -120
    val range = 240

    val points = values.mapIndexed { index, value ->
        val normalizedX = index.toFloat() / values.size.toFloat()
        val x = normalizedX * widthWithoutPadding + padding
        val normalizedY = 1F - ((value - minValue) / range.toFloat())
        val y = normalizedY * heightWithoutPadding + padding
        if (index == 0) {
            PathNode.MoveTo(x, y)
        } else {
            PathNode.LineTo(x, y)
        }
    }
    val vector = ImageVector.Builder(
        defaultWidth = width,
        defaultHeight = height,
        viewportWidth = width.value,
        viewportHeight = height.value,
    ).addPath(
        points,
        stroke = SolidColor(MaterialTheme.colorScheme.primary),
        strokeLineWidth = 2F,
        strokeAlpha = 0.4F,
        strokeLineCap = StrokeCap.Round,
        strokeLineJoin = StrokeJoin.Round,
    ).build()

    val painter = rememberVectorPainter(image = vector)

    Canvas(modifier = Modifier
        .width(width)
        .height(height)) {
        with(painter) {
            draw(painter.intrinsicSize)
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    val values by remember { mutableStateOf(listOf<Byte>(0, 10, 120, 0, -10, -120, 0, 0)) }
    OpenSCQ30Theme {
        EqualizerLine(values = values, width = 80.dp, height = 20.dp)
    }
}
