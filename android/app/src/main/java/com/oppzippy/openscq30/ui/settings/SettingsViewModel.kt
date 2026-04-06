package com.oppzippy.openscq30.ui.settings

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.content.Intent
import android.util.Log
import android.widget.Toast
import androidx.core.content.getSystemService
import androidx.lifecycle.ViewModel
import com.oppzippy.openscq30.features.autoconnect.AutoConnectService
import com.oppzippy.openscq30.features.preferences.Preferences
import com.oppzippy.openscq30.ui.theme.ThemeType
import com.oppzippy.openscq30.ui.utils.ToastHandler
import dagger.hilt.android.lifecycle.HiltViewModel
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject

@HiltViewModel
class SettingsViewModel @Inject constructor(
    @ApplicationContext private val context: Context,
    private val preferences: Preferences,
    private val toastHandler: ToastHandler,
) : ViewModel() {
    companion object {
        private const val TAG = "SettingsViewModel"
    }

    val autoConnect = preferences.autoConnectFlow
    val theme = preferences.themeFlow
    val dynamicColorEnabled = preferences.dynamicColorFlow

    fun setAutoConnect(value: Boolean) {
        preferences.autoConnect = value
        val intent = Intent(context, AutoConnectService::class.java)
        if (value) {
            context.startService(intent)
        } else {
            context.stopService(intent)
        }
    }

    fun setTheme(theme: ThemeType?) {
        preferences.theme = theme
    }

    fun setDynamicColor(enabled: Boolean) {
        preferences.dynamicColor = enabled
    }

    fun copyLogs() {
        Log.i(TAG, "exporting logs")
        val process = try {
            // logs get spammed with View : setRequestedFrameRate 60 times a second, so filter that out
            Runtime.getRuntime().exec(arrayOf("logcat", "-d", "-t", "200", "View:S"))
        } catch (ex: Exception) {
            toastHandler.add("Failed to execute logcat: ${ex.message}", Toast.LENGTH_SHORT)
            Log.e(TAG, "Failed to execute logcat", ex)
            return
        }
        val logs = process.inputStream.bufferedReader().use { reader ->
            reader.useLines { lines -> lines.joinToString("\n") }
        }

        val clipboardManager = context.getSystemService<ClipboardManager>()
        if (clipboardManager != null) {
            clipboardManager.setPrimaryClip(ClipData.newPlainText("OpenSCQ30 logs", logs))
        } else {
            toastHandler.add("Failed to get clipboard manager", Toast.LENGTH_SHORT)
        }
    }

    fun copyLogsUnfiltered() {
        Log.i(TAG, "exporting logs (unfiltered)")
        val process = try {
            // -d: print logs and exit without blocking
            // -t: limit number of lines to print
            Runtime.getRuntime().exec(arrayOf("logcat", "-d", "-t", "4000"))
        } catch (ex: Exception) {
            toastHandler.add("Failed to execute logcat: ${ex.message}", Toast.LENGTH_SHORT)
            Log.e(TAG, "Failed to execute logcat", ex)
            return
        }
        val logs = process.inputStream.bufferedReader().use { reader ->
            reader.useLines { lines -> lines.joinToString("\n") }
        }

        val clipboardManager = context.getSystemService<ClipboardManager>()
        if (clipboardManager != null) {
            clipboardManager.setPrimaryClip(ClipData.newPlainText("OpenSCQ30 logs", logs))
        } else {
            toastHandler.add("Failed to get clipboard manager", Toast.LENGTH_SHORT)
        }
    }
}
