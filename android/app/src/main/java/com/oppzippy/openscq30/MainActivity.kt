package com.oppzippy.openscq30

import android.os.Bundle
import android.util.Log
import com.google.android.material.bottomnavigation.BottomNavigationView
import androidx.appcompat.app.AppCompatActivity
import androidx.fragment.app.Fragment
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.setupActionBarWithNavController
import androidx.navigation.ui.setupWithNavController
import com.oppzippy.openscq30.databinding.ActivityMainBinding
import com.oppzippy.openscq30.lib.Init
import com.oppzippy.openscq30.lib.SoundcoreDeviceRegistry
import com.oppzippy.openscq30.ui.equalizer.EqualizerFragment
import com.oppzippy.openscq30.ui.general.GeneralFragment

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        System.loadLibrary("openscq30_android")
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        Init.logging()
        val reg = SoundcoreDeviceRegistry()
        reg.refreshDevices()
        Log.i("devices", reg.devices().size.toString())

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