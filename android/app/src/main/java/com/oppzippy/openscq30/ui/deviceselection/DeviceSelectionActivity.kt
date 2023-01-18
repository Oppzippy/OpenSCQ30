package com.oppzippy.openscq30.ui.deviceselection

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.util.Log
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
import androidx.core.app.ActivityCompat
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsActivity
import com.oppzippy.openscq30.lib.Init
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

class DeviceSelectionActivity : ComponentActivity() {
    init {
        System.loadLibrary("openscq30_android")
        Init.logging()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            var devices by remember { mutableStateOf(getDevices()) }

            val launcher = rememberLauncherForActivityResult(
                contract = ActivityResultContracts.RequestPermission(),
            ) { isGranted ->
                if (isGranted) {
                    devices = getDevices()
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

    private fun getDevices(): List<BluetoothDeviceModel> {
        val bluetoothManager: BluetoothManager = getSystemService(BluetoothManager::class.java)
        val adapter: BluetoothAdapter? = bluetoothManager.adapter
        if (adapter != null) {
            if (ActivityCompat.checkSelfPermission(
                    this, Manifest.permission.BLUETOOTH_CONNECT
                ) == PackageManager.PERMISSION_GRANTED
            ) {
                return adapter.bondedDevices.map {
                    BluetoothDeviceModel(it.name, it.address)
                }
            } else {
                Log.w("device-selection", "no permission")
            }
        } else {
            Log.w("device-selection", "no bluetooth adapter")
        }
        return listOf()
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
