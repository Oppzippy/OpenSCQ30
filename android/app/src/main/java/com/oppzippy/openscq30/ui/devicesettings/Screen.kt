package com.oppzippy.openscq30.ui.devicesettings

import androidx.annotation.StringRes
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Equalizer
import androidx.compose.material.icons.filled.ImportExport
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.RadioButtonChecked
import androidx.compose.material.icons.filled.Settings
import androidx.compose.ui.graphics.vector.ImageVector
import com.oppzippy.openscq30.R
import kotlinx.serialization.Serializable

@Serializable
sealed class Screen {
    @Serializable
    data object ScreenSelection

    @Serializable
    data object General : Screen() {
        val screenInfo = ScreenInfo(this, R.string.general, Icons.Filled.Settings)
    }

    @Serializable
    data object Equalizer : Screen() {
        val screenInfo = ScreenInfo(this, R.string.equalizer, Icons.Filled.Equalizer)
    }

    @Serializable
    data object QuickPresets : Screen() {
        val screenInfo = ScreenInfo(this, R.string.quick_presets, Icons.Filled.Settings)
    }

    @Serializable
    data object ButtonActions : Screen() {
        val screenInfo = ScreenInfo(this, R.string.button_actions, Icons.Filled.RadioButtonChecked)
    }

    @Serializable
    data object DeviceInfo : Screen() {
        val screenInfo = ScreenInfo(this, R.string.device_info, Icons.Filled.Info)
    }

    @Serializable
    class ImportExport(val index: Int = -1) : Screen() {
        companion object {
            val screenInfo =
                ScreenInfo(ImportExport(), R.string.import_export, Icons.Filled.ImportExport)
        }
    }
}

data class ScreenInfo(
    val baseRoute: Screen,
    @StringRes val nameResourceId: Int,
    val icon: ImageVector,
)
