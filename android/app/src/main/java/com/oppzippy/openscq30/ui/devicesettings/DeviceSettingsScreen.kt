package com.oppzippy.openscq30.ui.devicesettings

import DeviceSettings
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.devicesettings.composables.Disconnected
import com.oppzippy.openscq30.ui.devicesettings.composables.Loading
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState

@Composable
fun DeviceSettingsScreen(
    onBack: () -> Unit = {},
    deviceState: UiDeviceState,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onEqualizerConfigurationChange: (equalizerConfiguration: EqualizerConfiguration) -> Unit = {},
) {
    Surface(
        modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background,
    ) {
        deviceState.let { uiDeviceState ->
            when (uiDeviceState) {
                is UiDeviceState.Connected -> {
                    DeviceSettings(
                        uiState = uiDeviceState,
                        onBack = onBack,
                        onAmbientSoundModeChange = onAmbientSoundModeChange,
                        onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                        onEqualizerConfigurationChange = onEqualizerConfigurationChange,
                    )
                }

                UiDeviceState.Loading -> {
                    Loading()
                }

                UiDeviceState.Disconnected -> {
                    Disconnected()
                }
            }
        }
    }
}
