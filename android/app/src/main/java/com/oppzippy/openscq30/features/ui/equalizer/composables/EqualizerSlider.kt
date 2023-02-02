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
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.ui.equalizer.models.EqualizerSliderViewModel
import java.text.NumberFormat
import kotlin.math.roundToInt

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun EqualizerSlider(
    hz: Int,
    value: Byte,
    onValueChange: (value: Byte) -> Unit,
    enabled: Boolean = true,
    viewModel: EqualizerSliderViewModel = viewModel(key = hz.toString()),
) {
    val displayedText by viewModel.displayedText.collectAsState()
    viewModel.checkForValueChange(value)

    Column {
        Row {
            if (enabled) {
                TextField(
                    value = displayedText,
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                    modifier = Modifier.width(100.dp),
                    label = { Text(stringResource(R.string.hz, hz)) },
                    onValueChange = { viewModel.onValueChange(it, onValueChange) },
                    visualTransformation = { viewModel.transformText(it) },
                )
            } else {
                TextField(
                    value = viewModel.formatVolume(value),
                    onValueChange = {},
                    modifier = Modifier.width(100.dp),
                    label = { Text(stringResource(R.string.hz, hz)) },
                    enabled = false,
                )
            }
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

