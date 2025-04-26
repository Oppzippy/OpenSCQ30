package com.oppzippy.openscq30.ui.devicesettings

import androidx.annotation.StringRes
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Equalizer
import androidx.compose.material.icons.filled.ImportExport
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.RadioButtonChecked
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.Speaker
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.res.stringResource
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateCategoryId
import kotlinx.serialization.Serializable

@Serializable
sealed class Screen {
    @Serializable
    data object ScreenSelection

    @Serializable
    data class SettingsCategory(val categoryId: String) : Screen() {
        fun screenInfo(): ScreenInfo = ScreenInfo(
            this,
            StringResourceOrString.RawString(translateCategoryId(categoryId)),
            when (categoryId) {
                "General" -> Icons.Filled.Settings
                "SoundModes" -> Icons.Filled.Speaker
                "Equalizer" -> Icons.Filled.Equalizer
                "ButtonConfiguration" -> Icons.Filled.RadioButtonChecked
                "DeviceInformation" -> Icons.Filled.Info
                else -> Icons.Filled.Settings
            },
        )
    }

    @Serializable
    class ImportExport(val index: Int = -1) : Screen() {
        companion object {
            val screenInfo =
                ScreenInfo(
                    ImportExport(),
                    StringResourceOrString.StringResource(R.string.import_export),
                    Icons.Filled.ImportExport,
                )
        }
    }
}

data class ScreenInfo(val baseRoute: Screen, val name: StringResourceOrString, val icon: ImageVector)

sealed class StringResourceOrString {
    data class StringResource(@StringRes val nameResourceId: Int) : StringResourceOrString()
    data class RawString(val string: String) : StringResourceOrString()

    @Composable
    fun translated(): String = when (this) {
        is StringResource -> stringResource(nameResourceId)
        is RawString -> string
    }
}
