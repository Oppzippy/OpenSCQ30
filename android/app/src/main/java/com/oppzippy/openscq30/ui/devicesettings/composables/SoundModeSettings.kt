package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.selection.selectableGroup
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun SoundModeSettings(
    modifier: Modifier = Modifier,
    ambientSoundMode: AmbientSoundMode,
    noiseCancelingMode: NoiseCancelingMode,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
) {
    Column(modifier = modifier) {
        GroupHeader(stringResource(R.string.ambient_sound_mode))
        LabeledRadioButtonGroup(
            selectedValue = ambientSoundMode,
            values = linkedMapOf(
                Pair(AmbientSoundMode.Normal, stringResource(R.string.normal)),
                Pair(AmbientSoundMode.Transparency, stringResource(R.string.transparency)),
                Pair(AmbientSoundMode.NoiseCanceling, stringResource(R.string.noise_canceling)),
            ),
            onValueChange = onAmbientSoundModeChange,
        )
        Spacer(modifier = Modifier.padding(vertical = 8.dp))
        GroupHeader(stringResource(R.string.noise_canceling_mode))
        LabeledRadioButtonGroup(
            selectedValue = noiseCancelingMode,
            values = linkedMapOf(
                Pair(NoiseCancelingMode.Transport, stringResource(R.string.transport)),
                Pair(NoiseCancelingMode.Indoor, stringResource(R.string.indoor)),
                Pair(NoiseCancelingMode.Outdoor, stringResource(R.string.outdoor)),
            ),
            onValueChange = onNoiseCancelingModeChange,
        )
    }
}

@Composable
private fun GroupHeader(text: String) {
    Text(
        text = text,
        style = MaterialTheme.typography.titleMedium,
        modifier = Modifier.padding(horizontal = 2.dp, vertical = 2.dp),
    )
}

@Composable
private fun <T> LabeledRadioButtonGroup(
    selectedValue: T,
    values: LinkedHashMap<T, String>,
    onValueChange: (value: T) -> Unit,
) {
    Column(Modifier.selectableGroup()) {
        values.forEach { (value, text) ->
            LabeledRadioButton(text = text, selected = selectedValue == value, onClick = {
                onValueChange(value)
            })
        }
    }
}

@Composable
private fun LabeledRadioButton(text: String, selected: Boolean, onClick: () -> Unit) {
    Row(
        Modifier
            .fillMaxWidth()
            .selectable(selected = selected, onClick = onClick, role = Role.RadioButton)
            .padding(horizontal = 2.dp, vertical = 2.dp),
    ) {
        RadioButton(selected = selected, onClick = null)
        Text(
            text = text,
            modifier = Modifier.padding(start = 8.dp),
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        SoundModeSettings(
            ambientSoundMode = AmbientSoundMode.Normal,
            noiseCancelingMode = NoiseCancelingMode.Indoor,
        )
    }
}
