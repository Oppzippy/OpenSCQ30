package com.oppzippy.openscq30.ui.deviceselection

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.ui.deviceselection.composables.DeviceSelection

@Composable
fun DeviceSelectionScreen(
    onDeviceSelected: (device: BluetoothDevice) -> Unit,
    viewModel: DeviceSelectionViewModel = hiltViewModel(),
) {
    val devices by viewModel.devices.collectAsState()
    DeviceSelection(
        devices = devices,
        onRefreshDevices = { viewModel.refreshDevices() },
        onDeviceSelected = onDeviceSelected,
    )
}
