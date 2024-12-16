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
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.lib.wrapper.AvailableSoundModes
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.lib.wrapper.SoundModes
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun SoundModeSettings(
    modifier: Modifier = Modifier,
    soundModes: SoundModes,
    ambientSoundModeCycle: AmbientSoundModeCycle?,
    availableSoundModes: AvailableSoundModes,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onCustomNoiseCancelingChange: (customNoiseCanceling: UByte) -> Unit = {},
    onAmbientSoundModeCycleChange: (cycle: AmbientSoundModeCycle) -> Unit = {},
) {
    Column(modifier = modifier) {
        if (availableSoundModes.ambientSoundModes.isNotEmpty()) {
            GroupHeader(stringResource(R.string.ambient_sound_mode))
            AmbientSoundModeSelection(
                ambientSoundMode = soundModes.ambientSoundMode,
                onAmbientSoundModeChange = onAmbientSoundModeChange,
                availableSoundModes = availableSoundModes.ambientSoundModes,
            )
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
        }
        if (ambientSoundModeCycle != null) {
            GroupHeader(stringResource(R.string.ambient_sound_mode_cycle))
            AmbientSoundModeCycleSelection(
                cycle = ambientSoundModeCycle,
                onAmbientSoundModeCycleChange = onAmbientSoundModeCycleChange,
                availableSoundModes = availableSoundModes.ambientSoundModes,
            )
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
        }
        if (availableSoundModes.transparencyModes.isNotEmpty()) {
            GroupHeader(stringResource(R.string.transparency_mode))
            TransparencyModeSelection(
                transparencyMode = soundModes.transparencyMode,
                onTransparencyModeChange = onTransparencyModeChange,
                availableSoundModes = availableSoundModes.transparencyModes,
            )
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
        }
        if (availableSoundModes.noiseCancelingModes.isNotEmpty()) {
            GroupHeader(stringResource(R.string.noise_canceling_mode))
            NoiseCancelingModeSelection(
                noiseCancelingMode = soundModes.noiseCancelingMode,
                onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                availableSoundModes = availableSoundModes.noiseCancelingModes,
            )
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
        }
        if (availableSoundModes.customNoiseCanceling) {
            GroupHeader(stringResource(R.string.custom_noise_canceling))
            CustomNoiseCancelingSelection(
                customNoiseCanceling = soundModes.customNoiseCanceling,
                onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
            )
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
        }
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
            soundModes = SoundModes(
                AmbientSoundMode.Normal,
                NoiseCancelingMode.Indoor,
                TransparencyMode.VocalMode,
                0u,
            ),
            ambientSoundModeCycle = AmbientSoundModeCycle(
                normalMode = true,
                transparencyMode = false,
                noiseCancelingMode = true,
            ),
            availableSoundModes = AvailableSoundModes(
                ambientSoundModes = AmbientSoundMode.entries,
                transparencyModes = TransparencyMode.entries,
                noiseCancelingModes = NoiseCancelingMode.entries,
                customNoiseCanceling = true,
            ),
        )
    }
}
