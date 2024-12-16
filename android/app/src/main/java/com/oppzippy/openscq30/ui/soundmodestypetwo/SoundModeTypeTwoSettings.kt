package com.oppzippy.openscq30.ui.soundmodestypetwo

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
import com.oppzippy.openscq30.lib.wrapper.AdaptiveNoiseCanceling
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.lib.wrapper.ManualNoiseCanceling
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingModeTypeTwo
import com.oppzippy.openscq30.lib.wrapper.SoundModesTypeTwo
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.ui.soundmode.AmbientSoundModeCycleSelection
import com.oppzippy.openscq30.ui.soundmode.AmbientSoundModeSelection
import com.oppzippy.openscq30.ui.soundmode.TransparencyModeSelection
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun SoundModeTypeTwoSettings(
    modifier: Modifier = Modifier,
    soundModes: SoundModesTypeTwo,
    ambientSoundModeCycle: AmbientSoundModeCycle?,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingModeTypeTwo) -> Unit = {},
    onManualNoiseCancelingChange: (manualNoiseCanceling: ManualNoiseCanceling) -> Unit = {},
    onAmbientSoundModeCycleChange: (cycle: AmbientSoundModeCycle) -> Unit = {},
) {
    Column(modifier = modifier) {
        GroupHeader(stringResource(R.string.ambient_sound_mode))
        AmbientSoundModeSelection(
            ambientSoundMode = soundModes.ambientSoundMode,
            onAmbientSoundModeChange = onAmbientSoundModeChange,
            availableSoundModes = listOf(
                AmbientSoundMode.Normal,
                AmbientSoundMode.Transparency,
                AmbientSoundMode.NoiseCanceling,
            ),
        )
        if (ambientSoundModeCycle != null) {
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
            GroupHeader(stringResource(R.string.ambient_sound_mode_cycle))
            AmbientSoundModeCycleSelection(
                cycle = ambientSoundModeCycle,
                onAmbientSoundModeCycleChange = onAmbientSoundModeCycleChange,
                availableSoundModes = listOf(
                    AmbientSoundMode.Normal,
                    AmbientSoundMode.Transparency,
                    AmbientSoundMode.NoiseCanceling,
                ),
            )
        }
        Spacer(modifier = Modifier.padding(vertical = 8.dp))
        GroupHeader(stringResource(R.string.transparency_mode))
        TransparencyModeSelection(
            transparencyMode = soundModes.transparencyMode,
            onTransparencyModeChange = onTransparencyModeChange,
            availableSoundModes = listOf(TransparencyMode.FullyTransparent, TransparencyMode.VocalMode),
        )
        Spacer(modifier = Modifier.padding(vertical = 8.dp))
        GroupHeader(stringResource(R.string.noise_canceling_mode))
        NoiseCancelingModeTypeTwoSelection(
            noiseCancelingMode = soundModes.noiseCancelingMode,
            onNoiseCancelingModeChange = onNoiseCancelingModeChange,
        )
        when (soundModes.noiseCancelingMode) {
            NoiseCancelingModeTypeTwo.Adaptive -> {
                Spacer(modifier = Modifier.padding(vertical = 8.dp))
                GroupHeader(stringResource(R.string.adaptive_noise_canceling))
                AdaptiveNoiseCancelingSelection(
                    adaptiveNoiseCanceling = soundModes.adaptiveNoiseCanceling,
                )
            }

            NoiseCancelingModeTypeTwo.Manual -> {
                Spacer(modifier = Modifier.padding(vertical = 8.dp))
                GroupHeader(stringResource(R.string.manual_noise_canceling))
                ManualNoiseCancelingSelection(
                    manualNoiseCanceling = soundModes.manualNoiseCanceling,
                    onManualNoiseCancelingChange = onManualNoiseCancelingChange,
                )
            }
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
        SoundModeTypeTwoSettings(
            soundModes = com.oppzippy.openscq30.lib.bindings.SoundModesTypeTwo(
                ambientSoundMode = AmbientSoundMode.Normal,
                transparencyMode = TransparencyMode.VocalMode,
                noiseCancelingMode = NoiseCancelingModeTypeTwo.Manual,
                manualNoiseCanceling = ManualNoiseCanceling.Moderate,
                adaptiveNoiseCanceling = AdaptiveNoiseCanceling.HighNoise,
                windNoiseSuppression = false,
                noiseCancelingAdaptiveSensitivityLevel = 0u,
            ),
            ambientSoundModeCycle = AmbientSoundModeCycle(
                normalMode = true,
                transparencyMode = false,
                noiseCancelingMode = true,
            ),
        )
    }
}
