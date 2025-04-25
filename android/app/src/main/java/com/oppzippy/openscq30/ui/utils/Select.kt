@file:OptIn(ExperimentalMaterial3Api::class)

package com.oppzippy.openscq30.ui.utils

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.AlertDialogDefaults
import androidx.compose.material3.BasicAlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ProvideTextStyle
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun Select(
    modifier: Modifier = Modifier,
    name: String,
    options: List<String>,
    selectedIndex: Int,
    onSelect: (Int) -> Unit,
) {
    var isPickerOpen by remember { mutableStateOf(false) }
    if (isPickerOpen) {
        BasicAlertDialog(onDismissRequest = { isPickerOpen = false }) {
            Surface(
                modifier = Modifier
                    .wrapContentWidth()
                    .wrapContentHeight(),
                shape = MaterialTheme.shapes.large,
                tonalElevation = AlertDialogDefaults.TonalElevation,
            ) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    ProvideTextStyle(MaterialTheme.typography.titleLarge) {
                        Text(
                            text = name,
                            modifier = Modifier.padding(16.dp),
                        )
                    }
                    LazyColumn(
                        modifier = Modifier.padding(
                            start = 16.dp,
                            end = 16.dp,
                            bottom = 16.dp,
                        ),
                    ) {
                        itemsIndexed(options) { index, value ->
                            TextButton(
                                modifier = Modifier.fillMaxWidth(),
                                onClick = {
                                    isPickerOpen = false
                                    onSelect(index)
                                },
                            ) {
                                Text(value)
                            }
                        }
                    }
                }
            }
        }
    }

    Button(modifier = modifier, onClick = { isPickerOpen = true }) {
        Text(options.getOrElse(selectedIndex) { stringResource(R.string.unknown) })
    }
}

@Composable
fun OptionalSelect(
    modifier: Modifier = Modifier,
    name: String,
    options: List<String>,
    selectedIndex: Int?,
    onSelect: (Int?) -> Unit,
) {
    Select(
        modifier,
        name = name,
        options = listOf(stringResource(R.string.none)).plus(options),
        selectedIndex = if (selectedIndex != null) selectedIndex + 1 else 0,
        onSelect = { if (it == 0) onSelect(null) else onSelect(it - 1) },
    )
}

private sealed class ModifiableSelectDialog {
    data class AddOption(val optionName: String) : ModifiableSelectDialog()
    data object RemoveOption : ModifiableSelectDialog()
}

@Composable
fun ModifiableSelect(
    modifier: Modifier = Modifier,
    name: String,
    options: List<String>,
    selectedIndex: Int?,
    onSelect: (Int?) -> Unit,
    onAddOption: (String) -> Unit,
    onRemoveOption: () -> Unit,
) {
    var dialogState by remember { mutableStateOf<ModifiableSelectDialog?>(null) }
    dialogState?.let { dialog ->
        when (dialog) {
            is ModifiableSelectDialog.AddOption -> {
                AlertDialog(
                    onDismissRequest = { dialogState = null },
                    title = { Text(name) },
                    text = {
                        TextField(
                            value = dialog.optionName,
                            label = { Text(stringResource(R.string.name)) },
                            onValueChange = { dialogState = dialog.copy(optionName = it) },
                            modifier = Modifier.fillMaxWidth(),
                            singleLine = true,
                        )
                    },
                    confirmButton = {
                        Button(
                            onClick = {
                                onAddOption(dialog.optionName)
                                dialogState = null
                            },
                        ) {
                            if (!options.contains(dialog.optionName)) {
                                Text(stringResource(R.string.create))
                            } else {
                                Text(stringResource(R.string.replace))
                            }
                        }
                    },
                    dismissButton = {
                        Button(onClick = { dialogState = null }) {
                            Text(stringResource(R.string.cancel))
                        }
                    },
                )
            }

            is ModifiableSelectDialog.RemoveOption -> {
                val selectedOptionName = if (selectedIndex != null) {
                    options.getOrElse(selectedIndex) { stringResource(R.string.unknown) }
                } else {
                    stringResource(R.string.none)
                }
                AlertDialog(
                    onDismissRequest = { dialogState = null },
                    title = { Text(name) },
                    text = { Text(stringResource(R.string.delete_confirm, selectedOptionName)) },
                    confirmButton = {
                        Button(
                            onClick = {
                                onRemoveOption()
                                dialogState = null
                            },
                        ) {
                            Text(stringResource(R.string.delete))
                        }
                    },
                    dismissButton = {
                        Button(onClick = { dialogState = null }) {
                            Text(stringResource(R.string.cancel))
                        }
                    },
                )
            }
        }
    }
    Row {
        Select(
            modifier,
            name = name,
            options = listOf(stringResource(R.string.none)).plus(options),
            selectedIndex = if (selectedIndex != null) selectedIndex + 1 else 0,
            onSelect = { if (it == 0) onSelect(null) else onSelect(it - 1) },
        )

        if (selectedIndex == null) {
            IconButton(onClick = { dialogState = ModifiableSelectDialog.AddOption("") }) {
                Icon(Icons.Filled.Add, contentDescription = stringResource(R.string.add))
            }
        } else {
            IconButton(onClick = { dialogState = ModifiableSelectDialog.RemoveOption }) {
                Icon(Icons.Filled.Remove, contentDescription = stringResource(R.string.delete))
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewSelect() {
    OpenSCQ30Theme {
        Select(
            name = "Number",
            selectedIndex = 0,
            options = listOf("One", "Two", "Three"),
            onSelect = {},
        )
    }
}
