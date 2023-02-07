package com.oppzippy.openscq30.features.ui.deviceselection.composables

import android.Manifest
import android.content.Intent
import android.os.Build
import androidx.compose.foundation.layout.*
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.ui.devicesettings.DeviceSettingsActivity

@OptIn(ExperimentalPermissionsApi::class)
@Composable
fun DeviceSelectionPermissionCheck(
    bluetoothDeviceProvider: BluetoothDeviceProvider,
    onInfoClick: () -> Unit,
) {
    val permissionState = if (Build.VERSION.SDK_INT >= 31) {
        rememberPermissionState(Manifest.permission.BLUETOOTH_CONNECT)
    } else {
        rememberPermissionState(Manifest.permission.BLUETOOTH)
    }

    val context = LocalContext.current

    if (!permissionState.status.isGranted) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.Center,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(
                modifier = Modifier.fillMaxHeight(),
                verticalArrangement = Arrangement.Center,
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                Text(stringResource(R.string.bluetooth_permission_is_required))
                Button(onClick = { permissionState.launchPermissionRequest() }) {
                    Text(stringResource(R.string.request_permission))
                }
            }
        }
    } else {
        var devices by remember { mutableStateOf(bluetoothDeviceProvider.getDevices()) }
        DeviceSelection(
            devices,
            onRefreshClick = {
                devices = bluetoothDeviceProvider.getDevices()
            },
            onDeviceClick = { device ->
                val intent = Intent(context, DeviceSettingsActivity::class.java)
                intent.putExtra("macAddress", device.address)
                context.startActivity(intent)
            },
            onInfoClick = onInfoClick,
        )
    }
}