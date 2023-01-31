package com.oppzippy.openscq30.features.ui.deviceselection.composables

import android.Manifest
import android.content.Intent
import android.os.Build
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.oppzippy.openscq30.features.ui.deviceselection.models.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.ui.devicesettings.DeviceSettingsActivity
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.R

@OptIn(ExperimentalPermissionsApi::class)
@Composable
fun DeviceSelectionActivityView(
    bluetoothDeviceProvider: BluetoothDeviceProvider
) {
    val permissionState = if (Build.VERSION.SDK_INT >= 31) {
        rememberPermissionState(Manifest.permission.BLUETOOTH_CONNECT)
    } else {
        rememberPermissionState(Manifest.permission.BLUETOOTH)
    }

    val context = LocalContext.current

    OpenSCQ30Theme {
        Surface(
            modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
        ) {
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
                )
            }
        }
    }

}