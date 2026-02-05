package com.oppzippy.openscq30.ui.devicesettings

import androidx.annotation.DrawableRes
import androidx.annotation.StringRes
import androidx.compose.runtime.Composable
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
                "general" -> R.drawable.settings_24px
                "soundModes" -> R.drawable.speaker_24px
                "equalizer" -> R.drawable.equalizer_24px
                "buttonConfiguration" -> R.drawable.radio_button_checked_24px
                "deviceInformation" -> R.drawable.info_24px
                "equalizerImportExport" -> R.drawable.swap_vert_24px
                else -> R.drawable.settings_24px
            },
        )
    }

    @Serializable
    data object QuickPresets : Screen() {
        val screenInfo =
            ScreenInfo(
                this,
                StringResourceOrString.StringResource(R.string.quick_presets),
                R.drawable.bolt_24px,
            )
    }

    @Serializable
    data class EditQuickPreset(val name: String) : Screen()

    @Serializable
    data object StatusNotification : Screen() {
        val screenInfo = ScreenInfo(
            this,
            StringResourceOrString.StringResource(R.string.status_notification),
            R.drawable.notifications_24px,
        )
    }

    @Serializable
    data object MigrateLegacyEqualizerProfiles : Screen() {
        val screenInfo =
            ScreenInfo(
                this,
                StringResourceOrString.StringResource(R.string.migrate_legacy_equalizer_profiles),
                R.drawable.update_24px,
            )
    }
}

data class ScreenInfo(val baseRoute: Screen, val name: StringResourceOrString, @DrawableRes val icon: Int)

sealed class StringResourceOrString {
    data class StringResource(@StringRes val nameResourceId: Int) : StringResourceOrString()
    data class RawString(val string: String) : StringResourceOrString()

    @Composable
    fun translated(): String = when (this) {
        is StringResource -> stringResource(nameResourceId)
        is RawString -> string
    }
}
