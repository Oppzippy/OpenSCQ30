package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDevice
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDeviceFactory
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import kotlinx.coroutines.launch

@Composable
fun DeviceSettingsActivityView(
    macAddress: String,
    soundcoreDeviceFactory: SoundcoreDeviceFactory,
    onDeviceNotFound: () -> Unit,
) {
    OpenSCQ30Theme {
        Surface(
            modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background,
        ) {
            val coroutineScope = rememberCoroutineScope()
            var soundcoreDevice by remember { mutableStateOf<SoundcoreDevice?>(null) }
            DisposableEffect(macAddress) {
                val job = coroutineScope.launch {
                    soundcoreDevice = soundcoreDeviceFactory.createSoundcoreDevice(macAddress)
                    if (soundcoreDevice == null) {
                        onDeviceNotFound()
                    }
                }
                onDispose {
                    job.cancel()
                    soundcoreDevice?.destroy()
                    soundcoreDevice = null
                }
            }

            soundcoreDevice.let {
                if (it != null) {
                    SoundcoreDeviceSettings(it)
                } else {
                    Loading()
                }
            }
        }
    }
}