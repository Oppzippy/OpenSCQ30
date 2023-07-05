package com.oppzippy.openscq30.ui.quickpresets.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Divider
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.soundmode.AmbientSoundModeSelection
import com.oppzippy.openscq30.ui.soundmode.NoiseCancelingModeSelection
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel
import com.oppzippy.openscq30.ui.utils.Dropdown
import com.oppzippy.openscq30.ui.utils.DropdownOption

@Composable
fun QuickPresetConfiguration(
    ambientSoundMode: AmbientSoundMode?,
    noiseCancelingMode: NoiseCancelingMode?,
    equalizerProfileName: String?,
    allEqualizerProfileNames: List<String>,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode?) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode?) -> Unit = {},
    onEqualizerProfileNameChange: (profileName: String?) -> Unit = {},
) {
    Column(Modifier.verticalScroll(rememberScrollState())) {
        CheckboxWithLabel(
            text = stringResource(R.string.ambient_sound_mode),
            isChecked = ambientSoundMode != null,
            onCheckedChange = {
                onAmbientSoundModeChange(if (it) AmbientSoundMode.Normal else null)
            },
        )
        if (ambientSoundMode != null) {
            AmbientSoundModeSelection(
                ambientSoundMode = ambientSoundMode,
                onAmbientSoundModeChange = onAmbientSoundModeChange,
            )
        }
        Divider()
        CheckboxWithLabel(
            text = stringResource(R.string.noise_canceling_mode),
            isChecked = noiseCancelingMode != null,
            onCheckedChange = {
                onNoiseCancelingModeChange(if (it) NoiseCancelingMode.Transport else null)
            },
        )
        if (noiseCancelingMode != null) {
            NoiseCancelingModeSelection(
                noiseCancelingMode = noiseCancelingMode,
                onNoiseCancelingModeChange = onNoiseCancelingModeChange,
            )
        }
        Divider()

        var isEqualizerChecked by remember { mutableStateOf(equalizerProfileName != null) }
        CheckboxWithLabel(
            text = stringResource(R.string.equalizer),
            isChecked = equalizerProfileName != null || isEqualizerChecked,
            onCheckedChange = {
                isEqualizerChecked = it
                if (!isEqualizerChecked) {
                    onEqualizerProfileNameChange(null)
                }
            },
        )
        if (equalizerProfileName != null || isEqualizerChecked) {
            Dropdown(
                value = equalizerProfileName,
                options = allEqualizerProfileNames.map {
                    DropdownOption(
                        name = it,
                        value = it,
                        label = { Text(it) },
                    )
                },
                label = stringResource(R.string.custom_profile),
                onItemSelected = onEqualizerProfileNameChange,
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewQuickPresetConfiguration() {
    OpenSCQ30Theme {
        QuickPresetConfiguration(
            ambientSoundMode = AmbientSoundMode.NoiseCanceling,
            noiseCancelingMode = NoiseCancelingMode.Transport,
            equalizerProfileName = "test",
            allEqualizerProfileNames = emptyList(),
        )
    }
}
