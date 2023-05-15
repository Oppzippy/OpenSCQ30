package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.ui.equalizer.models.EqualizerProfile
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun PresetProfileSelection(
    value: EqualizerProfile, onProfileSelected: (profile: EqualizerProfile) -> Unit
) {
    val profiles = EqualizerProfile.values()

    Dropdown(
        value = value,
        options = profiles.asList().map { profile ->
            // Throws for the "custom" preset profile option, since that requires VolumeAdjustments
            // to be passed to toEqualizerConfiguration.
            val values = try {
                profile.toEqualizerConfiguration(null).volumeAdjustments().adjustments().toList()
            } catch (e: NullPointerException) {
                null
            }
            DropdownOption(
                value = profile,
                name = stringResource(profile.localizationStringId),
                label = {
                    ProfileSelectionRow(
                        name = stringResource(profile.localizationStringId),
                        volumeAdjustments = values,
                    )
                },
            )
        },
        label = stringResource(id = R.string.profile),
        onItemSelected = onProfileSelected,
    )
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        var profile by remember { mutableStateOf(EqualizerProfile.Classical) }
        PresetProfileSelection(value = profile, onProfileSelected = {
            profile = it
        })
    }
}
