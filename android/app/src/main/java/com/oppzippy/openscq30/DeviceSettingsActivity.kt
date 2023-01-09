package com.oppzippy.openscq30

import android.Manifest
import android.bluetooth.BluetoothManager
import android.content.pm.PackageManager
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.fragment.app.Fragment
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.whenStarted
import com.oppzippy.openscq30.databinding.ActivityDeviceSettingsBinding
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDevice
import com.oppzippy.openscq30.ui.equalizer.EqualizerFragment
import com.oppzippy.openscq30.ui.general.GeneralFragment
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.launch

class DeviceSettingsActivity : AppCompatActivity() {

    private lateinit var binding: ActivityDeviceSettingsBinding
    private lateinit var device: SoundcoreDevice

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val macAddress = intent.getStringExtra("macAddress")
        val bluetoothManager: BluetoothManager = getSystemService(BluetoothManager::class.java)
        if (ActivityCompat.checkSelfPermission(
                this, Manifest.permission.BLUETOOTH_CONNECT
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            // TODO: Consider calling
            //    ActivityCompat#requestPermissions
            // here to request the missing permissions, and then overriding
            //   public void onRequestPermissionsResult(int requestCode, String[] permissions,
            //                                          int[] grantResults)
            // to handle the case where the user grants the permission. See the documentation
            // for ActivityCompat#requestPermissions for more details.
            finish()
            return
        }
        val bluetoothDevice =
            bluetoothManager.adapter.bondedDevices.find { it.address == macAddress }
        if (bluetoothDevice == null) {
            finish()
            return
        }
        val soundcoreDevice = SoundcoreDevice(applicationContext, bluetoothDevice)
        this.device = soundcoreDevice

        binding = ActivityDeviceSettingsBinding.inflate(layoutInflater)
        setContentView(binding.root)

        val general = GeneralFragment()
        val equalizer = EqualizerFragment()


        setCurrentFragment(general)

        this.lifecycleScope.launch {
            general.lifecycle.whenStarted {
                general.ambientSoundMode.collect {
                    if (it != null) {
                        soundcoreDevice.setAmbientSoundMode(it)
                    }
                }
            }
        }
        this.lifecycleScope.launch {
            general.lifecycle.whenStarted {
                general.noiseCancelingMode.collect {
                    if (it != null) {
                        soundcoreDevice.setNoiseCancelingMode(it)
                    }
                }
            }
        }

        this.lifecycleScope.launch {
            general.lifecycle.whenStarted {
                device.state.collect { state ->
                    if (state != null) {
                        general.setAmbientSoundMode(state.ambientSoundMode())
                        general.setNoiseCancelingMode(state.noiseCancelingMode())
                    }
                }
            }
        }

        binding.navView.setOnItemSelectedListener {
            when (it.itemId) {
                R.id.navigation_general -> setCurrentFragment(general)
                R.id.navigation_equalizer -> setCurrentFragment(equalizer)
            }
            true
        }
    }

    private fun setCurrentFragment(fragment: Fragment) {
        supportFragmentManager.beginTransaction().apply {
            replace(binding.frameLayout.id, fragment)
            commit()
        }
    }
}