package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.material3.Slider
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import kotlin.math.roundToInt

@Composable
fun CustomNoiseCancelingSelection(
    customNoiseCanceling: UByte,
    onCustomNoiseCancelingChange: (customNoiseCanceling: UByte) -> Unit,
) {
    Slider(
        value = customNoiseCanceling.toFloat(),
        onValueChange = {
            onCustomNoiseCancelingChange(it.roundToInt().toUByte())
        },
        valueRange = 0f..10f,
        steps = 11,
        modifier = Modifier.testTag("customNoiseCancelingSlider"),
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewAmbientSoundModeSelection() {
    OpenSCQ30Theme {
        CustomNoiseCancelingSelection(
            customNoiseCanceling = 1u,
            onCustomNoiseCancelingChange = {},
        )
    }
}
