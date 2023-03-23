package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import java.math.BigDecimal
import kotlin.math.roundToInt

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun EqualizerSlider(
    hz: Int,
    value: Byte,
    onValueChange: (value: Byte) -> Unit,
    text: String,
    onTextChange: (text: String) -> Unit,
    enabled: Boolean = true,
) {
    Column {
        Row {
            TextField(
                value = text,
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                modifier = Modifier.testTag("equalizerInput").width(100.dp),
                label = {
                    if (hz < 1000) {
                        Text(stringResource(R.string.hz, hz))
                    } else {
                        Text(
                            stringResource(
                                R.string.khz,
                                BigDecimal(hz).divide(BigDecimal(1000)).toString(),
                            )
                        )
                    }
                },
                onValueChange = onTextChange,
                enabled = enabled
            )
            Slider(
                value = value.toFloat(),
                onValueChange = {
                    onValueChange(it.roundToInt().toByte())
                },
                valueRange = -60F..60F,
                steps = 120,
                modifier = Modifier.testTag("equalizerSlider"),
                enabled = enabled,
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        EqualizerSlider(hz = 100, value = 0, onValueChange = {}, text = "0", onTextChange = {})
    }
}
