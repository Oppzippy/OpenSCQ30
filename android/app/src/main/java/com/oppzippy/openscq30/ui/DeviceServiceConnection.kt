package com.oppzippy.openscq30.ui

import android.content.ComponentName
import android.content.ServiceConnection
import android.os.IBinder
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import java.lang.ref.WeakReference
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch

class DeviceServiceConnection(private val unbind: () -> Unit) : ServiceConnection {
    val connectionStatusFlow = MutableStateFlow<ConnectionStatus>(ConnectionStatus.Disconnected)
    private var serviceConnectionScope: CoroutineScope? = null
    private var service: WeakReference<DeviceService>? = null
    val deviceManager: DeviceConnectionManager?
        get() {
            return service?.get()?.connectionStatusFlow?.value?.let { connectionStatus ->
                if (connectionStatus is ConnectionStatus.Connected) {
                    connectionStatus.deviceManager
                } else {
                    null
                }
            }
        }

    override fun onServiceConnected(name: ComponentName?, binder: IBinder?) {
        val myServiceBinder = binder as DeviceService.MyBinder
        val service = myServiceBinder.getService()
        this.service = WeakReference(service)

        serviceConnectionScope?.cancel()
        serviceConnectionScope = CoroutineScope(Job() + Dispatchers.Main)

        serviceConnectionScope?.launch {
            service.connectionStatusFlow.first { it is ConnectionStatus.Disconnected }
            connectionStatusFlow.value = ConnectionStatus.Disconnected
            unbind()
        }
        serviceConnectionScope?.launch {
            service.connectionStatusFlow.collectLatest { connectionStatus ->
                this@DeviceServiceConnection.connectionStatusFlow.value = connectionStatus
            }
        }
    }

    override fun onServiceDisconnected(name: ComponentName?) {
        serviceConnectionScope?.cancel()
        serviceConnectionScope = null
        this.service = null
        connectionStatusFlow.value = ConnectionStatus.Disconnected
    }

    override fun onBindingDied(name: ComponentName?) {
        onServiceDisconnected(name)
    }

    override fun onNullBinding(name: ComponentName?) {
        onServiceDisconnected(name)
    }

    fun onUnbind() {
        onServiceDisconnected(null)
    }
}
