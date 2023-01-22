package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Divider
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.R
import java.text.NumberFormat
import kotlin.math.pow
import kotlin.math.roundToInt

@Composable
fun Equalizer(values: List<Byte>, onValueChange: (index: Int, value: Byte) -> Unit) {
    if (values.size != 8) {
        throw IllegalArgumentException("There must be exactly 8 values")
    }
    LazyColumn(
        userScrollEnabled = true,
    ) {
        values.forEachIndexed { index, value ->
            item {
                Column {
                    Row(
                        horizontalArrangement = Arrangement.SpaceBetween,
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 8.dp)
                    ) {
                        val hz = (100 * 2F.pow(index)).roundToInt()
                        Text(stringResource(R.string.hz, hz))
                        Text(formatVolume(value))
                    }
                    EqualizerSlider(value.toFloat(), onValueChange = { value ->
                        onValueChange(index, value.roundToInt().toByte())
                    })
                }
                Divider(modifier = Modifier.padding(vertical = 4.dp))
            }
        }
    }
}

private fun formatVolume(volume: Byte): String {
    val format = NumberFormat.getInstance()
    format.minimumFractionDigits = 1
    format.maximumFractionDigits = 1
    format.minimumIntegerDigits = 1
    return format.format(volume / 10F)
}

@Composable
private fun EqualizerSlider(value: Float, onValueChange: (value: Float) -> Unit) {
    Slider(
        value = value,
        onValueChange = onValueChange,
        valueRange = -60F..60F,
        steps = 120,
        modifier = Modifier.testTag("equalizerSlider"),
    )
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    var values by remember { mutableStateOf(listOf<Byte>(0, 0, 0, 0, 0, 0, 0, 0)) }
    OpenSCQ30Theme {
        Equalizer(
            values = values,
            onValueChange = { changedIndex, changedValue ->
                values = values.mapIndexed { index, value ->
                    return@mapIndexed if (index == changedIndex) {
                        changedValue
                    } else {
                        value
                    }
                }
            },
        )
    }
}
