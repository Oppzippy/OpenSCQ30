package com.oppzippy.openscq30.devicesettings

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.R
import kotlin.math.pow
import kotlin.math.roundToInt

@Composable
fun Equalizer() {
    Column {
        for (i in 0..7) {
            EqualizerSlider((100 * 2F.pow(i)).roundToInt(), 0F)
        }
    }
}

@Composable
private fun EqualizerSlider(hz: Int, initialValue: Float) {
    var value by remember { mutableStateOf(initialValue) }
    Row(
        verticalAlignment = Alignment.CenterVertically
    ) {
        Text(stringResource(R.string.hz, hz))
        Slider(value = value, onValueChange = { value = it }, valueRange = -60F..60F)
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        Equalizer()
    }
}
