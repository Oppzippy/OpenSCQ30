package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledRadioButtonGroup

@Composable
fun TransparencyModeSelection(
    transparencyMode: TransparencyMode,
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit,
) {
    LabeledRadioButtonGroup(
        selectedValue = transparencyMode,
        values = linkedMapOf(
            Pair(TransparencyMode.FullyTransparent, stringResource(R.string.fully_transparent)),
            Pair(TransparencyMode.VocalMode, stringResource(R.string.vocal_mode)),
        ),
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
        )
    }
}
