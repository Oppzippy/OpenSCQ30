package com.oppzippy.openscq30.ui.settings

import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel

@Composable
fun Settings(viewModel: SettingsViewModel = hiltViewModel()) {
    val autoConnect by viewModel.autoConnect.collectAsState()
    Settings(
        autoConnect = autoConnect,
        onAutoConnectChange = { viewModel.setAutoConnect(it) },
        onCopyLogs = { viewModel.copyLogs() },
    )
}

@Composable
private fun Settings(
    autoConnect: Boolean,
    onAutoConnectChange: (Boolean) -> Unit,
    onCopyLogs: () -> Unit,
) {
    Column {
        CheckboxWithLabel(
            text = stringResource(R.string.auto_connect),
            isChecked = autoConnect,
            onCheckedChange = { onAutoConnectChange(it) },
        )
        Button(
            onClick = { onCopyLogs() },
            content = { Text(stringResource(R.string.copy_logs_to_clipboard)) },
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewSettings() {
    OpenSCQ30Theme {
        Settings(
            autoConnect = false,
            onAutoConnectChange = {},
            onCopyLogs = {},
        )
    }
}
