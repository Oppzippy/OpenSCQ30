package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import kotlinx.coroutines.launch

@Composable
fun DeviceSettingsActivityView(
    macAddress: String,
    onDeviceNotFound: () -> Unit,
    viewModel: DeviceSettingsActivityViewModel = hiltViewModel(),
) {
    OpenSCQ30Theme {
        Surface(
            modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background,
        ) {
            val coroutineScope = rememberCoroutineScope()
            val soundcoreDevice by viewModel.soundcoreDeviceBox.device.collectAsState()
            DisposableEffect(macAddress) {
                val job = coroutineScope.launch {
                    viewModel.setMacAddress(macAddress)
                    if (viewModel.soundcoreDeviceBox.device.value == null) {
                        onDeviceNotFound()
                    }
                }
                onDispose {
                    job.cancel()
                    soundcoreDevice?.destroy()
                }
            }

            soundcoreDevice.let {
                if (it != null) {
                    DeviceSettings()
                } else {
                    Loading()
                }
            }
        }
    }
}