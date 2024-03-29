package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.foundation.clickable
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

@Composable
fun DeviceList(
    devices: List<BluetoothDevice>,
    modifier: Modifier = Modifier,
    onDeviceClick: (device: BluetoothDevice) -> Unit = {},
) {
    LazyColumn(
        modifier = modifier,
        userScrollEnabled = true,
    ) {
        items(devices) { device ->
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable {
                        onDeviceClick(device)
                    }
                    .padding(horizontal = 8.dp, vertical = 8.dp),
            ) {
                Text(text = device.name)
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
        for (i in 1..100) {
            devices.add(BluetoothDevice("Device #$i", "00:00:$i"))
        }
        DeviceList(devices)
    }
}
