package com.oppzippy.openscq30.features.ui.devicesettings

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import com.oppzippy.openscq30.features.ui.devicesettings.composables.DeviceSettingsActivityView
import dagger.hilt.android.AndroidEntryPoint

@AndroidEntryPoint
class DeviceSettingsActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        actionBar?.hide()
        val macAddress = intent.getStringExtra("macAddress")
        if (macAddress == null) {
            finish()
            return
        }

        setContent {
            DeviceSettingsActivityView(
                macAddress = macAddress,
                onDeviceNotFound = {
                    finish()
                },
            )
        }
    }
}
