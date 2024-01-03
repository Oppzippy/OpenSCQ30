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
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.lib.wrapper.SoundModes
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun SoundModeSettings(
    modifier: Modifier = Modifier,
    soundModes: SoundModes,
    ambientSoundModeCycle: AmbientSoundModeCycle?,
    hasTransparencyModes: Boolean,
    noiseCancelingType: NoiseCancelingType,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onCustomNoiseCancelingChange: (customNoiseCanceling: UByte) -> Unit = {},
    onAmbientSoundModeCycleChange: (cycle: AmbientSoundModeCycle) -> Unit = {},
) {
    Column(modifier = modifier) {
        GroupHeader(stringResource(R.string.ambient_sound_mode))
        AmbientSoundModeSelection(
            ambientSoundMode = soundModes.ambientSoundMode,
            onAmbientSoundModeChange = onAmbientSoundModeChange,
            hasNoiseCanceling = noiseCancelingType != NoiseCancelingType.None,
        )
        if (ambientSoundModeCycle != null) {
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
            GroupHeader(stringResource(R.string.ambient_sound_mode_cycle))
            AmbientSoundModeCycleSelection(
                cycle = ambientSoundModeCycle,
                onAmbientSoundModeCycleChange = onAmbientSoundModeCycleChange,
                hasNoiseCanceling = noiseCancelingType != NoiseCancelingType.None,
            )
        }
        if (hasTransparencyModes) {
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
            GroupHeader(stringResource(R.string.transparency_mode))
            TransparencyModeSelection(
                transparencyMode = soundModes.transparencyMode,
                onTransparencyModeChange = onTransparencyModeChange,
            )
        }
        if (noiseCancelingType != NoiseCancelingType.None) {
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
            GroupHeader(stringResource(R.string.noise_canceling_mode))
            NoiseCancelingModeSelection(
                noiseCancelingMode = soundModes.noiseCancelingMode,
                onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                hasCustomNoiseCanceling = noiseCancelingType == NoiseCancelingType.Custom,
            )
        }
        if (noiseCancelingType == NoiseCancelingType.Custom && soundModes.noiseCancelingMode == NoiseCancelingMode.Custom) {
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
            GroupHeader(stringResource(R.string.custom_noise_canceling))
            CustomNoiseCancelingSelection(
                customNoiseCanceling = soundModes.customNoiseCanceling,
                onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
            )
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
            hasTransparencyModes = true,
            noiseCancelingType = NoiseCancelingType.Custom,
        )
    }
}
