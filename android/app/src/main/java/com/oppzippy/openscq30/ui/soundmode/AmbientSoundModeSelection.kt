package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledRadioButtonGroup

@Composable
fun AmbientSoundModeSelection(
    ambientSoundMode: AmbientSoundMode,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit,
    hasNoiseCanceling: Boolean,
) {
    val values = linkedMapOf(
        Pair(AmbientSoundMode.Normal, stringResource(R.string.normal)),
        Pair(AmbientSoundMode.Transparency, stringResource(R.string.transparency)),
    )
    if (hasNoiseCanceling) {
        values[AmbientSoundMode.NoiseCanceling] = stringResource(R.string.noise_canceling)
    }

    LabeledRadioButtonGroup(
        selectedValue = ambientSoundMode,
        values = values,
        onValueChange = onAmbientSoundModeChange,
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewAmbientSoundModeSelection() {
    OpenSCQ30Theme {
        AmbientSoundModeSelection(
            ambientSoundMode = AmbientSoundMode.Normal,
            onAmbientSoundModeChange = {},
            hasNoiseCanceling = true,
        )
    }
}
