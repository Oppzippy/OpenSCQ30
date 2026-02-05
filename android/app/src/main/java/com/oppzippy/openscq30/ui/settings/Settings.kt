package com.oppzippy.openscq30.ui.settings

import android.os.Build
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.hilt.lifecycle.viewmodel.compose.hiltViewModel
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.theme.ThemeType
import com.oppzippy.openscq30.ui.utils.LabeledSwitch
import com.oppzippy.openscq30.ui.utils.Select

@Composable
fun Settings(viewModel: SettingsViewModel = hiltViewModel()) {
    val autoConnect by viewModel.autoConnect.collectAsState()
    val theme by viewModel.theme.collectAsState()
    val dynamicColorEnabled by viewModel.dynamicColorEnabled.collectAsState()
    Settings(
        autoConnect = autoConnect,
        onAutoConnectChange = { viewModel.setAutoConnect(it) },
        theme = theme,
        onThemeChange = { viewModel.setTheme(it) },
        dynamicColorEnabled = dynamicColorEnabled,
        onDynamicColorChange = { viewModel.setDynamicColor(it) },
        onCopyLogs = { viewModel.copyLogs() },
    )
}

@Composable
private fun Settings(
    autoConnect: Boolean,
    onAutoConnectChange: (Boolean) -> Unit,
    theme: ThemeType?,
    onThemeChange: (ThemeType?) -> Unit,
    dynamicColorEnabled: Boolean,
    onDynamicColorChange: (Boolean) -> Unit,
    onCopyLogs: () -> Unit,
) {
    Column(
        Modifier
            .verticalScroll(rememberScrollState())
            .padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        LabeledSwitch(
            label = stringResource(R.string.auto_connect),
            isChecked = autoConnect,
            onCheckedChange = { onAutoConnectChange(it) },
        )

        val themes = listOf(
            null to stringResource(R.string.system_theme),
            ThemeType.Light to stringResource(R.string.light),
            ThemeType.Dark to stringResource(R.string.dark),
        )
        Select(
            name = stringResource(R.string.theme),
            options = themes.map { (_, themeName) -> themeName },
            onSelect = { onThemeChange(themes[it].first) },
            selectedIndex = themes.indexOfFirst { it.first == theme },
        )

        // Android 12+ is required for Material You dynamic color. The setting will have no effect otherwise.
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            LabeledSwitch(
                label = stringResource(R.string.dynamic_color),
                isChecked = dynamicColorEnabled,
                onCheckedChange = { onDynamicColorChange(it) },
            )
        }

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
            theme = null,
            onThemeChange = {},
            dynamicColorEnabled = true,
            onDynamicColorChange = {},
            onCopyLogs = {},
        )
    }
}
