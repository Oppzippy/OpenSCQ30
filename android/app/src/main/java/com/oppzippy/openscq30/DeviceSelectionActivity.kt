package com.oppzippy.openscq30

import android.os.Bundle
import android.view.Menu
import androidx.appcompat.app.AppCompatActivity
import androidx.recyclerview.widget.LinearLayoutManager
import com.oppzippy.openscq30.databinding.ActivityDeviceSelectionBinding
import com.oppzippy.openscq30.lib.Init
import com.oppzippy.openscq30.lib.SoundcoreDeviceRegistry
import com.oppzippy.openscq30.ui.devicelistitem.DeviceListItem
import com.oppzippy.openscq30.ui.devicelistitem.DeviceListItemAdapter

class DeviceSelectionActivity : AppCompatActivity() {
    private lateinit var binding: ActivityDeviceSelectionBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        System.loadLibrary("openscq30_android")
        Init.logging()
        BtleplugInitializer().init()

        binding = ActivityDeviceSelectionBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setSupportActionBar(binding.appBar)

        val reg = SoundcoreDeviceRegistry()
        reg.refreshDevices()

        val items: ArrayList<DeviceListItem> = ArrayList()
        items.add(DeviceListItem("Soundcore Q30", "00:00:00:00:00:00"))

        binding.deviceSelectionListing.layoutManager = LinearLayoutManager(this)
        binding.deviceSelectionListing.adapter = DeviceListItemAdapter(applicationContext, items)

    }

    override fun onCreateOptionsMenu(menu: Menu?): Boolean {
        super.onCreateOptionsMenu(menu)
        menuInflater.inflate(R.menu.device_selection_menu, menu)
        return true
    }
}