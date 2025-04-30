package com.oppzippy.openscq30.features.soundcoredevice.service

import com.oppzippy.openscq30.lib.bindings.ConnectionStatusCallback
import com.oppzippy.openscq30.lib.bindings.NotificationCallback
import com.oppzippy.openscq30.lib.bindings.OpenScq30Device
import com.oppzippy.openscq30.lib.wrapper.ConnectionStatus
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.update

class DeviceConnectionManager(val device: OpenScq30Device) : AutoCloseable {
    val connectionStatusFlow = MutableStateFlow(ConnectionStatus.Connected)

    val watchForChangeNotification = MutableStateFlow(0)

    init {
        device.setConnectionStatusCallback(
            object : ConnectionStatusCallback {
                override fun onChange(connectionStatus: ConnectionStatus) {
                    connectionStatusFlow.value = connectionStatus
                }
            },
        )
        device.setWatchForChangesCallback(
            object : NotificationCallback {
                override fun onNotify() {
                    watchForChangeNotification.update { it + 1 }
                }
            },
        )
    }

    override fun close() {
        device.close()
    }
}
