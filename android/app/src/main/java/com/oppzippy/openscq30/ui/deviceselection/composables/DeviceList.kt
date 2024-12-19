package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun DeviceList(
    devices: List<BluetoothDevice>,
    modifier: Modifier = Modifier,
    onDeviceClick: (device: BluetoothDevice) -> Unit = {},
    onUnpair: (device: BluetoothDevice) -> Unit = {},
) {
    var deviceToUnpair: BluetoothDevice? by remember { mutableStateOf(null) }

    deviceToUnpair?.let { device ->
        AlertDialog(
            onDismissRequest = { deviceToUnpair = null },
            title = { Text(stringResource(id = R.string.unpair_device)) },
            text = { Text(stringResource(id = R.string.unpairing_from_device_name, device.name)) },
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
                    Text(text = device.name)

                    if (device.isAssociated) {
                        Text(text = stringResource(R.string.paired))
                    } else {
                        Text(text = stringResource(R.string.not_paired), color = MaterialTheme.colorScheme.secondary)
                    }
                }
                Text(text = device.address)
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewDeviceList() {
    OpenSCQ30Theme {
        val devices = ArrayList<BluetoothDevice>()
        for (i in 1..3) {
            devices.add(BluetoothDevice("Device #$i", "00:00:$i", isAssociated = i % 2 == 0))
        }
        DeviceList(devices)
    }
}
