package com.oppzippy.openscq30.ui.settings

import androidx.lifecycle.ViewModel
import com.oppzippy.openscq30.features.preferences.Preferences
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import javax.inject.Inject

@HiltViewModel
class SettingsViewModel @Inject constructor(
    private val preferences: Preferences,
) : ViewModel() {
    private val _autoConnect = MutableStateFlow(preferences.autoConnect)
    val autoConnect = _autoConnect.asStateFlow()

    fun setAutoConnect(value: Boolean) {
        _autoConnect.value = value
        preferences.autoConnect = value
    }
}
