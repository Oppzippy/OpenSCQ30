package com.oppzippy.openscq30.features.preferences

import android.content.Context
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
    }

    private val preferences = context.getSharedPreferences("preferences", Context.MODE_PRIVATE)

    var autoConnect: Boolean
        get() {
            return preferences.getBoolean("autoConnect", false)
        }
        set(value) {
            preferences.edit {
                putBoolean("autoConnect", value)
            }
        }

    var theme: ThemeType?
        get() {
            return preferences.getString("theme", null)?.let { themeName ->
                try {
                    ThemeType.valueOf(themeName)
                } catch (ex: IllegalArgumentException) {
                    Log.e(TAG, "error parsing theme: $themeName", ex)
                    null
                }
            }
        }
        set(theme) {
            _themeFlow.value = theme
            preferences.edit {
                if (theme != null) {
                    putString("theme", theme.name)
                } else {
                    remove("theme")
                }
            }
        }

    private val _themeFlow = MutableStateFlow(theme)
    val themeFlow = _themeFlow.asStateFlow()

    var dynamicColor: Boolean
        get() {
            return preferences.getBoolean("dynamicColor", true)
        }
        set(value) {
            preferences.edit {
                _dynamicColorFlow.value = value
                putBoolean("dynamicColor", value)
            }
        }

    private val _dynamicColorFlow = MutableStateFlow(dynamicColor)
    val dynamicColorFlow = _dynamicColorFlow.asStateFlow()
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
