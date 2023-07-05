package com.oppzippy.openscq30.ui.soundmode

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
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
        AmbientSoundModeSelection(
            ambientSoundMode = ambientSoundMode,
            onAmbientSoundModeChange = onAmbientSoundModeChange,
        )
        Spacer(modifier = Modifier.padding(vertical = 8.dp))
        GroupHeader(stringResource(R.string.noise_canceling_mode))
        NoiseCancelingModeSelection(
            noiseCancelingMode = noiseCancelingMode,
            onNoiseCancelingModeChange = onNoiseCancelingModeChange,
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

@Preview(showBackground = true)
@Composable
private fun PreviewSoundModeSettings() {
    OpenSCQ30Theme {
        SoundModeSettings(
            ambientSoundMode = AmbientSoundMode.Normal,
            noiseCancelingMode = NoiseCancelingMode.Indoor,
        )
    }
}
