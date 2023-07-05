package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.pullrefresh.PullRefreshIndicator
import androidx.compose.material.pullrefresh.pullRefresh
import androidx.compose.material.pullrefresh.rememberPullRefreshState
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class, ExperimentalMaterialApi::class)
@Composable
fun DeviceListing(
    devices: List<BluetoothDevice>,
    onRefreshClick: () -> Unit = {},
    onInfoClick: () -> Unit = {},
    onDeviceClick: (BluetoothDevice) -> Unit = {},
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
            IconButton(onClick = onInfoClick) {
                Icon(
                    imageVector = Icons.Filled.Info,
                    contentDescription = stringResource(id = R.string.info),
                )
            }
        })
    }, content = { innerPadding ->
        val pullRefreshState = rememberPullRefreshState(
            refreshing = false, // Refresh is instant
            onRefresh = { onRefreshClick() },
        )
        Box(
            Modifier
                .padding(innerPadding)
                .fillMaxSize()
                .pullRefresh(pullRefreshState),
        ) {
            if (devices.isEmpty()) {
                NoDevicesFound()
            } else {
                DeviceList(
                    devices = devices,
                    modifier = Modifier
                        .fillMaxWidth()
                        .fillMaxHeight(),
                    onDeviceClick = onDeviceClick,
                )
            }

            PullRefreshIndicator(
                refreshing = false,
                state = pullRefreshState,
                Modifier.align(Alignment.TopCenter),
            )
        }
    })
}

@Preview(showBackground = true)
@Composable
private fun PreviewNoDevicesFound() {
    OpenSCQ30Theme {
        DeviceListing(listOf())
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewDeviceListing() {
    OpenSCQ30Theme {
        val devices = ArrayList<BluetoothDevice>()
        for (i in 1..100) {
            devices.add(BluetoothDevice("Device #$i", "00:00:$i"))
        }
        DeviceListing(devices)
    }
}
