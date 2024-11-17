package com.oppzippy.openscq30.ui.soundmodestypetwo

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.ManualNoiseCanceling
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledRadioButtonGroup

@Composable
fun ManualNoiseCancelingSelection(
    manualNoiseCanceling: ManualNoiseCanceling,
    onManualNoiseCancelingChange: (manualNoiseCanceling: ManualNoiseCanceling) -> Unit,
) {
    LabeledRadioButtonGroup(
        selectedValue = manualNoiseCanceling,
        values = linkedMapOf(
            Pair(ManualNoiseCanceling.Weak, stringResource(R.string.weak)),
            Pair(ManualNoiseCanceling.Moderate, stringResource(R.string.moderate)),
            Pair(ManualNoiseCanceling.Strong, stringResource(R.string.strong)),
        ),
        onValueChange = onManualNoiseCancelingChange,
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewManualNoiseCancelingSelection() {
    OpenSCQ30Theme {
        ManualNoiseCancelingSelection(
            manualNoiseCanceling = ManualNoiseCanceling.Strong,
            onManualNoiseCancelingChange = {},
        )
    }
}
