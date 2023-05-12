package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.FindReplace
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
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
        var isReplaceDialogOpen by remember { mutableStateOf(false) }
        val selectedCustomProfile by viewModel.selectedCustomProfile.collectAsState()
        val customProfiles by viewModel.customProfiles.collectAsState()
        val valueTexts by viewModel.valueTexts.collectAsState()

        CreateCustomProfileDialog(
            isOpen = isCreateDialogOpen,
            onDismiss = { isCreateDialogOpen = false },
            onCreateCustomProfile = { viewModel.createCustomProfile(it) },
        )
        selectedCustomProfile?.let { customProfile ->
            DeleteCustomProfileDialog(
                isOpen = isDeleteDialogOpen,
                profileName = customProfile.name,
                onDismiss = { isDeleteDialogOpen = false },
                onDelete = { viewModel.deleteCustomProfile(customProfile.name) },
            )
        }
        ReplaceCustomProfileDialog(
            isOpen = isReplaceDialogOpen,
            profiles = customProfiles,
            onProfileSelected = {
                viewModel.createCustomProfile(it.name)
            },
            onDismiss = { isReplaceDialogOpen = false },
        )

        Column {
            PresetProfileSelection(value = profile, onProfileSelected = { newProfile ->
                viewModel.selectPresetProfile(newProfile)
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
                    if (selectedCustomProfile == null) {
                        IconButton(onClick = { isCreateDialogOpen = true }) {
                            Icon(
                                imageVector = Icons.Filled.AddCircle,
                                contentDescription = stringResource(R.string.add),
                            )
                        }
                    } else {
                        IconButton(onClick = { isDeleteDialogOpen = true }) {
                            Icon(
                                imageVector = Icons.Filled.Delete,
                                contentDescription = stringResource(R.string.delete),
                            )
                        }
                    }
                    if (customProfiles.isNotEmpty()) {
                        IconButton(onClick = { isReplaceDialogOpen = true }) {
                            Icon(
                                imageVector = Icons.Filled.FindReplace,
                                contentDescription = stringResource(R.string.replace_existing_profile),
                            )
                        }
                    }
                }
            }
            Equalizer(
                values = values,
                enabled = isCustomProfile,
                onValueChange = { changedIndex, changedValue ->
                    viewModel.onValueChange(changedIndex, changedValue)
                },
                texts = valueTexts,
                onTextChanged = { index, value ->
                    viewModel.onValueTextChange(index, value)
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
