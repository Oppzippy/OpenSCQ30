package com.oppzippy.openscq30.features.ui.devicesettings

import androidx.annotation.StringRes
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Equalizer
import androidx.compose.material.icons.filled.Settings
import androidx.compose.ui.graphics.vector.ImageVector
import com.oppzippy.openscq30.R

sealed class Screen(val route: String, @StringRes val resourceId: Int, val icon: ImageVector) {
    object General : Screen("general", R.string.general, Icons.Filled.Settings)
    object Equalizer : Screen("equalizer", R.string.equalizer, Icons.Filled.Equalizer)
}
