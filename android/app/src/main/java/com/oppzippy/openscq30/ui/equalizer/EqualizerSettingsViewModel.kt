package com.oppzippy.openscq30.ui.equalizer

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.VolumeAdjustments
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.equalizer.models.EqualizerProfile
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.filterNotNull
import kotlinx.coroutines.flow.mapLatest
import kotlinx.coroutines.launch
import java.math.BigDecimal
import javax.inject.Inject
import kotlin.jvm.optionals.getOrNull

@OptIn(ExperimentalCoroutinesApi::class, FlowPreview::class)
@HiltViewModel
class EqualizerSettingsViewModel @Inject constructor(
    private val customProfileDao: CustomProfileDao,
) : ViewModel() {
    private val minVolume = VolumeAdjustments.minVolume().toInt()
    private val maxVolume = VolumeAdjustments.maxVolume().toInt()

    private val _displayedEqualizerConfiguration: MutableStateFlow<EqualizerConfiguration?> =
        MutableStateFlow(null)
    val displayedEqualizerConfiguration = _displayedEqualizerConfiguration.asStateFlow()
    private val _selectedCustomProfile = MutableStateFlow<CustomProfile?>(null)
    val selectedCustomProfile = _selectedCustomProfile.asStateFlow()
    private val _customProfiles = MutableStateFlow<List<CustomProfile>>(listOf())
    val customProfiles = _customProfiles.asStateFlow()
    private val _valueTexts = MutableStateFlow(List(8) { "" })
    val valueTexts = _valueTexts.asStateFlow()

    var setRealEqualizerConfiguration: (
        equalizerConfiguration: EqualizerConfiguration,
    ) -> Unit = {}
    private val uiStateFlow = MutableStateFlow<UiDeviceState.Connected?>(null)

    init {
        viewModelScope.launch {
            uiStateFlow.mapLatest { state ->
                state?.deviceState?.equalizerConfiguration
            }.filterNotNull().distinctUntilChanged().collectLatest {
                _displayedEqualizerConfiguration.value = it
            }
        }

        viewModelScope.launch {
            _displayedEqualizerConfiguration.collectLatest {
                if (it != null) {
                    refreshValueTexts(it.volumeAdjustments().adjustments().toList())
                }
            }
        }

        viewModelScope.launch {
            _displayedEqualizerConfiguration.debounce(500).collectLatest {
                if (it != null) {
                    setRealEqualizerConfiguration(it)
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

    private fun setDisplayedEqualizerConfiguration(profile: EqualizerProfile, values: ByteArray) {
        // Values match the display, not the profile. Creating the rust EqualizerConfiguration first
        // will use proper values.
        val configuration = profile.toEqualizerConfiguration(values)
        _displayedEqualizerConfiguration.value = configuration
    }

    fun createCustomProfile(name: String) {
        _displayedEqualizerConfiguration.value?.let {
            viewModelScope.launch {
                customProfileDao.insert(
                    CustomProfile(
                        name,
                        it.volumeAdjustments().adjustments().toList(),
                    ),
                )
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
        setDisplayedEqualizerConfiguration(
            EqualizerProfile.Custom,
            customProfile.values.toByteArray(),
        )
    }

    fun onValueTextChange(changedIndex: Int, changedText: String) {
        var reformattedText = changedText
        try {
            val value = BigDecimal(changedText).multiply(BigDecimal.TEN)
                .coerceIn(BigDecimal(minVolume), BigDecimal(maxVolume))
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
            setDisplayedEqualizerConfiguration(
                EqualizerProfile.Custom,
                equalizerConfiguration.volumeAdjustments().adjustments()
                    .mapIndexed { index, value ->
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
            setDisplayedEqualizerConfiguration(
                profile,
                it.volumeAdjustments().adjustments(),
            )
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
            if (equalizerConfiguration.presetProfile().getOrNull() == null) {
                _customProfiles.value.find {
                    it.values == equalizerConfiguration.volumeAdjustments().adjustments()
                        .toList()
                }
            } else {
                null
            }
    }

    fun pushUiState(uiState: UiDeviceState.Connected) {
        uiStateFlow.value = uiState
    }
}
