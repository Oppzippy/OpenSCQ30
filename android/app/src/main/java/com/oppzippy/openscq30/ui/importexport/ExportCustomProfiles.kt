package com.oppzippy.openscq30.ui.importexport

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel

@Composable
fun ExportCustomProfiles(
    state: ExportCustomProfilesState,
    onSetState: (ExportCustomProfilesState) -> Unit,
    onPushState: (ExportCustomProfilesState) -> Unit,
    onDone: () -> Unit,
    setClipboard: (String) -> Unit,
) {
    when (state) {
        is ExportCustomProfilesState.ProfileSelection -> {
            LazyColumn {
                itemsIndexed(state.customProfiles) { index, profile ->
                    CheckboxWithLabel(
                        text = profile.name,
                        isChecked = state.selectedProfiles[index],
                        onCheckedChange = {
                            onSetState(
                                state.copy(
                                    selectedProfiles = state.selectedProfiles.let {
                                        val list = it.toMutableList()
                                        list[index] = !list[index]
                                        list
                                    },
                                ),
                            )
                        },
                    )
                }
                item {
                    Button(onClick = { onPushState(state.next()) }) {
                        Text(stringResource(R.string.next))
                    }
                }
            }
        }

        is ExportCustomProfilesState.CopyToClipboard -> {
            Column {
                Row {
                    Button(onClick = { setClipboard(state.profileString) }) {
                        Text(stringResource(R.string.copy_to_clipboard))
                    }
                    Button(onClick = { onDone() }) {
                        Text(stringResource(R.string.done))
                    }
                }
                TextField(
                    modifier = Modifier.testTag("json-output"),
                    value = state.profileString,
                    onValueChange = {},
                    readOnly = true,
                )
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewProfileSelection() {
    OpenSCQ30Theme {
        ExportCustomProfiles(
            state = ExportCustomProfilesState.ProfileSelection(
                customProfiles = listOf(
                    CustomProfile("Profile 1", emptyList()),
                    CustomProfile("Profile Two", emptyList()),
                ),
                selectedProfiles = listOf(true, false),
            ),
            onSetState = {},
            onPushState = {},
            onDone = {},
            setClipboard = {},
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewCopyToClipboard() {
    OpenSCQ30Theme {
        ExportCustomProfiles(
            state = ExportCustomProfilesState.CopyToClipboard("JSON here"),
            onSetState = {},
            onPushState = {},
            onDone = {},
            setClipboard = {},
        )
    }
}
