package com.oppzippy.openscq30.ui.devicesettings.composables.equalizer

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.ui.devicesettings.models.EqualizerProfile
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import kotlin.jvm.optionals.getOrNull

@Composable
fun EqualizerSettings(
    viewModel: EqualizerSettingsViewModel = hiltViewModel()
) {
    val equalizerConfiguration by viewModel.displayedEqualizerConfiguration.collectAsState()

    equalizerConfiguration?.let { equalizerConfiguration ->
        val profile = equalizerConfiguration.equalizerProfile
        val values = equalizerConfiguration.values

        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            PresetProfileSelection(value = profile, onProfileSelected = { newProfile ->
                viewModel.setEqualizerConfiguration(newProfile, values.toByteArray())
            })
            Equalizer(values = values, onValueChange = { changedIndex, changedValue ->
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
            })
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
