package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.equalizer.storage.CustomProfile

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
