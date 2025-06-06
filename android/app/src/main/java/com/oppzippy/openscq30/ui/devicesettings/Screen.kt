package com.oppzippy.openscq30.ui.devicesettings

import androidx.annotation.StringRes
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Equalizer
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Notifications
import androidx.compose.material.icons.filled.RadioButtonChecked
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.Speaker
import androidx.compose.material.icons.filled.Update
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
                "general" -> Icons.Filled.Settings
                "soundModes" -> Icons.Filled.Speaker
                "equalizer" -> Icons.Filled.Equalizer
                "buttonConfiguration" -> Icons.Filled.RadioButtonChecked
                "deviceInformation" -> Icons.Filled.Info
                else -> Icons.Filled.Settings
            },
        )
    }

    @Serializable
    data object QuickPresets : Screen() {
        val screenInfo =
            ScreenInfo(
                this,
                StringResourceOrString.StringResource(R.string.quick_presets),
                Icons.Filled.Settings,
            )
    }

    @Serializable
    data class EditQuickPreset(val name: String) : Screen()

    @Serializable
    data object StatusNotification : Screen() {
        val screenInfo = ScreenInfo(
            this,
            StringResourceOrString.StringResource(R.string.status_notification),
            Icons.Filled.Notifications,
        )
    }

    @Serializable
    data object MigrateLegacyEqualizerProfiles : Screen() {
        val screenInfo =
            ScreenInfo(
                this,
                StringResourceOrString.StringResource(R.string.migrate_legacy_equalizer_profiles),
                Icons.Filled.Update,
            )
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
