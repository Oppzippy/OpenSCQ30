package com.oppzippy.openscq30

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
import android.util.Log
import android.view.Menu
import android.view.MenuItem
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.recyclerview.widget.LinearLayoutManager
import com.oppzippy.openscq30.databinding.ActivityDeviceSelectionBinding
import com.oppzippy.openscq30.lib.Init
import com.oppzippy.openscq30.ui.devicelistitem.DeviceListItem
import com.oppzippy.openscq30.ui.devicelistitem.DeviceListItemAdapter

class DeviceSelectionActivity : AppCompatActivity() {
    private lateinit var binding: ActivityDeviceSelectionBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        System.loadLibrary("openscq30_android")
        Init.logging()

        binding = ActivityDeviceSelectionBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setSupportActionBar(binding.appBar)

        refreshDevices()

        binding.deviceSelectionListing.layoutManager = LinearLayoutManager(this)
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        return when (item.itemId) {
            R.id.refresh -> {
                refreshDevices()
                true
            }
            else -> super.onOptionsItemSelected(item)
        }
    }

    private fun refreshDevices() {
        val bluetoothManager: BluetoothManager = getSystemService(BluetoothManager::class.java)
        val adapter: BluetoothAdapter? = bluetoothManager.adapter
        if (adapter != null) {
            val items = if (ActivityCompat.checkSelfPermission(
                    this,
                    Manifest.permission.BLUETOOTH_CONNECT
                ) == PackageManager.PERMISSION_GRANTED
            ) {
                adapter.bondedDevices.map { device ->
                    return@map DeviceListItem(device.name, device.address) {
                        val intent = Intent(applicationContext, DeviceSettingsActivity::class.java)
                        intent.putExtra("macAddress", device.address)
                        startActivity(intent)
                    }
                }.toList()
            } else {
                Log.w("device-selection", "no permission")
                ArrayList()
            }
            binding.deviceSelectionListing.adapter =
                DeviceListItemAdapter(applicationContext, items)
        } else {
            Log.w("device-selection", "no bluetooth adapter")
        }
    }

    override fun onCreateOptionsMenu(menu: Menu?): Boolean {
        super.onCreateOptionsMenu(menu)
        menuInflater.inflate(R.menu.device_selection_menu, menu)
        return true
    }
}