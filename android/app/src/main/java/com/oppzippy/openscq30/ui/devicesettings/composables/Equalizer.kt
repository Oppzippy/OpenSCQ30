package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.HorizontalDivider
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
import com.oppzippy.openscq30.ui.utils.throttledState
import java.math.BigDecimal
import kotlin.math.pow
import kotlin.math.roundToInt

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
            EqualizerRow(
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

private val removeSecondDecimalRegex = Regex("^([^.]*\\.[^.]*)\\..*$")

@Composable
private fun EqualizerRow(
    hz: UShort,
    value: Short,
    minValue: Short,
    maxValue: Short,
    fractionDigits: Short,
    onValueChange: (Short) -> Unit,
) {
    val (displayedValue, setDisplayedValue) = throttledState(
        value,
        250,
        onValueChange = { onValueChange(it) },
    )

    Column {
        Row {
            EqualizerTextInput(
                hz = hz,
                value = displayedValue,
                minValue = minValue,
                maxValue = maxValue,
                fractionDigits = fractionDigits,
                onValueChange = setDisplayedValue,
            )
            EqualizerSlider(
                value = displayedValue,
                minValue = minValue,
                maxValue = maxValue,
                fractionDigits = fractionDigits,
                onValueChange = setDisplayedValue,
            )
        }
    }
}

@Composable
private fun EqualizerTextInput(
    hz: UShort,
    value: Short,
    minValue: Short,
    maxValue: Short,
    fractionDigits: Short,
    onValueChange: (Short) -> Unit,
) {
    var text by remember(hz, value) {
        mutableStateOf(BigDecimal(value.toInt()).scaleByPowerOfTen(-fractionDigits).toString())
    }
    val matches = removeSecondDecimalRegex.matchEntire(text)
    val reformattedText = if (matches != null) {
        matches.groupValues[1]
    } else {
        text
    }
    TextField(
        value = reformattedText,
        onValueChange = {
            text = it
            try {
                val newValue = BigDecimal(it).scaleByPowerOfTen(fractionDigits.toInt()).toShort()
                if (newValue in minValue..maxValue) {
                    onValueChange(newValue)
                }
            } catch (_: NumberFormatException) {
            }
        },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
        modifier = Modifier
            .testTag("equalizerInput")
            .width(100.dp),
        singleLine = true,
        label = {
            if (hz < 1000u) {
                Text(stringResource(R.string.hz, hz.toInt()))
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
}

@Composable
private fun EqualizerSlider(
    value: Short,
    minValue: Short,
    maxValue: Short,
    fractionDigits: Short,
    onValueChange: (Short) -> Unit,
) {
    val divisor = 10f.pow(fractionDigits.toInt())
    Slider(
        value = value.toFloat() / divisor,
        onValueChange = {
            onValueChange((it * divisor).roundToInt().toShort())
        },
        valueRange = (minValue.toFloat() / divisor)..(maxValue.toFloat() / divisor),
        // The min/max values are not included in the steps number
        steps = (maxValue - minValue) - 1,
        modifier = Modifier.testTag("equalizerSlider"),
    )
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
