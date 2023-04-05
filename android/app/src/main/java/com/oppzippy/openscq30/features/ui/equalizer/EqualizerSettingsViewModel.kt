package com.oppzippy.openscq30.features.ui.equalizer

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.features.soundcoredevice.api.contentEquals
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceBox
import com.oppzippy.openscq30.features.ui.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.ui.equalizer.models.EqualizerConfiguration
import com.oppzippy.openscq30.features.ui.equalizer.models.EqualizerProfile
import com.oppzippy.openscq30.features.ui.equalizer.storage.CustomProfileDao
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch
import java.math.BigDecimal
import javax.inject.Inject

val MIN_VOLUME = -120
val MAX_VOLUME = 120;

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
    private val _valueTexts = MutableStateFlow(List(8) { "" })
    val valueTexts = _valueTexts.asStateFlow()

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
            _displayedEqualizerConfiguration.collectLatest {
                if (it != null) {
                    refreshValueTexts(it.values)
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

    private fun setEqualizerConfiguration(profile: EqualizerProfile, values: ByteArray) {
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

    fun onValueTextChange(changedIndex: Int, changedText: String) {
        var reformattedText = changedText
        try {
            val value = BigDecimal(changedText).multiply(BigDecimal.TEN)
                .coerceIn(BigDecimal(MIN_VOLUME), BigDecimal(MAX_VOLUME))
            onValueChange(changedIndex, value.toByte())
            // don't delete trailing decimals
            if (!changedText.endsWith(".") && !changedText.endsWith(",")) {
                reformattedText = value.div(BigDecimal.TEN).toString()
            }
        } catch (_: NumberFormatException) {
        }

        _valueTexts.value = _valueTexts.value.mapIndexed { index, text ->
            if (index == changedIndex) {
                reformattedText
            } else {
                text
            }
        }
    }

    fun onValueChange(changedIndex: Int, changedValue: Byte) {
        _displayedEqualizerConfiguration.value?.let { equalizerConfiguration ->
            setEqualizerConfiguration(
                equalizerConfiguration.equalizerProfile,
                equalizerConfiguration.values.mapIndexed { index, value ->
                    if (index == changedIndex) {
                        changedValue
                    } else {
                        value
                    }
                }.toByteArray(),
            )
        }
    }

    private fun refreshValueTexts(values: List<Byte>) {
        _valueTexts.value = values.map {
            BigDecimal(it.toInt()).divide(BigDecimal.TEN).toString()
        }
    }

    fun selectPresetProfile(profile: EqualizerProfile) {
        _displayedEqualizerConfiguration.value?.let {
            setEqualizerConfiguration(profile, it.values.toByteArray())
        }
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
            if (equalizerConfiguration.equalizerProfile == EqualizerProfile.Custom) {
                _customProfiles.value.find { it.values == equalizerConfiguration.values }
            } else {
                null
            }
    }
}
