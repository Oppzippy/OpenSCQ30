package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.features.ui.equalizer.models.EqualizerProfile
import com.oppzippy.openscq30.features.ui.equalizer.EqualizerSettingsViewModel
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.R

@Composable
fun EqualizerSettings(
    viewModel: EqualizerSettingsViewModel = hiltViewModel()
) {
    val maybeEqualizerConfiguration by viewModel.displayedEqualizerConfiguration.collectAsState()

    maybeEqualizerConfiguration?.let { equalizerConfiguration ->
        val profile = equalizerConfiguration.equalizerProfile
        val values = equalizerConfiguration.values
        val isCustomProfile = profile == EqualizerProfile.Custom
        var isCreateDialogOpen by remember { mutableStateOf(false) }
        var isDeleteDialogOpen by remember { mutableStateOf(false) }
        val selectedCustomProfile by viewModel.selectedCustomProfile.collectAsState()
        val customProfiles by viewModel.customProfiles.collectAsState()

        CreateCustomProfileDialog(
            isOpen = isCreateDialogOpen,
            onDismiss = { isCreateDialogOpen = false },
            onCreateCustomProfile = { viewModel.createCustomProfile(it) },
        )
        selectedCustomProfile?.let { profile ->
            DeleteCustomProfileDialog(
                isOpen = isDeleteDialogOpen,
                profileName = profile.name,
                onDismiss = { isDeleteDialogOpen = false },
                onDelete = { viewModel.deleteCustomProfile(profile.name) },
            )
        }

        Column {
            PresetProfileSelection(value = profile, onProfileSelected = { newProfile ->
                viewModel.setEqualizerConfiguration(newProfile, values.toByteArray())
            })
            if (isCustomProfile) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    CustomProfileSelection(
                        selectedProfile = selectedCustomProfile,
                        profiles = customProfiles,
                        onProfileSelected = {
                            viewModel.selectCustomProfile(it)
                        },
                    )
                    IconButton(onClick = { isCreateDialogOpen = true }) {
                        Icon(
                            imageVector = Icons.Filled.AddCircle,
                            contentDescription = stringResource(R.string.add),
                        )
                    }
                    IconButton(onClick = { isDeleteDialogOpen = true }) {
                        Icon(
                            imageVector = Icons.Filled.Delete,
                            contentDescription = stringResource(R.string.add),
                        )
                    }
                }
            }
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
