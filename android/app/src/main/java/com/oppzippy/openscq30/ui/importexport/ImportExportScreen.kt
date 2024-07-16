package com.oppzippy.openscq30.ui.importexport

import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.res.stringResource
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.R

@Composable
fun ImportExportScreen(viewModel: ImportExportViewModel = hiltViewModel()) {
    val customProfiles by viewModel.customProfiles.collectAsState(emptyList())

    when (val state = viewModel.importExportState.collectAsState().value) {
        null -> ActionSelection(
            customProfiles,
            setState = { viewModel.importExportState.value = it },
        )

        is ImportExportState.ExportCustomProfiles -> ExportCustomProfiles(
            state = state.state,
            setState = {
                viewModel.importExportState.value =
                    it?.let { ImportExportState.ExportCustomProfiles(it) }
            },
            setClipboard = { viewModel.copyToClipboard(it) },
        )

        is ImportExportState.ImportCustomProfiles -> ImportCustomProfiles(
            state = state.state,
            setState = {
                viewModel.importExportState.value =
                    it?.let { ImportExportState.ImportCustomProfiles(it) }
            },
            importProfiles = { profiles, overwrite ->
                viewModel.importCustomProfiles(
                    profiles,
                    overwrite,
                )
            },
        )
    }
}

@Composable
private fun ActionSelection(
    customProfiles: List<CustomProfile>,
    setState: (ImportExportState) -> Unit,
) {
    Column {
        Text(stringResource(R.string.import_export))
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
