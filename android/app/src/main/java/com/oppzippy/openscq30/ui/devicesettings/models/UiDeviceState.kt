package com.oppzippy.openscq30.ui.devicesettings.models

import com.oppzippy.openscq30.lib.wrapper.DeviceState
import java.util.UUID

sealed class UiDeviceState {
    class Connected(
        val name: String,
        val macAddress: String,
        val deviceState: DeviceState,
        val deviceBleServiceUuid: UUID,
    ) : UiDeviceState()

    data object Loading : UiDeviceState()
    data object Disconnected : UiDeviceState()
}
