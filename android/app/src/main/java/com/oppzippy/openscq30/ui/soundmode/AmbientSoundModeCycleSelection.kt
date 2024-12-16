package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.lib.extensions.resources.toStringResource
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel

@Composable
fun AmbientSoundModeCycleSelection(
    cycle: AmbientSoundModeCycle,
    onAmbientSoundModeCycleChange: (cycle: AmbientSoundModeCycle) -> Unit,
    availableSoundModes: List<AmbientSoundMode>,
) {
    Column(Modifier.testTag("ambientSoundModeCycleSelection")) {
        availableSoundModes.forEach { mode ->
            CheckboxWithLabel(
                text = stringResource(mode.toStringResource()),
                isChecked = when (mode) {
                    AmbientSoundMode.Normal -> cycle.normalMode
                    AmbientSoundMode.Transparency -> cycle.transparencyMode
                    AmbientSoundMode.NoiseCanceling -> cycle.noiseCancelingMode
                },
                onCheckedChange = { isChecked ->
                    onAmbientSoundModeCycleChange(
                        when (mode) {
                            AmbientSoundMode.Normal -> cycle.copy(normalMode = isChecked)
                            AmbientSoundMode.Transparency -> cycle.copy(transparencyMode = isChecked)
                            AmbientSoundMode.NoiseCanceling -> cycle.copy(noiseCancelingMode = isChecked)
                        },
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
            availableSoundModes = AmbientSoundMode.entries,
        )
    }
}
