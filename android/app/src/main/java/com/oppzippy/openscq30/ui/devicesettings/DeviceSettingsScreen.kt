package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.runtime.Composable
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.ui.devicesettings.composables.DeviceSettings
import com.oppzippy.openscq30.ui.devicesettings.composables.Disconnected
import com.oppzippy.openscq30.ui.utils.Loading
import kotlinx.coroutines.flow.Flow
import kotlin.String

@Composable
fun DeviceSettingsScreen(
    onBack: () -> Unit = {},
    connectionStatus: ConnectionStatus,
    categoryIds: List<String>,
    getSettingsInCategoryFlow: (String) -> Flow<List<Pair<String, Setting>>>,
    setSettings: (List<Pair<String, Value>>) -> Unit,
) {
    when (connectionStatus) {
        is ConnectionStatus.Connected -> {
            DeviceSettings(
                connectionStatus = connectionStatus,
                onBack = onBack,
                categoryIds = categoryIds,
                getSettingsInCategoryFlow = getSettingsInCategoryFlow,
                setSettings = setSettings,
            )
        }

        ConnectionStatus.AwaitingConnection -> Loading()
        is ConnectionStatus.Connecting -> Loading()
        ConnectionStatus.Disconnected -> Disconnected()
    }
}
