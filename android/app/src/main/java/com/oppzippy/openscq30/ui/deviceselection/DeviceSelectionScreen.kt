package com.oppzippy.openscq30.ui.deviceselection

import android.app.Activity
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.platform.LocalContext
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.ui.deviceselection.composables.DeviceSelection

@Composable
fun DeviceSelectionScreen(
    onDeviceSelected: (device: BluetoothDevice) -> Unit,
    viewModel: DeviceSelectionViewModel = hiltViewModel(),
) {
    val devices by viewModel.devices.collectAsState()
    val activity = LocalContext.current as Activity
    DeviceSelection(
        devices = devices,
        onRefreshDevices = { viewModel.refreshDevices() },
        onDeviceClick = {
            if (it.isAssociated) {
                onDeviceSelected(it)
            } else {
                viewModel.pair(activity, it.address)
            }
        },
        onUnpair = { viewModel.unpair(it) },
        isBluetoothEnabled = viewModel.isBluetoothEnabled(),
    )
}
