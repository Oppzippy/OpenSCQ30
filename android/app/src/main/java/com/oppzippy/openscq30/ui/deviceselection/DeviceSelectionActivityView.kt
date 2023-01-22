package com.oppzippy.openscq30.ui.deviceselection

import android.Manifest
import android.content.Intent
import android.os.Build
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsActivity
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme


@Composable
fun DeviceSelectionActivityView(
    bluetoothDeviceProvider: BluetoothDeviceProvider
) {
    var devices by remember { mutableStateOf(bluetoothDeviceProvider.getDevices()) }

    val launcher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestPermission(),
    ) { isGranted ->
        if (isGranted) {
            devices = bluetoothDeviceProvider.getDevices()
        }
    }

    val context = LocalContext.current

    OpenSCQ30Theme {
        Surface(
            modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
        ) {
            DeviceSelection(devices, onRefreshClick = {
                launcher.launch(Manifest.permission.BLUETOOTH)
            }, onDeviceClick = { device ->
                val intent = Intent(context, DeviceSettingsActivity::class.java)
                intent.putExtra("macAddress", device.address)
                context.startActivity(intent)
            })
        }
    }

}