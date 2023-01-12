package com.oppzippy.openscq30

import android.Manifest
import android.bluetooth.BluetoothManager
import android.content.pm.PackageManager
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.fragment.app.Fragment
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.databinding.ActivityDeviceSettingsBinding
import com.oppzippy.openscq30.soundcoredevice.createSoundcoreDevice
import kotlinx.coroutines.launch

class DeviceSettingsActivity : AppCompatActivity() {

    private lateinit var binding: ActivityDeviceSettingsBinding

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

        binding = ActivityDeviceSettingsBinding.inflate(layoutInflater)
        setContentView(binding.root)

        val loading = Loading()
        setCurrentFragment(loading)

        lifecycleScope.launch {
            val soundcoreDevice = createSoundcoreDevice(applicationContext, lifecycleScope, bluetoothDevice)
            setCurrentFragment(DeviceSettings(soundcoreDevice))
        }
    }

    private fun setCurrentFragment(fragment: Fragment) {
        supportFragmentManager.beginTransaction().apply {
            replace(binding.content.id, fragment)
            commit()
        }
    }
}