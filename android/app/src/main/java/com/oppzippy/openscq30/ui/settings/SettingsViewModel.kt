package com.oppzippy.openscq30.ui.settings

import android.content.Context
import android.content.Intent
import androidx.lifecycle.ViewModel
import com.oppzippy.openscq30.features.autoconnect.AutoConnectService
import com.oppzippy.openscq30.features.preferences.Preferences
import dagger.hilt.android.lifecycle.HiltViewModel
import dagger.hilt.android.qualifiers.ApplicationContext
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import javax.inject.Inject

@HiltViewModel
class SettingsViewModel @Inject constructor(
    @ApplicationContext private val context: Context,
    private val preferences: Preferences,
) : ViewModel() {
    private val _autoConnect = MutableStateFlow(preferences.autoConnect)
    val autoConnect = _autoConnect.asStateFlow()

    fun setAutoConnect(value: Boolean) {
        _autoConnect.value = value
        preferences.autoConnect = value
        val intent = Intent(context, AutoConnectService::class.java)
        if (value) {
            context.startService(intent)
        } else {
            context.stopService(intent)
        }
    }
}
