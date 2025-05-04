package com.oppzippy.openscq30.ui.importexport

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.LabeledSwitch

@Composable
fun ImportCustomProfiles(
    state: ImportCustomProfilesState,
    onSetState: (ImportCustomProfilesState) -> Unit,
    onPushState: (ImportCustomProfilesState) -> Unit,
    onDone: () -> Unit,
    importProfiles: (profiles: List<CustomProfile>, overwrite: Boolean) -> Unit,
) {
    when (state) {
        is ImportCustomProfilesState.StringInput -> {
            Column(
                Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState()),
            ) {
                TextField(
                    modifier = Modifier
                        .testTag("json-input")
                        .fillMaxWidth(),
                    label = { Text(stringResource(R.string.import_json)) },
                    value = state.profileString,
                    onValueChange = { onSetState(state.copy(profileString = it)) },
                    singleLine = true,
                )
                Button(
                    modifier = Modifier.fillMaxWidth(),
                    onClick = {
                        val nextState = state.next()
                        if (nextState is ImportCustomProfilesState.StringInput) {
                            onSetState(nextState)
                        } else {
                            onPushState(nextState)
                        }
                    },
                ) {
                    Text(stringResource(R.string.next))
                }
                if (state.exception != null) {
                    Text(
                        text = state.exception.localizedMessage ?: state.exception.message
                            ?: stringResource(R.string.unknown_error),
                        color = MaterialTheme.colorScheme.error,
                    )
                }
            }
        }

        is ImportCustomProfilesState.ImportOptions -> {
            Column {
                LazyColumn(
                    modifier = Modifier.weight(1F),
                    verticalArrangement = Arrangement.spacedBy(10.dp),
                ) {
                    if (state.profiles.isEmpty()) {
                        item {
                            Text(stringResource(R.string.no_profiles_found))
                        }
                    }
                    itemsIndexed(state.profiles) { index, profile ->
                        Card(Modifier.fillMaxWidth()) {
                            Column(Modifier.padding(10.dp)) {
                                Text(
                                    text = profile.name,
                                    fontSize = MaterialTheme.typography.titleSmall.fontSize,
                                    fontFamily = MaterialTheme.typography.titleSmall.fontFamily,
                                    fontWeight = MaterialTheme.typography.titleSmall.fontWeight,
                                    color = MaterialTheme.typography.titleSmall.color,
                                )
                                LabeledSwitch(
                                    label = stringResource(R.string.import_),
                                    isChecked = state.selection[index],
                                    onCheckedChange = {
                                        onSetState(
                                            state.copy(
                                                selection = state.selection.let {
                                                    val list = it.toMutableList()
                                                    list[index] = !list[index]
                                                    list
                                                },
                                            ),
                                        )
                                    },
                                )
                                if (state.selection[index]) {
                                    LabeledSwitch(
                                        label = stringResource(R.string.rename),
                                        isChecked = state.rename[index] != null,
                                        onCheckedChange = {
                                            onSetState(
                                                state.copy(
                                                    rename = state.rename.let {
                                                        val list = it.toMutableList()
                                                        list[index] = if (list[index] == null) {
                                                            profile.name
                                                        } else {
                                                            null
                                                        }
                                                        list
                                                    },
                                                ),
                                            )
                                        },
                                    )
                                    state.rename[index]?.let { renameTo ->
                                        TextField(
                                            label = { Text(stringResource(R.string.new_name)) },
                                            value = renameTo,
                                            onValueChange = { newRenameTo ->
                                                onSetState(
                                                    state.copy(
                                                        rename = state.rename.let {
                                                            val list = it.toMutableList()
                                                            list[index] = newRenameTo
                                                            list
                                                        },
                                                    ),
                                                )
                                            },
                                            singleLine = true,
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
                Row(
                    horizontalArrangement = Arrangement.spacedBy(4.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    LabeledSwitch(
                        modifier = Modifier.weight(1F),
                        label = stringResource(R.string.overwrite),
                        isChecked = state.overwrite,
                        onCheckedChange = {
                            onSetState(state.copy(overwrite = !state.overwrite))
                        },
                    )
                    Button(
                        modifier = Modifier.weight(1F),
                        onClick = {
                            importProfiles(state.getFilteredProfiles(), state.overwrite)
                            onPushState(ImportCustomProfilesState.ImportComplete)
                        },
                    ) {
                        Text(stringResource(R.string.next))
                    }
                }
            }
        }

        ImportCustomProfilesState.ImportComplete -> {
            Column(
                modifier = Modifier.fillMaxSize(),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.Center,
            ) {
                Text(
                    text = stringResource(R.string.import_complete),
                    style = MaterialTheme.typography.headlineSmall,
                )
                Button(onClick = { onDone() }) {
                    Text(stringResource(R.string.done))
                }
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewStringInput() {
    OpenSCQ30Theme {
        ImportCustomProfiles(
            state = ImportCustomProfilesState.StringInput(profileString = "import string here"),
            onPushState = {},
            onSetState = {},
            onDone = {},
            importProfiles = { _, _ -> },
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewImportOptions() {
    OpenSCQ30Theme {
        ImportCustomProfiles(
            state = ImportCustomProfilesState.ImportOptions(
                profiles = listOf(
                    CustomProfile("Profile 1", emptyList()),
                    CustomProfile("Profile Two", emptyList()),
                ),
                selection = listOf(true, false),
                rename = listOf("Renamed Profile 1", null),
            ),
            onPushState = {},
            onSetState = {},
            onDone = {},
            importProfiles = { _, _ -> },
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewImportComplete() {
    OpenSCQ30Theme {
        ImportCustomProfiles(
            state = ImportCustomProfilesState.ImportComplete,
            onPushState = {},
            onSetState = {},
            onDone = {},
            importProfiles = { _, _ -> },
        )
    }
}
