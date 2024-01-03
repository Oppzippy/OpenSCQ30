package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel

@Composable
fun AmbientSoundModeCycleSelection(
    cycle: AmbientSoundModeCycle,
    onAmbientSoundModeCycleChange: (cycle: AmbientSoundModeCycle) -> Unit,
    hasNoiseCanceling: Boolean,
) {
    Column(Modifier.testTag("ambientSoundModeCycleSelection")) {
        CheckboxWithLabel(
            text = stringResource(R.string.normal),
            isChecked = cycle.normalMode,
            onCheckedChange = { isChecked ->
                onAmbientSoundModeCycleChange(
                    cycle.copy(normalMode = isChecked),
                )
            },
        )
        CheckboxWithLabel(
            text = stringResource(R.string.transparency),
            isChecked = cycle.transparencyMode,
            onCheckedChange = { isChecked ->
                onAmbientSoundModeCycleChange(
                    cycle.copy(transparencyMode = isChecked),
                )
            },
        )
        if (hasNoiseCanceling) {
            CheckboxWithLabel(
                text = stringResource(R.string.noise_canceling),
                isChecked = cycle.noiseCancelingMode,
                onCheckedChange = { isChecked ->
                    onAmbientSoundModeCycleChange(
                        cycle.copy(noiseCancelingMode = isChecked),
                    )
                },
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewAmbientSoundModeSelection() {
    OpenSCQ30Theme {
        AmbientSoundModeCycleSelection(
            cycle = AmbientSoundModeCycle(
                normalMode = true,
                transparencyMode = false,
                noiseCancelingMode = true,
            ),
            onAmbientSoundModeCycleChange = {},
            hasNoiseCanceling = true,
        )
    }
}
