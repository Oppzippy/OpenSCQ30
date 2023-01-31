package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.features.ui.equalizer.EqualizerProfile
import com.oppzippy.openscq30.features.ui.equalizer.EqualizerSettingsViewModel
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun EqualizerSettings(
    viewModel: EqualizerSettingsViewModel = hiltViewModel()
) {
    val maybeEqualizerConfiguration by viewModel.displayedEqualizerConfiguration.collectAsState()

    maybeEqualizerConfiguration?.let { equalizerConfiguration ->
        val profile = equalizerConfiguration.equalizerProfile
        val values = equalizerConfiguration.values
        val isCustomProfile = profile == EqualizerProfile.Custom

        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            PresetProfileSelection(value = profile, onProfileSelected = { newProfile ->
                viewModel.setEqualizerConfiguration(newProfile, values.toByteArray())
            })
            Equalizer(
                values = values,
                enabled = isCustomProfile,
                onValueChange = { changedIndex, changedValue ->
                    viewModel.setEqualizerConfiguration(
                        profile,
                        values.mapIndexed { index, value ->
                            if (index == changedIndex) {
                                changedValue
                            } else {
                                value
                            }
                        }.toByteArray(),
                    )
                },
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        EqualizerSettings()
    }
}
