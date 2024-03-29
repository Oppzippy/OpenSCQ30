package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.lib.wrapper.CustomButtonModel
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingModeType
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.lib.wrapper.TransparencyModeType
import com.oppzippy.openscq30.ui.buttonactions.ButtonActionSelection
import com.oppzippy.openscq30.ui.buttonactions.ButtonActions
import com.oppzippy.openscq30.ui.deviceinfo.DeviceInfoScreen
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.equalizer.EqualizerSettings
import com.oppzippy.openscq30.ui.quickpresets.QuickPresetScreen
import com.oppzippy.openscq30.ui.soundmode.NoiseCancelingType
import com.oppzippy.openscq30.ui.soundmode.SoundModeSettings

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings(
    uiState: UiDeviceState.Connected,
    onBack: () -> Unit = {},
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onAmbientSoundModeCycleChange: (ambientSoundMode: AmbientSoundModeCycle) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onCustomNoiseCancelingChange: (customNoiseCanceling: UByte) -> Unit = {},
    onEqualizerConfigurationChange: (equalizerConfiguration: EqualizerConfiguration) -> Unit = {},
    onCustomButtonModelChange: (CustomButtonModel) -> Unit = {},
) {
    val navController = rememberNavController()
    val screens = ArrayList<Screen>()
    if (uiState.deviceState.deviceProfile.soundMode != null) {
        screens.add(Screen.General)
    }
    if (uiState.deviceState.deviceProfile.numEqualizerChannels > 0) {
        screens.add(Screen.Equalizer)
    }
    screens.add(Screen.QuickPresets)
    if (uiState.deviceState.deviceProfile.hasCustomButtonModel) {
        screens.add(Screen.ButtonActions)
    }
    screens.add(Screen.DeviceInfo)
    Scaffold(
        topBar = {
            TopAppBar(title = {
                Text(uiState.name)
            }, navigationIcon = {
                IconButton(onClick = {
                    val isAtTopOfStack = !navController.popBackStack()
                    if (isAtTopOfStack) {
                        onBack()
                    }
                }) {
                    Icon(
                        imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                        contentDescription = stringResource(R.string.back),
                    )
                }
            })
        },
    ) { innerPadding ->
        NavHost(
            navController = navController,
            startDestination = "/",
            modifier = Modifier.padding(innerPadding),
        ) {
            composable("/") {
                ScreenSelection(screens = screens, onNavigation = { screen ->
                    navController.navigate(screen.route) {
                        popUpTo("/")
                        launchSingleTop = true
                    }
                })
            }
            uiState.deviceState.soundModes?.let { soundModes ->
                composable(Screen.General.route) {
                    val soundModeProfile =
                        uiState.deviceState.deviceProfile.soundMode ?: return@composable
                    SoundModeSettings(
                        soundModes = soundModes,
                        ambientSoundModeCycle = uiState.deviceState.ambientSoundModeCycle,
                        hasTransparencyModes = soundModeProfile.transparencyModeType == TransparencyModeType.Custom,
                        noiseCancelingType = when (soundModeProfile.noiseCancelingModeType) {
                            NoiseCancelingModeType.None -> NoiseCancelingType.None
                            NoiseCancelingModeType.Basic -> NoiseCancelingType.Normal
                            NoiseCancelingModeType.Custom -> NoiseCancelingType.Custom
                        },
                        onAmbientSoundModeChange = onAmbientSoundModeChange,
                        onTransparencyModeChange = onTransparencyModeChange,
                        onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                        onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
                        onAmbientSoundModeCycleChange = onAmbientSoundModeCycleChange,
                    )
                }
            }
            composable(Screen.Equalizer.route) {
                EqualizerSettings(
                    uiState = uiState,
                    onEqualizerConfigurationChange = onEqualizerConfigurationChange,
                )
            }
            composable(Screen.QuickPresets.route) {
                QuickPresetScreen(
                    deviceProfile = uiState.deviceState.deviceProfile,
                    deviceBleServiceUuid = uiState.deviceBleServiceUuid,
                )
            }
            uiState.deviceState.customButtonModel?.let { buttonModel ->
                composable(Screen.ButtonActions.route) {
                    ButtonActionSelection(
                        buttonActions = ButtonActions(
                            buttonModel.leftSingleClick.actionOrNull(),
                            buttonModel.leftDoubleClick.connectedActionOrNull(),
                            buttonModel.leftLongPress.connectedActionOrNull(),
                            buttonModel.rightSingleClick.actionOrNull(),
                            buttonModel.rightDoubleClick.connectedActionOrNull(),
                            buttonModel.rightLongPress.connectedActionOrNull(),
                        ),
                        onChange = {
                            onCustomButtonModelChange(it.toCustomButtonModel(buttonModel))
                        },
                    )
                }
            }
            composable(Screen.DeviceInfo.route) {
                DeviceInfoScreen(deviceState = uiState.deviceState)
            }
        }
    }
}
