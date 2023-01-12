package com.oppzippy.openscq30

import android.os.Bundle
import android.util.Log
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.databinding.DataBindingUtil.setContentView
import androidx.lifecycle.*
import com.oppzippy.openscq30.databinding.ActivityDeviceSettingsBinding
import com.oppzippy.openscq30.databinding.FragmentDeviceSettingsBinding
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDevice
import com.oppzippy.openscq30.ui.equalizer.EqualizerFragment
import com.oppzippy.openscq30.ui.general.GeneralFragment
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch

class DeviceSettings(private val device: SoundcoreDevice) : Fragment() {
    private lateinit var binding: FragmentDeviceSettingsBinding

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?
    ): View? {
        super.onCreate(savedInstanceState)

        binding = FragmentDeviceSettingsBinding.inflate(layoutInflater)

        val deviceStateFlow =
            device.stateFlow.stateIn(lifecycleScope, SharingStarted.Eagerly, device.state)
        val general = GeneralFragment()
        val equalizer = EqualizerFragment(deviceStateFlow)

        setCurrentFragment(general)

        lifecycleScope.launch {
            general.soundMode.debounce(100).collectLatest {
                device.setSoundMode(it.first, it.second)
            }
        }
        lifecycleScope.launch {
            equalizer.equalizerConfiguration.debounce(250).collectLatest {
                device.setEqualizerConfiguration(it)
            }
        }

        lifecycleScope.launch {
            deviceStateFlow.collectLatest {
                general.setAmbientSoundMode(it.ambientSoundMode())
                general.setNoiseCancelingMode(it.noiseCancelingMode())
            }
        }

        binding.navView.setOnItemSelectedListener {
            when (it.itemId) {
                R.id.navigation_general -> setCurrentFragment(general)
                R.id.navigation_equalizer -> setCurrentFragment(equalizer)
            }
            true
        }

        return binding.root
    }

    private fun setCurrentFragment(fragment: Fragment) {
        parentFragmentManager.beginTransaction().apply {
            replace(binding.frameLayout.id, fragment)
            commit()
        }
    }
}