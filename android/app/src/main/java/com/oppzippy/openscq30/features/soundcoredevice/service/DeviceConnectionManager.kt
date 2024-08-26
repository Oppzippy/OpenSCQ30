package com.oppzippy.openscq30.features.soundcoredevice.service

import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDevice
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.lib.wrapper.CustomButtonModel
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.SoundModes
import java.lang.Exception
import javax.inject.Inject
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class DeviceConnectionManager @Inject constructor(
    private val deviceConnector: SoundcoreDeviceConnector,
    private val scope: CoroutineScope,
) {
    private val mutex: Mutex = Mutex()
    private val connectionStateFlow: MutableStateFlow<ConnectionStatus> =
        MutableStateFlow(ConnectionStatus.AwaitingConnection)
    val connectionStatusFlow = connectionStateFlow.asStateFlow()

    init {
        scope.launch {
            connectionStateFlow.collectLatest { status ->
                if (status is ConnectionStatus.Connected) {
                    status.device.isDisconnected.first { isDisconnected -> isDisconnected }
                    disconnect()
                }
            }
        }
    }

    suspend fun connect(macAddress: String) {
        mutex.withLock {
            if (macAddress == getMacAddress()) {
                return
            }
            disconnectWithoutLocking(connectionStateFlow.value)
            connectUnconditionally(macAddress)
        }
    }

    /**
     * Does not check if we're already connected or anything. Does not disconnect from existing devices.
     * Will continue connecting even if the scope in which the function is called is canceled.
     */
    private suspend fun connectUnconditionally(macAddress: String) {
        val job = scope.launch {
            try {
                val device = deviceConnector.connectToSoundcoreDevice(macAddress, scope)
                connectionStateFlow.value = if (device != null) {
                    ConnectionStatus.Connected(device)
                } else {
                    ConnectionStatus.Disconnected
                }
            } catch (ex: Exception) {
                Log.w("DeviceConnectionManager", "error connecting to device:", ex)
                connectionStateFlow.value = ConnectionStatus.Disconnected
            }
        }
        connectionStateFlow.value = ConnectionStatus.Connecting(macAddress, job)
        try {
            job.join()
        } catch (cancellationException: CancellationException) {
            Log.d("DeviceConnectionManager", "Connecting coroutine canceled while waiting")
        }
    }

    suspend fun disconnect() {
        mutex.withLock {
            disconnectWithoutLocking(connectionStateFlow.value)
            connectionStateFlow.value = ConnectionStatus.Disconnected
        }
    }

    private fun disconnectWithoutLocking(state: ConnectionStatus) {
        when (state) {
            ConnectionStatus.AwaitingConnection -> {}
            is ConnectionStatus.Connecting -> state.job.cancel()
            is ConnectionStatus.Connected -> state.device.close()
            ConnectionStatus.Disconnected -> {}
        }
    }

    private fun getMacAddress(): String? = connectionStateFlow.value.let { state ->
        when (state) {
            ConnectionStatus.AwaitingConnection -> null
            is ConnectionStatus.Connecting -> state.macAddress
            is ConnectionStatus.Connected -> state.device.macAddress
            ConnectionStatus.Disconnected -> null
        }
    }

    private val device: SoundcoreDevice?
        get() {
            return connectionStateFlow.value.let {
                if (it is ConnectionStatus.Connected) {
                    it.device
                } else {
                    null
                }
            }
        }

    fun setSoundModes(soundModes: SoundModes) {
        scope.launch {
            device?.setSoundModes(soundModes)
        }
    }

    fun setAmbientSoundModeCycle(cycle: AmbientSoundModeCycle) {
        scope.launch {
            device?.setAmbientSoundModeCycle(cycle)
        }
    }

    fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        scope.launch {
            device?.setEqualizerConfiguration(equalizerConfiguration)
        }
    }

    fun setCustomButtonModel(buttonModel: CustomButtonModel) {
        scope.launch {
            device?.setCustomButtonModel(buttonModel)
        }
    }
}
