package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.lib.extensions.resources.toStringResource
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledRadioButtonGroup

@Composable
fun NoiseCancelingModeSelection(
    noiseCancelingMode: NoiseCancelingMode,
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit,
    availableSoundModes: List<NoiseCancelingMode>,
) {
    val values = availableSoundModes.associateWith { stringResource(it.toStringResource()) }
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
            availableSoundModes = NoiseCancelingMode.entries,
        )
    }
}
