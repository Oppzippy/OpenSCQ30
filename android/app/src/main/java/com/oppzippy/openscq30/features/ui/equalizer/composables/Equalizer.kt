package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.OffsetMapping
import androidx.compose.ui.text.input.TransformedText
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.ui.equalizer.models.DecimalSeparatorOffsetMapping
import java.text.NumberFormat
import kotlin.math.pow
import kotlin.math.roundToInt

@Composable
fun Equalizer(
    values: List<Byte>, onValueChange: (index: Int, value: Byte) -> Unit, enabled: Boolean = true,
) {
    if (values.size != 8) {
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
                    onValueChange = { value ->
                        onValueChange(index, value)
                    },
                    enabled = enabled,
                )
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

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun EqualizerSlider(
    hz: Int,
    value: Byte,
    onValueChange: (value: Byte) -> Unit,
    enabled: Boolean = true,
) {
    var displayedText by remember { mutableStateOf(value.toString()) }
    var lastValue by remember { mutableStateOf(value) }
    if (lastValue != value) {
        lastValue = value
        displayedText = value.toString()
    }

    Column {
        Row {
            if (enabled) {
                TextField(
                    value = displayedText,
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                    modifier = Modifier.width(100.dp),
                    label = { Text(stringResource(R.string.hz, hz)) },
                    onValueChange = { text ->
                        // TODO clean up this mess
                        Regex("^(-?)(\\d?)(\\d?)(\\d*)$").matchEntire(text)?.let {
                            displayedText = text
                            val tens = it.groupValues[2]
                            val ones = it.groupValues[3]
                            if (tens.isNotEmpty()) {
                                try {
                                    var value = tens.toInt() * 10
                                    var divisor = 10
                                    if (ones.isNotEmpty()) {
                                        value += ones.toInt()
                                        divisor = 1
                                    }
                                    if (it.groupValues[1].isNotEmpty()) {
                                        value *= -1
                                    }
                                    displayedText = value.div(divisor).toString()
                                    lastValue = value.toByte()
                                    onValueChange(value.toByte())
                                } catch (_: NumberFormatException) {
                                }
                            }
                        }
                    },
                    visualTransformation = {
                        try {
                            val number = it.text.toInt()
                            if (number != 0) {
                                val newString = it.text.replace(Regex("(\\d)(\\d)"), "$1\\.$2")
                                if (newString != it.text) {
                                    return@TextField TransformedText(
                                        AnnotatedString(
                                            newString,
                                            it.spanStyles,
                                            it.paragraphStyles,
                                        ),
                                        DecimalSeparatorOffsetMapping(it.text.length - 2),
                                    )
                                }
                            }
                        } catch (_: NumberFormatException) {
                        }
                        return@TextField TransformedText(it, OffsetMapping.Identity)
                    },
                )
            } else {
                TextField(
                    value = formatVolume(value),
                    onValueChange = {},
                    modifier = Modifier.width(100.dp),
                    label = { Text(stringResource(R.string.hz, hz)) },
                    enabled = false,
                )
            }
            Slider(
                value = value.toFloat(),
                onValueChange = {
                    displayedText = it.roundToInt().toString()
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
