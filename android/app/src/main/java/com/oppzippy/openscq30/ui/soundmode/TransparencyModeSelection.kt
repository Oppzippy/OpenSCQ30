package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.lib.extensions.resources.toStringResource
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledRadioButtonGroup

@Composable
fun TransparencyModeSelection(
    transparencyMode: TransparencyMode,
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit,
    availableSoundModes: List<TransparencyMode>,
) {
    val values = availableSoundModes.associateWith { stringResource(it.toStringResource()) }

    LabeledRadioButtonGroup(
        selectedValue = transparencyMode,
        values = values,
        onValueChange = onTransparencyModeChange,
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewAmbientSoundModeSelection() {
    OpenSCQ30Theme {
        TransparencyModeSelection(
            transparencyMode = TransparencyMode.VocalMode,
            onTransparencyModeChange = {},
            availableSoundModes = TransparencyMode.entries,
        )
    }
}
