@file:OptIn(ExperimentalMaterial3Api::class)

package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.wrapper.QuickPreset
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledSwitch

@Composable
fun QuickPresetsPage(
    allSettings: List<String>,
    quickPresets: List<QuickPreset>,
    onActivate: (String) -> Unit,
    onCreate: (String) -> Unit,
    onToggleSetting: (presetName: String, settingId: String, isEnabled: Boolean) -> Unit,
) {
    var editing by remember { mutableStateOf<QuickPreset?>(null) }
    editing.let { quickPreset ->
        if (quickPreset == null) {
            QuickPresetsList(quickPresets, onCreate, onActivate, onEdit = { editing = it })
        } else {
            QuickPresetEdit(allSettings, quickPreset, onToggleSetting)
        }
    }
}

@Composable
private fun QuickPresetsList(
    quickPresets: List<QuickPreset>,
    onCreate: (String) -> Unit,
    onActivate: (String) -> Unit,
    onEdit: (QuickPreset) -> Unit,
) {
    CreateQuickPresetButton(onCreate = onCreate)
    if (quickPresets.isNotEmpty()) {
        LazyColumn {
            items(quickPresets) { preset ->
                Row {
                    Text(preset.name)
                    Row {
                        Button(
                            onClick = { onActivate(preset.name) },
                            enabled = preset.isActive,
                        ) {
                            Text(
                                if (preset.isActive) {
                                    stringResource(
                                        R.string.active,
                                    )
                                } else {
                                    stringResource(R.string.activate)
                                },
                            )
                        }
                        Button(onClick = { onEdit(preset) }) { Text(stringResource(R.string.edit)) }
                    }
                }
            }
        }
    } else {
        Text(stringResource(R.string.no_quick_presets))
    }
}

@Composable
private fun CreateQuickPresetButton(onCreate: (String) -> Unit) {
    var isDialogShown by remember { mutableStateOf(false) }
    if (isDialogShown) {
        var name by remember { mutableStateOf("") }
        AlertDialog(
            onDismissRequest = { isDialogShown = false },
            confirmButton = {
                Button(onClick = { onCreate(name) }) {
                    Text(stringResource(R.string.create))
                }
            },
            dismissButton = {
                TextButton(onClick = { isDialogShown = false }) {
                    Text(stringResource(R.string.cancel))
                }
            },
            text = {
                TextField(
                    value = name,
                    onValueChange = { name = it },
                    label = { Text(stringResource(R.string.name)) },
                )
            },
        )
    }

    FloatingActionButton(onClick = { isDialogShown = true }) {
        Icon(
            imageVector = Icons.Filled.Add,
            contentDescription = stringResource(R.string.create),
        )
    }
}

@Composable
private fun QuickPresetEdit(
    allSettings: List<String>,
    quickPreset: QuickPreset,
    onToggleSetting: (name: String, setting: String, isEnabled: Boolean) -> Unit,
) {
    val fields = quickPreset.settings.associate { Pair(it.settingId, it.value) }
    LazyColumn {
        items(allSettings) { settingId ->
            Column {
                val isEnabled = fields.containsKey(settingId)
                LabeledSwitch(
                    label = translateSettingId(settingId),
                    isChecked = isEnabled,
                    onCheckedChange = { onToggleSetting(quickPreset.name, settingId, it) },
                )
                if (isEnabled) {
                    Text(fields[settingId].toString())
                }
            }
        }
    }
}

@Preview
@Composable
private fun PreviewQuickPresetsList() {
    OpenSCQ30Theme {
        QuickPresetsList(
            quickPresets = listOf(
                QuickPreset(name = "Preset 1", isActive = true, settings = emptyList()),
                QuickPreset(name = "Preset 2", isActive = false, settings = emptyList()),
                QuickPreset(name = "Preset 3", isActive = false, settings = emptyList()),
            ),
            onEdit = {},
            onCreate = {},
            onActivate = {},
        )
    }
}
