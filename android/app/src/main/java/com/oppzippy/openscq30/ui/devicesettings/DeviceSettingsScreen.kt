package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.runtime.Composable
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.ManualNoiseCanceling
import com.oppzippy.openscq30.lib.wrapper.MultiButtonConfiguration
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingModeTypeTwo
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.ui.devicesettings.composables.DeviceSettings
import com.oppzippy.openscq30.ui.devicesettings.composables.Disconnected
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.utils.Loading

@Composable
fun DeviceSettingsScreen(
    onBack: () -> Unit = {},
    deviceState: UiDeviceState,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onAmbientSoundModeCycleChange: (ambientSoundMode: AmbientSoundModeCycle) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onCustomNoiseCancelingChange: (customNoiseCanceling: UByte) -> Unit = {},
    onEqualizerConfigurationChange: (equalizerConfiguration: EqualizerConfiguration) -> Unit = {},
    onButtonConfigurationChange: (MultiButtonConfiguration) -> Unit = {},
    onAmbientSoundModeTypeTwoChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onTransparencyModeTypeTwoChange: (transparencyMode: TransparencyMode) -> Unit = {},
    onNoiseCancelingModeTypeTwoChange: (noiseCancelingMode: NoiseCancelingModeTypeTwo) -> Unit = {},
    onManualNoiseCancelingChange: (manualNoiseCanceling: ManualNoiseCanceling) -> Unit = {},
) {
    deviceState.let { uiDeviceState ->
        when (uiDeviceState) {
            is UiDeviceState.Connected -> {
                DeviceSettings(
                    uiState = uiDeviceState,
                    onBack = onBack,
                    onAmbientSoundModeChange = onAmbientSoundModeChange,
                    onAmbientSoundModeCycleChange = onAmbientSoundModeCycleChange,
                    onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                    onEqualizerConfigurationChange = onEqualizerConfigurationChange,
                    onTransparencyModeChange = onTransparencyModeChange,
                    onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
                    onButtonConfigurationChange = onButtonConfigurationChange,
                    onAmbientSoundModeTypeTwoChange = onAmbientSoundModeTypeTwoChange,
                    onTransparencyModeTypeTwoChange = onTransparencyModeTypeTwoChange,
                    onNoiseCancelingModeTypeTwoChange = onNoiseCancelingModeTypeTwoChange,
                    onManualNoiseCancelingChange = onManualNoiseCancelingChange,
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
