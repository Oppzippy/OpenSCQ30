package com.oppzippy.openscq30.ui.deviceselection.models

sealed class Screen(val route: String) {
    data object Home : Screen("/")
    data object Info : Screen("/info")
    data object Settings : Screen("/settings")
}
