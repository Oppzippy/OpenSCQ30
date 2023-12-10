package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledRadioButtonGroup

@Composable
fun NoiseCancelingModeSelection(
    noiseCancelingMode: NoiseCancelingMode,
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit,
    hasCustomNoiseCanceling: Boolean,
) {
    val values = linkedMapOf(
        Pair(NoiseCancelingMode.Transport, stringResource(R.string.transport)),
        Pair(NoiseCancelingMode.Indoor, stringResource(R.string.indoor)),
        Pair(NoiseCancelingMode.Outdoor, stringResource(R.string.outdoor)),
    )
    if (hasCustomNoiseCanceling) {
        values[NoiseCancelingMode.Custom] = stringResource(R.string.custom)
    }

    LabeledRadioButtonGroup(
        selectedValue = noiseCancelingMode,
        values = values,
        onValueChange = onNoiseCancelingModeChange,
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewNoiseCancelingModeSelection() {
    OpenSCQ30Theme {
        NoiseCancelingModeSelection(
            noiseCancelingMode = NoiseCancelingMode.Transport,
            onNoiseCancelingModeChange = {},
            hasCustomNoiseCanceling = true,
        )
    }
}
