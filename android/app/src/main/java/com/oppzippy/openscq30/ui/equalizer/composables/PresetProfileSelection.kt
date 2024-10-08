package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.equalizer.models.EqualizerProfile
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.Dropdown
import com.oppzippy.openscq30.ui.utils.DropdownOption

@Composable
fun PresetProfileSelection(value: EqualizerProfile, onProfileSelected: (profile: EqualizerProfile) -> Unit) {
    Dropdown(
        value = value,
        options = EqualizerProfile.entries.map { profile ->
            // Throws for the "custom" preset profile option, since that requires VolumeAdjustments
            // to be passed to toEqualizerConfiguration.
            val values = try {
                profile.toEqualizerConfiguration(null).volumeAdjustments
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
private fun PreviewPresetProfileSelection() {
    OpenSCQ30Theme {
        var profile by remember { mutableStateOf(EqualizerProfile.Classical) }
        PresetProfileSelection(value = profile, onProfileSelected = {
            profile = it
        })
    }
}
