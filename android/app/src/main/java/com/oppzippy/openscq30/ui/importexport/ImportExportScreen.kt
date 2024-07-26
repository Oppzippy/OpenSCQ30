package com.oppzippy.openscq30.ui.importexport

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.R

@Composable
fun ImportExportScreen(
    viewModel: ImportExportViewModel = hiltViewModel(),
    index: Int,
    onIndexChange: (Int) -> Unit,
) {
    val customProfiles by viewModel.customProfiles.collectAsState(emptyList())

    fun pushState(state: ImportExportState, index: Int) {
        onIndexChange(viewModel.pushState(state, index))
    }

    fun setState(state: ImportExportState, index: Int) {
        onIndexChange(viewModel.setState(state, index))
    }

    fun resetState() {
        onIndexChange(viewModel.resetState())
    }

    Box(
        Modifier
            .fillMaxSize()
            .padding(5.dp),
    ) {
        when (val state = viewModel.stateStack.collectAsState().value.getOrNull(index)) {
            null -> ActionSelection(
                customProfiles,
                setState = { setState(it, index) },
            )

            is ImportExportState.ExportCustomProfiles -> ExportCustomProfiles(
                state = state.state,
                onPushState = { pushState(ImportExportState.ExportCustomProfiles(it), index) },
                onSetState = { setState(ImportExportState.ExportCustomProfiles(it), index) },
                onDone = { resetState() },
                setClipboard = { viewModel.copyToClipboard(it) },
            )

            is ImportExportState.ImportCustomProfiles -> ImportCustomProfiles(
                state = state.state,
                onPushState = { pushState(ImportExportState.ImportCustomProfiles(it), index) },
                onSetState = { setState(ImportExportState.ImportCustomProfiles(it), index) },
                onDone = { resetState() },
                importProfiles = { profiles, overwrite ->
                    viewModel.importCustomProfiles(
                        profiles,
                        overwrite,
                    )
                },
            )
        }
    }
}

@Composable
private fun ActionSelection(
    customProfiles: List<CustomProfile>,
    setState: (ImportExportState) -> Unit,
) {
    Column(
        modifier = Modifier.fillMaxSize(),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center,
    ) {
        Button(onClick = {
            setState(ImportExportState.ImportCustomProfiles(ImportCustomProfilesState.StringInput()))
        }) {
            Text(stringResource(R.string.import_custom_profiles))
        }
        Button(onClick = {
            setState(
                ImportExportState.ExportCustomProfiles(
                    ExportCustomProfilesState.ProfileSelection(
                        customProfiles = customProfiles,
                        selectedProfiles = List(customProfiles.size) { false },
                    ),
                ),
            )
        }) {
            Text(stringResource(R.string.export_custom_profiles))
        }
    }
}
