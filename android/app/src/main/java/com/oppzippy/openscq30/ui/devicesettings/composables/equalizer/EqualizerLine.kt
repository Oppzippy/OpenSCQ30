package com.oppzippy.openscq30.ui.devicesettings.composables.equalizer

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
import com.oppzippy.openscq30.features.equalizer.visualization.EqualizerLine
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun EqualizerLine(values: List<Double>, width: Dp, height: Dp) {
    val line = EqualizerLine(values)
    val points = line.points(width.value, height.value, 4F)

    val pathNodes = points.mapIndexed { index, pair ->
        if (index == 0) {
            PathNode.MoveTo(pair.first, pair.second)
        } else {
            PathNode.LineTo(pair.first, pair.second)
        }
    }

    val vector = ImageVector.Builder(
        defaultWidth = width,
        defaultHeight = height,
        viewportWidth = width.value,
        viewportHeight = height.value,
    ).addPath(
        pathNodes,
        stroke = SolidColor(MaterialTheme.colorScheme.primary),
        strokeLineWidth = 2F,
        strokeAlpha = 0.4F,
        strokeLineCap = StrokeCap.Round,
        strokeLineJoin = StrokeJoin.Round,
    ).build()

    val painter = rememberVectorPainter(image = vector)

    Canvas(
        modifier = Modifier
            .width(width)
            .height(height),
    ) {
        with(painter) {
            draw(painter.intrinsicSize)
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewEqualizerLine() {
    val values by remember {
        mutableStateOf(
            listOf(
                0.0,
                1.0,
                12.0,
                0.0,
                -1.0,
                -12.0,
                0.0,
                0.0,
            ),
        )
    }
    OpenSCQ30Theme {
        EqualizerLine(values = values, width = 80.dp, height = 20.dp)
    }
}
