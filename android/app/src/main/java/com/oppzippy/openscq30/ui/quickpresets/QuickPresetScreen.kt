package com.oppzippy.openscq30.ui.quickpresets

import android.Manifest
import android.os.Build
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.quickpresets.composables.QuickPresetConfiguration
import com.oppzippy.openscq30.ui.quickpresets.composables.QuickPresetSelection
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.Loading
import com.oppzippy.openscq30.ui.utils.PermissionCheck

@Composable
fun QuickPresetScreen(viewModel: QuickPresetViewModel = hiltViewModel()) {
    val preset = viewModel.quickPreset.collectAsState().value
    val allEqualizerProfileNames by viewModel.equalizerProfileNames.collectAsState()

    // We can't nest the content inside the permission check since we need to ensure the permission
    // check doesn't run on versions of android that don't require permission for foreground service
    // notifications.
    val isTiramisuOrNewer = Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU
    var permissionCheckPassed by remember { mutableStateOf(!isTiramisuOrNewer) }
    // Redundant check to fix lint error since permissionCheckPassed will always be true if
    // isTiramisuOrNewer is false.
    if (!permissionCheckPassed && isTiramisuOrNewer) {
        PermissionCheck(
            permission = Manifest.permission.POST_NOTIFICATIONS,
            prompt = stringResource(R.string.notification_permission_is_required),
        ) {
            permissionCheckPassed = true
        }
    }

    if (permissionCheckPassed) {
        if (preset != null) {
            QuickPresetScreen(
                preset = preset,
                allEqualizerProfileNames = allEqualizerProfileNames,
                onSelectedIndexChange = { viewModel.selectQuickPreset(it) },
                onAmbientSoundModeChange = {
                    viewModel.upsertQuickPreset(
                        preset.copy(ambientSoundMode = it),
                    )
                },
                onNoiseCancelingModeChange = {
                    viewModel.upsertQuickPreset(
                        preset.copy(noiseCancelingMode = it),
                    )
                },
                onEqualizerProfileNameChange = {
                    viewModel.upsertQuickPreset(
                        preset.copy(equalizerProfileName = it),
                    )
                },
                onNameChange = { viewModel.upsertQuickPreset(preset.copy(name = it)) },
            )
        } else {
            Loading()
        }
    }
}

@Composable
private fun QuickPresetScreen(
    preset: QuickPreset,
    allEqualizerProfileNames: List<String>,
    onSelectedIndexChange: (index: Int) -> Unit = {},
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode?) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode?) -> Unit = {},
    onEqualizerProfileNameChange: (name: String?) -> Unit = {},
    onNameChange: (name: String?) -> Unit = {},
) {
    Column {
        QuickPresetSelection(
            selectedIndex = preset.id,
            onSelectedIndexChange = onSelectedIndexChange,
        )
        QuickPresetConfiguration(
            name = preset.name,
            defaultName = stringResource(R.string.quick_preset_number, preset.id + 1),
            ambientSoundMode = preset.ambientSoundMode,
            noiseCancelingMode = preset.noiseCancelingMode,
            equalizerProfileName = preset.equalizerProfileName,
            allEqualizerProfileNames = allEqualizerProfileNames,
            onAmbientSoundModeChange = onAmbientSoundModeChange,
            onNoiseCancelingModeChange = onNoiseCancelingModeChange,
            onEqualizerProfileNameChange = onEqualizerProfileNameChange,
            onNameChange = onNameChange,
        )
    }
}

@Preview(showBackground = true)
@Composable
fun PreviewQuickPresetScreenWithAllOptionsChecked() {
    OpenSCQ30Theme {
        QuickPresetScreen(
            preset = QuickPreset(
                id = 0,
                ambientSoundMode = AmbientSoundMode.Normal,
                noiseCancelingMode = NoiseCancelingMode.Transport,
                equalizerProfileName = "Test EQ Profile",
            ),
            allEqualizerProfileNames = emptyList(),
        )
    }
}

@Preview(showBackground = true)
@Composable
fun PreviewQuickPresetScreenWithNoOptionsChecked() {
    OpenSCQ30Theme {
        QuickPresetScreen(
            preset = QuickPreset(0),
            allEqualizerProfileNames = emptyList(),
        )
    }
}
