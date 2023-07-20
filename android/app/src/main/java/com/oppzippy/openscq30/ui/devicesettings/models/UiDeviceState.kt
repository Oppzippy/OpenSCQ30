package com.oppzippy.openscq30.ui.devicesettings.models

import com.oppzippy.openscq30.libbindings.SoundcoreDeviceState

sealed class UiDeviceState {
    class Connected(val name: String, val macAddress: String, val deviceState: SoundcoreDeviceState) : UiDeviceState()
    object Loading : UiDeviceState()
    object Disconnected : UiDeviceState()
}
