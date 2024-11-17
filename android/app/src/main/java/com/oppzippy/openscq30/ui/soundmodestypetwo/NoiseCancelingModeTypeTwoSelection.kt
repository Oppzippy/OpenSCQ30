package com.oppzippy.openscq30.ui.soundmodestypetwo

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingModeTypeTwo
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledRadioButtonGroup

@Composable
fun NoiseCancelingModeTypeTwoSelection(
    noiseCancelingMode: NoiseCancelingModeTypeTwo,
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingModeTypeTwo) -> Unit,
) {
    LabeledRadioButtonGroup(
        selectedValue = noiseCancelingMode,
        values = linkedMapOf(
            Pair(NoiseCancelingModeTypeTwo.Manual, stringResource(R.string.manual)),
            Pair(NoiseCancelingModeTypeTwo.Adaptive, stringResource(R.string.adaptive)),
        ),
        onValueChange = onNoiseCancelingModeChange,
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewNoiseCancelingModeTypeTwoSelection() {
    OpenSCQ30Theme {
        NoiseCancelingModeTypeTwoSelection(
            noiseCancelingMode = NoiseCancelingModeTypeTwo.Manual,
            onNoiseCancelingModeChange = {},
        )
    }
}
