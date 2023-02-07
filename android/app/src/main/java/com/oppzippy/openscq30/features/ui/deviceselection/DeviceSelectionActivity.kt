package com.oppzippy.openscq30.features.ui.deviceselection

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import com.oppzippy.openscq30.features.ui.deviceselection.composables.DeviceSelectionActivityView
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

@AndroidEntryPoint
class DeviceSelectionActivity : ComponentActivity() {
    @Inject
    lateinit var bluetoothDeviceProvider: BluetoothDeviceProvider

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        actionBar?.hide()
        setContent {
            DeviceSelectionActivityView(
                bluetoothDeviceProvider = bluetoothDeviceProvider,
            )
        }
    }
}
