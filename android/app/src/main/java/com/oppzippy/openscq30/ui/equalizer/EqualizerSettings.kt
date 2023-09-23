package com.oppzippy.openscq30.ui.equalizer

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.FindReplace
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.equalizer.composables.CreateCustomProfileDialog
import com.oppzippy.openscq30.ui.equalizer.composables.CustomProfileSelection
import com.oppzippy.openscq30.ui.equalizer.composables.DeleteCustomProfileDialog
import com.oppzippy.openscq30.ui.equalizer.composables.Equalizer
import com.oppzippy.openscq30.ui.equalizer.composables.PresetProfileSelection
import com.oppzippy.openscq30.ui.equalizer.composables.ReplaceCustomProfileDialog
import com.oppzippy.openscq30.ui.equalizer.models.EqualizerProfile
import com.oppzippy.openscq30.ui.equalizer.models.toEqualizerProfile
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import java.util.UUID
import kotlin.jvm.optionals.getOrNull

@Composable
fun EqualizerSettings(
    uiState: UiDeviceState.Connected,
    onEqualizerConfigurationChange: (equalizerConfiguration: EqualizerConfiguration) -> Unit = {},
    viewModel: EqualizerSettingsViewModel = hiltViewModel(),
) {
    viewModel.pushUiState(uiState)
    viewModel.setRealEqualizerConfiguration = onEqualizerConfigurationChange

    val maybeEqualizerConfiguration by viewModel.displayedEqualizerConfiguration.collectAsState()

    maybeEqualizerConfiguration?.let { equalizerConfiguration ->
        val profile = equalizerConfiguration.presetProfile().getOrNull().toEqualizerProfile()
        val values = equalizerConfiguration.volumeAdjustments().adjustments().toList()
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
            existingProfiles = customProfiles,
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
                if (isCustomProfile) {
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
        EqualizerSettings(
            uiState = UiDeviceState.Connected(
                "Test Device",
                "00:00:00:00:00:00",
                SoundcoreDeviceState(
                    featureFlags = -1, // TODO
                    equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
                    soundModes = null,
                    serialNumber = null,
                    leftFirmwareVersion = null,
                    rightFirmwareVersion = null,
                    leftBatteryLevel = 0,
                    rightBatteryLevel = 0,
                    isLeftBatteryCharging = false,
                    isRightBatteryCharging = false,
                    ageRange = null,
                    dynamicRangeCompressionMinFirmwareVersion = null,
                    customHearId = null,
                    gender = null,
                ),
                UUID(0, 0),
            ),
        )
    }
}
