package com.oppzippy.openscq30.ui.soundmodestypetwo

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.AdaptiveNoiseCanceling
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledRadioButtonGroup

@Composable
fun AdaptiveNoiseCancelingSelection(adaptiveNoiseCanceling: AdaptiveNoiseCanceling) {
    LabeledRadioButtonGroup(
        selectedValue = adaptiveNoiseCanceling,
        values = linkedMapOf(
            Pair(AdaptiveNoiseCanceling.LowNoise, stringResource(R.string.low_noise)),
            Pair(AdaptiveNoiseCanceling.MediumNoise, stringResource(R.string.medium_noise)),
            Pair(AdaptiveNoiseCanceling.HighNoise, stringResource(R.string.high_noise)),
        ),
        onValueChange = {},
        enabled = false,
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewAdaptiveNoiseCancelingSelection() {
    OpenSCQ30Theme {
        AdaptiveNoiseCancelingSelection(
            adaptiveNoiseCanceling = AdaptiveNoiseCanceling.HighNoise,
        )
    }
}
