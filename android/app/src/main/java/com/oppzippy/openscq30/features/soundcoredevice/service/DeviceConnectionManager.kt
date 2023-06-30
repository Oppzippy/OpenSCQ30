package com.oppzippy.openscq30.features.soundcoredevice.service

import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import javax.inject.Inject

class DeviceConnectionManager @Inject constructor(
    private val factory: SoundcoreDeviceFactory, private val scope: CoroutineScope,
) {
    private val mutex: Mutex = Mutex()
    private var _connectionStateFlow: MutableStateFlow<ConnectionStatus> =
        MutableStateFlow(ConnectionStatus.AwaitingConnection)
    val connectionStateFlow = _connectionStateFlow.asStateFlow()

    init {
        scope.launch {
            _connectionStateFlow.collectLatest { status ->
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
            disconnectWithoutLocking(_connectionStateFlow.value)
            connectUnconditionally(macAddress)
        }
    }

    /**
     * Does not check if we're already connected or anything. Does not disconnect from existing devices.
     */
    private suspend fun connectUnconditionally(macAddress: String) {
        val job = scope.launch {
            val device = factory.createSoundcoreDevice(macAddress, scope)
            _connectionStateFlow.value = if (device != null) {
                ConnectionStatus.Connected(device)
            } else {
                ConnectionStatus.Disconnected
            }
        }
        _connectionStateFlow.value = ConnectionStatus.Connecting(macAddress, job)
        try {
            job.join()
        } catch (cancellationException: CancellationException) {
            Log.d("DeviceConnectionManager", "Connecting coroutine canceled while waiting")
        }
    }

    suspend fun disconnect() {
        mutex.withLock {
            disconnectWithoutLocking(_connectionStateFlow.value)
            _connectionStateFlow.value = ConnectionStatus.Disconnected
        }
    }

    private fun disconnectWithoutLocking(state: ConnectionStatus) {
        when (state) {
            ConnectionStatus.AwaitingConnection -> {}
            is ConnectionStatus.Connecting -> state.job.cancel()
            is ConnectionStatus.Connected -> state.device.destroy()
            ConnectionStatus.Disconnected -> {}
        }
    }

    private fun getMacAddress(): String? {
        return _connectionStateFlow.value.let { state ->
            when (state) {
                ConnectionStatus.AwaitingConnection -> null
                is ConnectionStatus.Connecting -> state.macAddress
                is ConnectionStatus.Connected -> state.device.macAddress
                ConnectionStatus.Disconnected -> null
            }
        }
    }

    private val device: SoundcoreDevice?
        get() {
            return _connectionStateFlow.value.let {
                if (it is ConnectionStatus.Connected) {
                    it.device
                } else {
                    null
                }
            }
        }

    fun setAmbientSoundMode(ambientSoundMode: AmbientSoundMode) {
        device?.state?.noiseCancelingMode()?.let { noiseCancelingMode ->
            device?.setSoundMode(ambientSoundMode, noiseCancelingMode)
        }
    }

    fun setNoiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
        device?.state?.ambientSoundMode()?.let { ambientSoundMode ->
            device?.setSoundMode(ambientSoundMode, noiseCancelingMode)
        }
    }

    fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        device?.setEqualizerConfiguration(equalizerConfiguration)
    }

    private suspend fun attemptReconnect() {
        mutex.withLock {
            val status = this.connectionStateFlow.value
            if (status is ConnectionStatus.Connected) {
                connectUnconditionally(status.device.macAddress)
            }
        }
    }
}
