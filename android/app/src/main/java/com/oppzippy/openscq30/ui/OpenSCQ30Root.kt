package com.oppzippy.openscq30.ui

import androidx.activity.compose.BackHandler
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.Crossfade
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import com.oppzippy.openscq30.ui.deviceselection.DeviceSelectionRoot
import com.oppzippy.openscq30.ui.devicesettings.composables.DeviceSettingsRoot
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import kotlinx.coroutines.flow.asStateFlow

@Composable
fun OpenSCQ30Root(viewModel: RootViewModel = hiltViewModel()) {
    val lifecycleOwner = LocalLifecycleOwner.current

    OpenSCQ30Theme {
        DisposableEffect(1) {
            val observer = LifecycleEventObserver { _, event ->
                when (event) {
                    Lifecycle.Event.ON_START -> viewModel.bind()
                    Lifecycle.Event.ON_STOP -> viewModel.unbind()
                    else -> {}
                }
            }
            lifecycleOwner.lifecycle.addObserver(observer)
            onDispose {
                lifecycleOwner.lifecycle.removeObserver(observer)
            }
        }
        val deviceState by viewModel.deviceState.collectAsState()
        Crossfade(targetState = deviceState is UiDeviceState.Connected || deviceState is UiDeviceState.Loading) { isConnected ->
            BackHandler(enabled = isConnected) {
                viewModel.deselectDevice()
            }
            if (isConnected) {
                DeviceSettingsRoot(
                    deviceState = deviceState,
                    onBack = { viewModel.deselectDevice() },
                    onAmbientSoundModeChange = { viewModel.setAmbientSoundMode(it) },
                    onNoiseCancelingModeChange = { viewModel.setNoiseCancelingMode(it) },
                    onEqualizerConfigurationChange = { viewModel.setEqualizerConfiguration(it) },
                )
            } else {
                val devices by viewModel.devices.collectAsState()
                DeviceSelectionRoot(
                    devices = devices,
                    onRefreshDevices = { viewModel.refreshDevices() },
                    onDeviceSelected = { viewModel.selectDevice(it) },
                )
            }
        }
    }
}
