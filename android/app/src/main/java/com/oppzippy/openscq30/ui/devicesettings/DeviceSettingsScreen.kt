package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.runtime.Composable
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.ui.devicesettings.composables.DeviceSettings
import com.oppzippy.openscq30.ui.devicesettings.composables.Disconnected
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.utils.Loading

@Composable
fun DeviceSettingsScreen(
    onBack: () -> Unit = {},
    deviceState: UiDeviceState,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onEqualizerConfigurationChange: (equalizerConfiguration: EqualizerConfiguration) -> Unit = {},
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
