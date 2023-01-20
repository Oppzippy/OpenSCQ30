package com.oppzippy.openscq30.ui.deviceselection

import android.Manifest
import android.content.Intent
import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsActivity
import com.oppzippy.openscq30.lib.Init
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

@AndroidEntryPoint
class DeviceSelectionActivity : ComponentActivity() {
    @Inject
    lateinit var bluetoothDeviceProvider: BluetoothDeviceProvider

    init {
        System.loadLibrary("openscq30_android")
        Init.logging()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        actionBar?.hide()
        setContent {
            var devices by remember { mutableStateOf(bluetoothDeviceProvider.getDevices()) }

            val launcher = rememberLauncherForActivityResult(
                contract = ActivityResultContracts.RequestPermission(),
            ) { isGranted ->
                if (isGranted) {
                    devices = bluetoothDeviceProvider.getDevices()
                }
            }

            OpenSCQ30Theme {
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    DeviceSelection(devices, onRefreshClick = {
                        if (Build.VERSION.SDK_INT >= 31) {
                            launcher.launch(Manifest.permission.BLUETOOTH_CONNECT)
                        } else {
                            launcher.launch(Manifest.permission.BLUETOOTH)
                        }
                    }, onDeviceClick = { device ->
                        val intent = Intent(applicationContext, DeviceSettingsActivity::class.java)
                        intent.putExtra("macAddress", device.address)
                        startActivity(intent)
                    })
                }
            }
        }
    }

}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSelection(
    devices: List<BluetoothDeviceModel>,
    onRefreshClick: () -> Unit = {},
    onDeviceClick: (BluetoothDeviceModel) -> Unit = {},
) {
    Scaffold(topBar = {
        TopAppBar(title = {
            Text(text = stringResource(id = R.string.app_name))
        }, actions = {
            IconButton(onClick = onRefreshClick) {
                Icon(
                    imageVector = Icons.Filled.Refresh,
                    contentDescription = stringResource(id = R.string.refresh),
                )
            }
        })
    }, content = { innerPadding ->
        Column(
            modifier = Modifier
                .padding(innerPadding)
                .fillMaxWidth()
                .fillMaxHeight()
        ) {
            if (devices.isEmpty()) {
                NoDevicesFound()
            } else {
                DeviceList(
                    devices = devices,
                    modifier = Modifier.fillMaxWidth(),
                    onDeviceClick = onDeviceClick,
                )
            }
        }
    })
}


@Preview(showBackground = true)
@Composable
private fun NoDevicesFoundPreview() {
    OpenSCQ30Theme {
        DeviceSelection(listOf())
    }
}

@Preview(showBackground = true)
@Composable
private fun DevicesPreview() {
    OpenSCQ30Theme {
        val devices = ArrayList<BluetoothDeviceModel>()
        for (i in 1..100) {
            devices.add(BluetoothDeviceModel("Device #${i}", "00:00:${i}"))
        }
        DeviceSelection(devices)
    }
}
