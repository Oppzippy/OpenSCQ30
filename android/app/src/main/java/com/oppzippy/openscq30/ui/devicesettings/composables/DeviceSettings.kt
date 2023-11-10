package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingModeType
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.lib.bindings.TransparencyModeType
import com.oppzippy.openscq30.ui.deviceinfo.DeviceInfoScreen
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.equalizer.EqualizerSettings
import com.oppzippy.openscq30.ui.quickpresets.QuickPresetScreen
import com.oppzippy.openscq30.ui.soundmode.NoiseCancelingType
import com.oppzippy.openscq30.ui.soundmode.SoundModeSettings
import kotlin.jvm.optionals.getOrNull

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings(
    uiState: UiDeviceState.Connected,
    onBack: () -> Unit = {},
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onCustomNoiseCancelingChange: (customNoiseCanceling: CustomNoiseCanceling) -> Unit = {},
    onEqualizerConfigurationChange: (equalizerConfiguration: EqualizerConfiguration) -> Unit = {},
) {
    val navController = rememberNavController()
    val navItems = ArrayList<Screen>()
    if (uiState.deviceState.deviceProfile.soundMode().isPresent) {
        navItems.add(Screen.General)
    }
    if (uiState.deviceState.deviceProfile.numEqualizerChannels() > 0) {
        navItems.add(Screen.Equalizer)
    }
    navItems.add(Screen.QuickPresets)
    navItems.add(Screen.DeviceInfo)
    Scaffold(
        topBar = {
            TopAppBar(title = {
                Text(uiState.name)
            }, navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(
                        imageVector = Icons.Filled.ArrowBack,
                        contentDescription = stringResource(R.string.back),
                    )
                }
            })
        },
        bottomBar = {
            NavigationBar {
                val navBarStackEntry by navController.currentBackStackEntryAsState()
                val currentDestination = navBarStackEntry?.destination
                navItems.forEach { screen ->
                    NavigationBarItem(
                        icon = { Icon(screen.icon, contentDescription = null) },
                        label = { Text(stringResource(screen.resourceId)) },
                        selected = currentDestination?.route == screen.route,
                        onClick = {
                            navController.navigate(screen.route) {
                                popUpTo(navController.graph.id)
                                launchSingleTop = true
                            }
                        },
                    )
                }
            }
        },
    ) { innerPadding ->
        NavHost(
            navController = navController,
            startDestination = navItems.first().route,
            modifier = Modifier.padding(innerPadding),
        ) {
            uiState.deviceState.soundModes?.let { soundModes ->
                composable(Screen.General.route) {
                    val soundModeProfile = uiState.deviceState.deviceProfile.soundMode().getOrNull()
                        ?: return@composable
                    SoundModeSettings(
                        soundModes = soundModes,
                        hasTransparencyModes = soundModeProfile.transparencyModeType() == TransparencyModeType.Custom,
                        noiseCancelingType = when (soundModeProfile.noiseCancelingModeType()) {
                            NoiseCancelingModeType.None -> NoiseCancelingType.None
                            NoiseCancelingModeType.Basic -> NoiseCancelingType.Normal
                            NoiseCancelingModeType.Custom -> NoiseCancelingType.Custom
                            null -> throw NotImplementedError("should be unreachable since the enum is not nullable")
                        },
                        onAmbientSoundModeChange = onAmbientSoundModeChange,
                        onTransparencyModeChange = onTransparencyModeChange,
                        onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                        onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
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
            composable(Screen.DeviceInfo.route) {
                DeviceInfoScreen(deviceState = uiState.deviceState)
            }
        }
    }
}
