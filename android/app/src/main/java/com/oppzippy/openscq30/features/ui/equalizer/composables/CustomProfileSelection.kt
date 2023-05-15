package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
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
        options = profiles.map { profile ->
            DropdownOption(value = profile, name = profile.name, label = {
                ProfileSelectionRow(name = profile.name, volumeAdjustments = profile.values)
            })
        },
        label = stringResource(R.string.custom_profile),
        onItemSelected = onProfileSelected,
    )
}
