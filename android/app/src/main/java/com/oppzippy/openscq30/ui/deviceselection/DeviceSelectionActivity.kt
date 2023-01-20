package com.oppzippy.openscq30.ui.deviceselection

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
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
