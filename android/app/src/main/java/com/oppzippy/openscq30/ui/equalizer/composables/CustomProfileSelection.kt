package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.Dropdown
import com.oppzippy.openscq30.ui.utils.DropdownOption

@Composable
fun CustomProfileSelection(
    selectedProfile: CustomProfile?,
    profiles: List<CustomProfile>,
    onProfileSelected: (selectedProfile: CustomProfile) -> Unit,
    modifier: Modifier = Modifier,
) {
    Dropdown(
        value = selectedProfile,
        options = profiles.map { profile ->
            DropdownOption(value = profile, name = profile.name, label = {
                ProfileSelectionRow(
                    name = profile.name,
                    volumeAdjustments = profile.getVolumeAdjustments().adjustments().toList(),
                )
            })
        },
        label = stringResource(R.string.custom_profile),
        onItemSelected = onProfileSelected,
        modifier = modifier,
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewCustomProfileSelection() {
    val selectedProfile = CustomProfile("Test Profile", 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0)
    OpenSCQ30Theme {
        CustomProfileSelection(
            selectedProfile = selectedProfile,
            profiles = listOf(selectedProfile),
            onProfileSelected = {},
        )
    }
}
