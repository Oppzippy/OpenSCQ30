package com.oppzippy.openscq30.ui

import android.app.Application
import android.content.ComponentName
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import java.lang.ref.WeakReference
import javax.inject.Inject

@HiltViewModel
class RootViewModel @Inject constructor(
    private val application: Application,
    private val bluetoothDeviceProvider: BluetoothDeviceProvider,
) : AndroidViewModel(application) {
    // TODO move to two child classes?
    // Device Selection
    val devices = MutableStateFlow(bluetoothDeviceProvider.getDevices())

    fun refreshDevices() {
        devices.value = bluetoothDeviceProvider.getDevices()
    }

    // Device Settings

    private val _isConnected = MutableStateFlow(false)

    private val _connectionStatus = MutableStateFlow<UiDeviceState>(UiDeviceState.Disconnected)
    val deviceState = _connectionStatus.asStateFlow()

    private var service: WeakReference<DeviceService>? = null
    private var serviceConnectionScope: CoroutineScope? = null
    private val serviceConnection = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName?, binder: IBinder?) {
            val myServiceBinder = binder as DeviceService.MyBinder
            val service = myServiceBinder.getService()
            this@RootViewModel.service = WeakReference(service)
            _isConnected.value = true

            serviceConnectionScope?.cancel()
            serviceConnectionScope = CoroutineScope(Job() + Dispatchers.Main)

            serviceConnectionScope?.launch {
                service.connectionManager.connectionStateFlow.first { it is ConnectionStatus.Disconnected }
                unbind()
            }
            serviceConnectionScope?.launch {
                service.connectionManager.connectionStateFlow.collectLatest { connectionStatus ->
                    when (connectionStatus) {
                        ConnectionStatus.AwaitingConnection, is ConnectionStatus.Connecting -> {
                            _connectionStatus.value = UiDeviceState.Loading
                        }

                        ConnectionStatus.Disconnected -> {
                            _connectionStatus.value = UiDeviceState.Disconnected
                        }

                        is ConnectionStatus.Connected -> {
                            connectionStatus.device.stateFlow.collectLatest { deviceState ->
                                _connectionStatus.value = UiDeviceState.Connected(
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
            _connectionStatus.value = UiDeviceState.Disconnected
            service = null
        }
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

    fun selectDevice(bluetoothDevice: BluetoothDevice) {
        val intent = Intent(application, DeviceService::class.java)
        intent.putExtra(DeviceService.MAC_ADDRESS, bluetoothDevice.address)
        application.startForegroundService(intent)
        bind()
    }

    fun deselectDevice() {
        application.stopService(Intent(application, DeviceService::class.java))
        unbind()
    }

    override fun onCleared() {
        unbind()
    }

    fun bind() {
        val context = application.applicationContext
        try {
            context.bindService(
                Intent(context, DeviceService::class.java),
                serviceConnection,
                0,
            )
        } catch (ex: SecurityException) {
            Log.e("RootViewModel", "failed to bind service", ex)
            unbind()
        }
    }

    fun unbind() {
        try {
            val context = application.applicationContext
            context.unbindService(serviceConnection)
        } catch (_: IllegalArgumentException) {
            // service is not bound
        }
    }
}
