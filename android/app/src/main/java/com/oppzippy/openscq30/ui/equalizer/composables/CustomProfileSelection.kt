package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.Dropdown
import com.oppzippy.openscq30.ui.utils.DropdownOption

@Composable
fun CustomProfileSelection(
    selectedProfile: CustomProfile?,
    profiles: List<CustomProfile>,
    onProfileSelected: (selectedProfile: CustomProfile) -> Unit,
) {
    Dropdown(
        value = selectedProfile,
        options = profiles.map { profile ->
            DropdownOption(value = profile, name = profile.name, label = {
                ProfileSelectionRow(name = profile.name, volumeAdjustments = profile.values)
            })
        },
        label = stringResource(R.string.custom_profile),
        onItemSelected = onProfileSelected,
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewCustomProfileSelection() {
    val selectedProfile = CustomProfile("Test Profile", listOf(0, 10, 20, 30, 40, 50, 60, 70))
    OpenSCQ30Theme {
        CustomProfileSelection(
            selectedProfile = selectedProfile,
            profiles = listOf(selectedProfile),
            onProfileSelected = {},
        )
    }
}
