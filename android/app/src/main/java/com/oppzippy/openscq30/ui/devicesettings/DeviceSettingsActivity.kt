package com.oppzippy.openscq30.ui.devicesettings

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDeviceFactory
import com.oppzippy.openscq30.ui.devicesettings.composables.DeviceSettingsActivityView
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

@AndroidEntryPoint
class DeviceSettingsActivity : ComponentActivity() {
    @Inject
    lateinit var soundcoreDeviceFactory: SoundcoreDeviceFactory

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
                soundcoreDeviceFactory = soundcoreDeviceFactory,
                onDeviceNotFound = {
                    finish()
                },
            )
        }
    }
}
