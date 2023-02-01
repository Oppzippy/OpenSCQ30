package com.oppzippy.openscq30.features.ui.equalizer

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.features.soundcoredevice.contentEquals
import com.oppzippy.openscq30.features.soundcoredevice.SoundcoreDeviceBox
import com.oppzippy.openscq30.features.ui.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.ui.equalizer.models.EqualizerConfiguration
import com.oppzippy.openscq30.features.ui.equalizer.models.EqualizerProfile
import com.oppzippy.openscq30.features.ui.equalizer.storage.CustomProfileDao
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch
import javax.inject.Inject

@OptIn(ExperimentalCoroutinesApi::class, FlowPreview::class)
@HiltViewModel
class EqualizerSettingsViewModel @Inject constructor(
    private val deviceBox: SoundcoreDeviceBox,
    private val customProfileDao: CustomProfileDao,
) : ViewModel() {
    private val _displayedEqualizerConfiguration: MutableStateFlow<EqualizerConfiguration?> =
        MutableStateFlow(null)
    val displayedEqualizerConfiguration = _displayedEqualizerConfiguration.asStateFlow()
    private val _selectedCustomProfile = MutableStateFlow<CustomProfile?>(null)
    val selectedCustomProfile = _selectedCustomProfile.asStateFlow()
    private val _customProfiles = MutableStateFlow<List<CustomProfile>>(listOf())
    val customProfiles = _customProfiles.asStateFlow()

    init {
        viewModelScope.launch {
            deviceBox.device.collectLatest { device ->
                if (device == null) {
                    throw IllegalStateException("device must not be null")
                }
                device.stateFlow.mapLatest {
                    it.equalizerConfiguration()
                }.distinctUntilChanged { old, new -> old.contentEquals(new) }.collectLatest {
                    _displayedEqualizerConfiguration.value = EqualizerConfiguration.fromRust(it)
                }
            }
        }

        viewModelScope.launch {
            _displayedEqualizerConfiguration.debounce(500).collectLatest {
                if (it != null) {
                    deviceBox.device.value?.setEqualizerConfiguration(it.toRust())
                }
            }
        }

        viewModelScope.launch {
            refreshCustomProfiles()
            _displayedEqualizerConfiguration.collectLatest { equalizerConfiguration ->
                if (equalizerConfiguration != null) {
                    updateSelectedCustomProfile(equalizerConfiguration)
                }
            }
        }
    }

    fun setEqualizerConfiguration(profile: EqualizerProfile, values: ByteArray) {
        // Values match the display, not the profile. Creating the rust EqualizerConfiguration first
        // will use proper values.
        val configuration =
            EqualizerConfiguration.fromRust(profile.toEqualizerConfiguration(values))
        _displayedEqualizerConfiguration.value = configuration
    }

    fun createCustomProfile(name: String) {
        _displayedEqualizerConfiguration.value?.let {
            viewModelScope.launch {
                customProfileDao.insert(CustomProfile(name, it.values))
                refreshCustomProfiles()
            }
        }
    }

    fun deleteCustomProfile(name: String) {
        viewModelScope.launch {
            customProfileDao.delete(name)
            refreshCustomProfiles()
        }
    }

    fun selectCustomProfile(customProfile: CustomProfile) {
        setEqualizerConfiguration(EqualizerProfile.Custom, customProfile.values.toByteArray())
    }

    private suspend fun refreshCustomProfiles() {
        _customProfiles.value = customProfileDao.getAll()
        _selectedCustomProfile.value = _customProfiles.value.find {
            it.name == _selectedCustomProfile.value?.name
        }
        _displayedEqualizerConfiguration.value?.let { equalizerConfiguration ->
            updateSelectedCustomProfile(equalizerConfiguration)
        }
    }

    private fun updateSelectedCustomProfile(equalizerConfiguration: EqualizerConfiguration) {
        _selectedCustomProfile.value =
            if (equalizerConfiguration?.equalizerProfile == EqualizerProfile.Custom) {
                _customProfiles.value.find { it.values == equalizerConfiguration.values }
            } else {
                null
            }
    }
}