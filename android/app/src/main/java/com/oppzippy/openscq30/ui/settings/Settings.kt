package com.oppzippy.openscq30.ui.settings

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
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
    )
}

@Composable
private fun Settings(autoConnect: Boolean, onAutoConnectChange: (Boolean) -> Unit) {
    Column {
        Row {
            CheckboxWithLabel(
                text = stringResource(R.string.auto_connect),
                isChecked = autoConnect,
                onCheckedChange = { onAutoConnectChange(it) },
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewSettings() {
    OpenSCQ30Theme {
        Settings(
            autoConnect = false,
            onAutoConnectChange = {},
        )
    }
}
