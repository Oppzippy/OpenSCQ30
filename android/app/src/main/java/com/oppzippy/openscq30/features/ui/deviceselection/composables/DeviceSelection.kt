package com.oppzippy.openscq30.features.ui.deviceselection.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.google.accompanist.swiperefresh.SwipeRefresh
import com.google.accompanist.swiperefresh.rememberSwipeRefreshState
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.ui.deviceselection.models.BluetoothDeviceModel
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSelection(
    devices: List<BluetoothDeviceModel>,
    onRefreshClick: () -> Unit = {},
    onInfoClick: () -> Unit = {},
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
            IconButton(onClick = onInfoClick) {
                Icon(
                    imageVector = Icons.Filled.Info,
                    contentDescription = stringResource(id = R.string.info),
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
            SwipeRefresh(state = rememberSwipeRefreshState(false), onRefresh = {
                onRefreshClick()
            }) {
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
