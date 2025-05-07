package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.runtime.Composable
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.ui.devicesettings.composables.DeviceSettings
import com.oppzippy.openscq30.ui.devicesettings.composables.Disconnected
import com.oppzippy.openscq30.ui.utils.Loading

@Composable
fun DeviceSettingsScreen(onBack: () -> Unit = {}, connectionStatus: ConnectionStatus) {
    when (connectionStatus) {
        is ConnectionStatus.Connected -> {
            DeviceSettings(
                connectionStatus = connectionStatus,
                onBack = onBack,
            )
        }

        ConnectionStatus.AwaitingConnection -> Loading()
        is ConnectionStatus.Connecting -> Loading()
        ConnectionStatus.Disconnected -> Disconnected()
    }
}
