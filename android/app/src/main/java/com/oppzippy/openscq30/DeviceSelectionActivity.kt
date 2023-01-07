package com.oppzippy.openscq30

import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.view.Menu
import android.view.MenuItem
import android.view.View
import androidx.appcompat.app.AppCompatActivity
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
        initializeBtleplug()

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
        soundcoreDeviceRegistry.refreshDevices()
        val items = soundcoreDeviceRegistry.devices().map { device ->
            return@map DeviceListItem(device.name(), device.macAddress(), View.OnClickListener {
                val intent = Intent(applicationContext, DeviceSettingsActivity::class.java)
                intent.putExtra("macAddress", device.macAddress())
                startActivity(intent)
            })
        }.toList()
        binding.deviceSelectionListing.adapter = DeviceListItemAdapter(applicationContext, items)
    }

    override fun onCreateOptionsMenu(menu: Menu?): Boolean {
        super.onCreateOptionsMenu(menu)
        menuInflater.inflate(R.menu.device_selection_menu, menu)
        return true
    }
}