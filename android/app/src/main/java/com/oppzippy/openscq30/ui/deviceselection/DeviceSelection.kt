package com.oppzippy.openscq30.ui.deviceselection

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

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
