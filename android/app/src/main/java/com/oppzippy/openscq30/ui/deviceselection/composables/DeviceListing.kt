package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.material3.pulltorefresh.rememberPullToRefreshState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceListing(
    devices: List<BluetoothDevice>,
    onRefreshClick: () -> Unit = {},
    onInfoClick: () -> Unit = {},
    onDeviceClick: (BluetoothDevice) -> Unit = {},
    onSettingsClick: () -> Unit = {},
    onUnpair: (BluetoothDevice) -> Unit = {},
    isBluetoothEnabled: Boolean,
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(text = stringResource(id = R.string.app_name))
                },
                actions = {
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
                    IconButton(onClick = onSettingsClick) {
                        Icon(
                            imageVector = Icons.Filled.Settings,
                            contentDescription = stringResource(id = R.string.settings),
                        )
                    }
                },
            )
        },
        content = { innerPadding ->
            val pullToRefreshState = rememberPullToRefreshState()
            val scope = rememberCoroutineScope()
            PullToRefreshBox(
                modifier = Modifier
                    .padding(innerPadding)
                    .fillMaxSize(),
                isRefreshing = false,
                onRefresh = {
                    onRefreshClick()
                    scope.launch {
                        pullToRefreshState.animateToHidden()
                    }
                },
                state = pullToRefreshState,
            ) {
                // child needs to be scrollable so that pull to refresh works
                if (!isBluetoothEnabled) {
                    CenteredScrollableBox {
                        Text(
                            stringResource(R.string.bluetooth_is_disabled),
                            color = MaterialTheme.colorScheme.error,
                        )
                    }
                } else if (devices.isEmpty()) {
                    CenteredScrollableBox {
                        Text(stringResource(R.string.no_devices_found))
                    }
                } else {
                    DeviceList(
                        devices = devices,
                        modifier = Modifier.fillMaxSize(),
                        onDeviceClick = onDeviceClick,
                        onUnpair = onUnpair,
                    )
                }
            }
        },
    )
}

@Composable
private fun CenteredScrollableBox(content: @Composable () -> Unit) {
    Box(
        Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState()),
        contentAlignment = Alignment.Center,
    ) {
        content()
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewBluetoothDisabled() {
    OpenSCQ30Theme {
        DeviceListing(listOf(), isBluetoothEnabled = false)
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewEmptyListing() {
    OpenSCQ30Theme {
        DeviceListing(listOf(), isBluetoothEnabled = true)
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewDeviceListing() {
    OpenSCQ30Theme {
        val devices = ArrayList<BluetoothDevice>()
        for (i in 1..10) {
            devices.add(BluetoothDevice("Device #$i", "00:00:$i", i % 2 == 0))
        }
        DeviceListing(devices, isBluetoothEnabled = true)
    }
}
