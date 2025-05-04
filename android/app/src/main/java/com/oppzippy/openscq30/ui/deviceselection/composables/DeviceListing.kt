package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.material3.pulltorefresh.rememberPullToRefreshState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceListing(
    devices: List<PairedDevice>,
    onRefreshClick: () -> Unit = {},
    onInfoClick: () -> Unit = {},
    onDeviceClick: (PairedDevice) -> Unit = {},
    onSettingsClick: () -> Unit = {},
    onUnpair: (PairedDevice) -> Unit = {},
    onAddDeviceClick: () -> Unit = {},
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(text = stringResource(id = R.string.app_name))
                },
                actions = {
                    IconButton(onClick = onAddDeviceClick) {
                        Icon(
                            imageVector = Icons.Filled.Add,
                            contentDescription = stringResource(id = R.string.add),
                        )
                    }
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
                if (devices.isEmpty()) {
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

@Composable
fun DeviceList(
    devices: List<PairedDevice>,
    modifier: Modifier = Modifier,
    onDeviceClick: (device: PairedDevice) -> Unit = {},
    onUnpair: (device: PairedDevice) -> Unit = {},
) {
    var deviceToUnpair: PairedDevice? by remember { mutableStateOf(null) }

    deviceToUnpair?.let { device ->
        AlertDialog(
            onDismissRequest = { deviceToUnpair = null },
            title = { Text(stringResource(id = R.string.unpair_device)) },
            text = {
                Text(
                    stringResource(
                        id = R.string.unpairing_from_device_name,
                        translateDeviceModel(device.model),
                    ),
                )
            },
            dismissButton = {
                TextButton(onClick = { deviceToUnpair = null }) {
                    Text(stringResource(R.string.cancel))
                }
            },
            confirmButton = {
                TextButton(
                    onClick = {
                        onUnpair(device)
                        deviceToUnpair = null
                    },
                ) {
                    Text(stringResource(R.string.unpair))
                }
            },
        )
    }

    LazyColumn(
        modifier = modifier,
        horizontalAlignment = Alignment.CenterHorizontally,
        userScrollEnabled = true,
    ) {
        items(devices) { device ->
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .combinedClickable(
                        onClick = { onDeviceClick(device) },
                        onLongClick = { deviceToUnpair = device },
                    )
                    .padding(horizontal = 8.dp, vertical = 8.dp),
            ) {
                Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                    Text(text = translateDeviceModel(device.model))

                    if (device.isDemo) {
                        Text(text = stringResource(R.string.demo), color = MaterialTheme.colorScheme.secondary)
                    }
                }
                Text(text = device.macAddress)
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewDeviceList() {
    OpenSCQ30Theme {
        val devices = ArrayList<PairedDevice>()
        for (i in 1..3) {
            devices.add(PairedDevice("Device #$i", "00:00:$i", isDemo = i % 2 == 0))
        }
        DeviceList(devices)
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewEmptyListing() {
    OpenSCQ30Theme {
        DeviceListing(listOf())
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewDeviceListing() {
    OpenSCQ30Theme {
        val devices = ArrayList<PairedDevice>()
        for (i in 1..10) {
            devices.add(PairedDevice("Device #$i", "00:00:$i", i % 2 == 0))
        }
        DeviceListing(devices)
    }
}
