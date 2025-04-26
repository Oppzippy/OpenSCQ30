package com.oppzippy.openscq30.ui.devicesettings.composables.equalizer

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import java.math.BigDecimal
import kotlin.math.pow
import kotlin.math.roundToInt

@Composable
fun EqualizerSlider(
    hz: UShort,
    value: Short,
    minValue: Short,
    maxValue: Short,
    fractionDigits: Short,
    onValueChange: (Short) -> Unit,
) {
    Column {
        Row {
            var text by remember(hz, value) {
                mutableStateOf(BigDecimal(value.toInt()).scaleByPowerOfTen(-fractionDigits).toString())
            }
            TextField(
                value = text,
                onValueChange = {
                    onValueChange(BigDecimal(it).scaleByPowerOfTen(fractionDigits.toInt()).toShort())
                },
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
                modifier = Modifier
                    .testTag("equalizerInput")
                    .width(100.dp),
                singleLine = true,
                label = {
                    if (hz < 1000u) {
                        Text(stringResource(R.string.hz, hz))
                    } else {
                        Text(
                            stringResource(
                                R.string.khz,
                                BigDecimal(hz.toInt()).divide(BigDecimal(1000)).toString(),
                            ),
                        )
                    }
                },
            )
            Slider(
                value = value.toFloat(),
                onValueChange = {
                    onValueChange((it * 10f.pow(fractionDigits.toInt())).roundToInt().toShort())
                },
                valueRange = minValue.toFloat()..maxValue.toFloat(),
                // The min/max values are not included in the steps number
                steps = (maxValue - minValue) - 1,
                modifier = Modifier.testTag("equalizerSlider"),
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewEqualizerSlider() {
    OpenSCQ30Theme {
        EqualizerSlider(
            hz = 100u,
            value = 0,
            onValueChange = {},
            maxValue = 135,
            minValue = -120,
            fractionDigits = 1,
        )
    }
}
