package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.ui.equalizer.storage.CustomProfile

@Composable
fun CustomProfileSelection(
    selectedProfile: CustomProfile?,
    profiles: List<CustomProfile>,
    onProfileSelected: (selectedProfile: CustomProfile) -> Unit,
) {
    Dropdown(
        value = selectedProfile,
        values = profiles.map {
            Pair(it, it.name)
        },
        label = stringResource(R.string.custom_profile),
        onItemSelected = onProfileSelected,
    )
}