package com.oppzippy.openscq30.ui.devicesettings.models

import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import java.util.UUID

sealed class UiDeviceState {
    class Connected(
        val name: String,
        val macAddress: String,
        val deviceState: SoundcoreDeviceState,
        val deviceBleServiceUuid: UUID,
    ) : UiDeviceState()

    object Loading : UiDeviceState()
    object Disconnected : UiDeviceState()
}
