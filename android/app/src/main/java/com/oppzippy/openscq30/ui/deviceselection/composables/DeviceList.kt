package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
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
    LazyColumn(
        modifier = modifier,
        userScrollEnabled = true,
    ) {
        items(devices) { device ->
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .combinedClickable(
                        onClick = { onDeviceClick(device) },
                        onLongClick = { onUnpair(device) },
                    )
                    .padding(horizontal = 8.dp, vertical = 8.dp),
            ) {
                Text(text = device.name)
                Text(text = device.address.toString())
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewDeviceList() {
    OpenSCQ30Theme {
        val devices = ArrayList<BluetoothDevice>()
        for (i in 1..100) {
            devices.add(BluetoothDevice("Device #$i", "00:00:$i"))
        }
        DeviceList(devices)
    }
}
