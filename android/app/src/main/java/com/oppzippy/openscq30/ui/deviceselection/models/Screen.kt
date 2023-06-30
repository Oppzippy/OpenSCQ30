package com.oppzippy.openscq30.ui.deviceselection.models

sealed class Screen(val route: String) {
    object Home : Screen("/")
    object Info : Screen("/info")
}
