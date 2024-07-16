package com.oppzippy.openscq30.ui.importexport

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel

@Composable
fun ImportCustomProfiles(
    state: ImportCustomProfilesState,
    setState: (ImportCustomProfilesState?) -> Unit,
    importProfiles: (profiles: List<CustomProfile>, overwrite: Boolean) -> Unit,
) {
    when (state) {
        is ImportCustomProfilesState.StringInput -> {
            Column {
                TextField(
                    modifier = Modifier.testTag("json-input"),
                    label = { Text(stringResource(R.string.import_json)) },
                    value = state.profileString,
                    onValueChange = { setState(state.copy(profileString = it)) },
                    singleLine = true,
                )
                if (state.exception != null) {
                    Text(
                        text = state.exception.localizedMessage ?: state.exception.message
                            ?: "Unknown error",
                        color = MaterialTheme.colorScheme.error,
                    )
                }
                Button(onClick = { setState(state.next()) }) {
                    Text(stringResource(R.string.next))
                }
            }
        }

        is ImportCustomProfilesState.ImportOptions -> {
            LazyColumn(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                item {
                    CheckboxWithLabel(
                        text = stringResource(R.string.overwrite),
                        isChecked = state.overwrite,
                        onCheckedChange = {
                            setState(state.copy(overwrite = !state.overwrite))
                        },
                    )
                }
                itemsIndexed(state.profiles) { index, profile ->
                    Card {
                        Column(Modifier.padding(10.dp)) {
                            Text(
                                text = profile.name,
                                fontSize = MaterialTheme.typography.titleSmall.fontSize,
                                fontFamily = MaterialTheme.typography.titleSmall.fontFamily,
                                fontWeight = MaterialTheme.typography.titleSmall.fontWeight,
                                color = MaterialTheme.typography.titleSmall.color,
                            )
                            CheckboxWithLabel(
                                text = stringResource(R.string.import_),
                                isChecked = state.selection[index],
                                onCheckedChange = {
                                    setState(
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
                                CheckboxWithLabel(
                                    text = stringResource(R.string.rename),
                                    isChecked = state.rename[index] != null,
                                    onCheckedChange = {
                                        setState(
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
                                            setState(
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
                item {
                    Button(onClick = {
                        importProfiles(state.getFilteredProfiles(), state.overwrite)
                        setState(null)
                    }) {
                        Text(stringResource(R.string.next))
                    }
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
            setState = {},
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
            setState = {},
            importProfiles = { _, _ -> },
        )
    }
}
