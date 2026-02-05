package com.oppzippy.openscq30.features.preferences

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.core.content.edit
import androidx.hilt.lifecycle.viewmodel.compose.hiltViewModel
import androidx.lifecycle.ViewModel
import com.oppzippy.openscq30.ui.theme.ThemeType
import com.oppzippy.openscq30.ui.theme.prefersDarkTheme
import dagger.hilt.android.lifecycle.HiltViewModel
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject
import javax.inject.Singleton
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

@Singleton
class Preferences @Inject constructor(@ApplicationContext context: Context) {
    companion object {
        const val TAG = "Preferences"

        private const val PREFERENCE_AUTO_CONNECT = "autoConnect"
        private const val PREFERENCE_THEME = "theme"
        private const val PREFERENCE_DYNAMIC_COLOR = "dynamicColor"
    }

    private val preferences = context.getSharedPreferences("preferences", Context.MODE_PRIVATE)

    private val autoConnectPreference = Preference(
        get = { preferences.getBoolean(PREFERENCE_AUTO_CONNECT, false) },
        set = { preferences.edit { putBoolean(PREFERENCE_AUTO_CONNECT, it) } },
    )
    private val themePreference = Preference(
        get = {
            preferences.getString(PREFERENCE_THEME, null)?.let { themeName ->
                try {
                    ThemeType.valueOf(themeName)
                } catch (ex: IllegalArgumentException) {
                    Log.e(TAG, "error parsing theme: $themeName", ex)
                    null
                }
            }
        },
        set = {
            preferences.edit {
                if (it != null) {
                    putString(PREFERENCE_THEME, it.name)
                } else {
                    remove(PREFERENCE_THEME)
                }
            }
        },
    )
    private val dynamicColorPreference = Preference(
        get = { preferences.getBoolean(PREFERENCE_DYNAMIC_COLOR, true) },
        set = { preferences.edit { putBoolean(PREFERENCE_DYNAMIC_COLOR, it) } },
    )

    private val preferenceKeysToPreferences = mapOf(
        PREFERENCE_AUTO_CONNECT to autoConnectPreference,
        PREFERENCE_THEME to themePreference,
        PREFERENCE_DYNAMIC_COLOR to dynamicColorPreference,
    )
    private val onChangeListener =
        SharedPreferences.OnSharedPreferenceChangeListener { _, key -> preferenceKeysToPreferences[key]?.refresh() }

    init {
        preferences.registerOnSharedPreferenceChangeListener(onChangeListener)
    }

    val autoConnectFlow = autoConnectPreference.flow
    var autoConnect: Boolean
        get() = autoConnectPreference.flow.value
        set(value) = autoConnectPreference.set(value)

    val themeFlow = themePreference.flow
    var theme: ThemeType?
        get() = themePreference.flow.value
        set(theme) = themePreference.set(theme)

    val dynamicColorFlow = dynamicColorPreference.flow
    var dynamicColor: Boolean
        get() = dynamicColorPreference.flow.value
        set(theme) = dynamicColorPreference.set(theme)
}

private data class Preference<T>(val get: () -> T, val set: (T) -> Unit) {
    private val _flow = MutableStateFlow(get())
    val flow = _flow.asStateFlow()

    fun refresh() {
        _flow.value = get()
    }
}

@HiltViewModel
class ThemeViewModel @Inject constructor(preferences: Preferences) : ViewModel() {
    val themeFlow = preferences.themeFlow
    val dynamicColorFlow = preferences.dynamicColorFlow
}

@Composable
fun prefersDarkTheme(themeViewModel: ThemeViewModel = hiltViewModel()): Boolean =
    themeViewModel.themeFlow.collectAsState().value.prefersDarkTheme()

@Composable
fun prefersDynamicColor(themeViewModel: ThemeViewModel = hiltViewModel()): Boolean =
    themeViewModel.dynamicColorFlow.collectAsState().value
