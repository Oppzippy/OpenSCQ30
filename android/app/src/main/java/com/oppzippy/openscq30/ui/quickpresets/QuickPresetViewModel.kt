package com.oppzippy.openscq30.ui.quickpresets

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

@HiltViewModel
class QuickPresetViewModel @Inject constructor(
    private val quickPresetRepository: QuickPresetRepository,
    customProfileDao: CustomProfileDao,
) : ViewModel() {
    private val _quickPreset = MutableStateFlow<QuickPreset?>(null)
    val quickPreset = _quickPreset.asStateFlow()
    val customEqualizerProfiles =
        customProfileDao.all().stateIn(viewModelScope, SharingStarted.Eagerly, emptyList())

    fun selectQuickPreset(deviceModel: String, index: Int) {
        viewModelScope.launch {
            _quickPreset.value =
                quickPresetRepository.getForDevice(deviceModel).getOrNull(index)
                    ?: QuickPreset(id = null, deviceModel = deviceModel, index = index)
        }
    }

    fun clearSelection() {
        _quickPreset.value = null
    }

    fun upsertQuickPreset(quickPreset: QuickPreset) {
        _quickPreset.value = quickPreset
        viewModelScope.launch {
            quickPresetRepository.insert(quickPreset)
        }
    }
}
