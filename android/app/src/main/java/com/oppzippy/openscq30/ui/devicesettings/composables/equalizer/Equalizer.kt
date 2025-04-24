package com.oppzippy.openscq30.ui.devicesettings.composables.equalizer

import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
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
import kotlin.math.pow
import kotlin.math.roundToInt

@Composable
fun Equalizer(
    values: List<Double>,
    onValueChange: (index: Int, value: Double) -> Unit,
    texts: List<String>,
    onTextChanged: (index: Int, value: String) -> Unit,
) {
    if (values.size != 8 || texts.size != 8) {
        throw IllegalArgumentException("There must be exactly 8 values")
    }
    LazyColumn(
        userScrollEnabled = true,
    ) {
        values.forEachIndexed { index, value ->
            item {
                EqualizerSlider(
                    hz = (100 * 2F.pow(index)).roundToInt(),
                    value = value,
                    onValueChange = {
                        onValueChange(index, it)
                    },
                    text = texts[index],
                    onTextChange = {
                        onTextChanged(index, it)
                    },
                )
                HorizontalDivider(modifier = Modifier.padding(vertical = 4.dp))
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewEqualizer() {
    var values by remember { mutableStateOf(listOf(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)) }
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
            texts = values.map { it.toString() },
            onTextChanged = { _, _ -> },
        )
    }
}
