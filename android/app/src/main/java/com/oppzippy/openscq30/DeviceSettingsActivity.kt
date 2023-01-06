package com.oppzippy.openscq30

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.fragment.app.Fragment
import com.oppzippy.openscq30.databinding.ActivityDeviceSettingsBinding
import com.oppzippy.openscq30.lib.SoundcoreDevice
import com.oppzippy.openscq30.ui.equalizer.EqualizerFragment
import com.oppzippy.openscq30.ui.general.GeneralFragment
import kotlinx.coroutines.flow.subscribe
import kotlin.jvm.optionals.getOrNull

class DeviceSettingsActivity : AppCompatActivity() {

    private lateinit var soundcoreDevice: SoundcoreDevice
    private lateinit var binding: ActivityDeviceSettingsBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val macAddress = intent.getStringExtra("macAddress")

        val maybeDevice = macAddress?.let {
            soundcoreDeviceRegistry.deviceByMacAddress(macAddress)
        }?.getOrNull()
        if (maybeDevice != null) {
            soundcoreDevice = maybeDevice
        } else {
            finish()
            return
        }

        binding = ActivityDeviceSettingsBinding.inflate(layoutInflater)
        setContentView(binding.root)

        val general = GeneralFragment()
        val equalizer = EqualizerFragment()

        setCurrentFragment(general)

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