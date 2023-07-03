package com.oppzippy.openscq30.ui

import android.content.ComponentName
import android.content.ServiceConnection
import android.os.IBinder
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import java.lang.ref.WeakReference

class DeviceServiceConnection(
    private val unbind: () -> Unit,
) : ServiceConnection {
    val uiDeviceStateFlow = MutableStateFlow<UiDeviceState>(UiDeviceState.Disconnected)
    private var serviceConnectionScope: CoroutineScope? = null
    private var service: WeakReference<DeviceService>? = null

    override fun onServiceConnected(name: ComponentName?, binder: IBinder?) {
        val myServiceBinder = binder as DeviceService.MyBinder
        val service = myServiceBinder.getService()
        this.service = WeakReference(service)

        serviceConnectionScope?.cancel()
        serviceConnectionScope = CoroutineScope(Job() + Dispatchers.Main)

        serviceConnectionScope?.launch {
            service.connectionManager.connectionStatusFlow.first { it is ConnectionStatus.Disconnected }
            unbind()
        }
        serviceConnectionScope?.launch {
            service.connectionManager.connectionStatusFlow.collectLatest { connectionStatus ->
                when (connectionStatus) {
                    ConnectionStatus.AwaitingConnection, is ConnectionStatus.Connecting -> {
                        uiDeviceStateFlow.value = UiDeviceState.Loading
                    }

                    ConnectionStatus.Disconnected -> {
                        uiDeviceStateFlow.value = UiDeviceState.Disconnected
                    }

                    is ConnectionStatus.Connected -> {
                        connectionStatus.device.stateFlow.collectLatest { deviceState ->
                            uiDeviceStateFlow.value = UiDeviceState.Connected(
                                connectionStatus.device.name,
                                connectionStatus.device.macAddress,
                                deviceState,
                            )
                        }
                    }
                }
            }
        }
    }

    override fun onServiceDisconnected(name: ComponentName?) {
        serviceConnectionScope?.cancel()
        serviceConnectionScope = null
        this.service = null
        uiDeviceStateFlow.value = UiDeviceState.Disconnected
    }

    fun setAmbientSoundMode(ambientSoundMode: AmbientSoundMode) {
        service?.get()?.connectionManager?.setAmbientSoundMode(ambientSoundMode)
    }

    fun setNoiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
        service?.get()?.connectionManager?.setNoiseCancelingMode(noiseCancelingMode)
    }

    fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        service?.get()?.connectionManager?.setEqualizerConfiguration(equalizerConfiguration)
    }

    fun doesNotificationExist(): Boolean {
        return service?.get()?.doesNotificationExist() ?: false
    }
}
