package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.ui.devicesettings.models.EqualizerProfile
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun EqualizerSettings(
    profile: EqualizerProfile,
    equalizerValues: List<Byte>,
    onProfileChange: (profile: EqualizerProfile) -> Unit,
    onEqualizerValueChange: (index: Int, changedValue: Byte) -> Unit,
) {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        PresetProfileSelection(value = profile, onProfileSelected = onProfileChange)
        Equalizer(equalizerValues, onEqualizerValueChange)
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        var profile by remember { mutableStateOf(EqualizerProfile.Acoustic) }
        var equalizerValues by remember { mutableStateOf(listOf<Byte>(0, 0, 0, 0, 0, 0, 0, 0)) }
        EqualizerSettings(profile = profile,
            equalizerValues = equalizerValues,
            onProfileChange = { profile = it },
            onEqualizerValueChange = { changedIndex, changedValue ->
                equalizerValues = equalizerValues.mapIndexed { index, value ->
                    if (changedIndex == index) {
                        changedValue
                    } else {
                        value
                    }
                }
            })
    }
}
