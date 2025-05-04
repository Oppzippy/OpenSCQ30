package com.oppzippy.openscq30.ui.devicesettings.composables.equalizer

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.HorizontalDivider
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun Equalizer(
    bands: List<UShort>,
    values: List<Short>,
    minValue: Short,
    maxValue: Short,
    fractionDigits: Short,
    onValueChange: (index: Int, value: Short) -> Unit,
) {
    Column {
        bands.zip(values).forEachIndexed { index, (hz, value) ->
            EqualizerSlider(
                hz = hz,
                value = value,
                minValue = minValue,
                maxValue = maxValue,
                fractionDigits = fractionDigits,
                onValueChange = {
                    onValueChange(index, it)
                },
            )
            HorizontalDivider(modifier = Modifier.padding(vertical = 4.dp))
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewEqualizer() {
    val bands = listOf<UShort>(100u, 200u, 400u, 800u, 1600u, 3200u, 6400u, 12000u)
    var values by remember { mutableStateOf(listOf<Short>(0, 0, 0, 0, 0, 0, 0, 0)) }
    OpenSCQ30Theme {
        Equalizer(
            bands = bands,
            values = values,
            minValue = -120,
            maxValue = 135,
            fractionDigits = 1,
            onValueChange = { changedIndex, changedValue ->
                values = values.mapIndexed { index, value ->
                    if (index == changedIndex) {
                        changedValue
                    } else {
                        value
                    }
                }
            },
        )
    }
}
