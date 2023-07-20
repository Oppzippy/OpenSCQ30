package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.annotation.SuppressLint
import android.bluetooth.BluetoothGatt
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.api.contentEquals
import com.oppzippy.openscq30.libbindings.AmbientSoundMode
import com.oppzippy.openscq30.libbindings.EqualizerConfiguration
import com.oppzippy.openscq30.libbindings.NoiseCancelingMode
import com.oppzippy.openscq30.libbindings.SetEqualizerPacket
import com.oppzippy.openscq30.libbindings.SetSoundModePacket
import com.oppzippy.openscq30.libbindings.SoundcoreDeviceState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.launch

@SuppressLint("MissingPermission")
class SoundcoreDeviceImpl(
    private val gatt: BluetoothGatt,
    private val callbacks: SoundcoreDeviceCallbackHandler,
    scope: CoroutineScope,
    initialState: SoundcoreDeviceState,
) : SoundcoreDevice {
    override val state: SoundcoreDeviceState
        get() {
            return _stateFlow.value
        }
    private val _stateFlow: MutableStateFlow<SoundcoreDeviceState> = MutableStateFlow(initialState)
    override val stateFlow: Flow<SoundcoreDeviceState> =
        _stateFlow.distinctUntilChanged { old, new ->
            old.ambientSoundMode() == new.ambientSoundMode() && old.noiseCancelingMode() == new.noiseCancelingMode() && old.equalizerConfiguration()
                .contentEquals(new.equalizerConfiguration())
        }
    override val isDisconnected = callbacks.isDisconnected

    override val name: String = gatt.device.name
    override val macAddress: String = gatt.device.address

    init {
        scope.launch {
            callbacks.packetsFlow.collect {
                when (it) {
                    is Packet.SoundModeUpdate -> {
                        _stateFlow.value =
                            _stateFlow.value.withAmbientSoundMode(it.packet.ambientSoundMode())
                                .withNoiseCancelingMode(it.packet.noiseCancelingMode())
                    }

                    is Packet.StateUpdate -> _stateFlow.value = SoundcoreDeviceState(it.packet)
                    is Packet.SetSoundModeOk -> {}
                    is Packet.SetEqualizerOk -> {}
                }
            }
        }
    }

    override fun destroy() {
        gatt.close()
    }

    override fun setSoundMode(
        newAmbientSoundMode: AmbientSoundMode,
        newNoiseCancelingMode: NoiseCancelingMode,
    ) {
        val prevState = _stateFlow.value
        val prevAmbientSoundMode = prevState.ambientSoundMode()
        val prevNoiseCancelingMode = prevState.noiseCancelingMode()

        if (newAmbientSoundMode != AmbientSoundMode.NoiseCanceling && newNoiseCancelingMode != prevNoiseCancelingMode) {
            queueSetSoundMode(AmbientSoundMode.NoiseCanceling, newNoiseCancelingMode)
        }
        if (prevAmbientSoundMode != newAmbientSoundMode || prevNoiseCancelingMode != newNoiseCancelingMode) {
            queueSetSoundMode(newAmbientSoundMode, newNoiseCancelingMode)

            val newState = prevState.withAmbientSoundMode(newAmbientSoundMode)
                .withNoiseCancelingMode(newNoiseCancelingMode)
            _stateFlow.value = newState
        }
    }

    private fun queueSetSoundMode(
        ambientSoundMode: AmbientSoundMode,
        noiseCancelingMode: NoiseCancelingMode,
    ) {
        val packet = SetSoundModePacket(ambientSoundMode, noiseCancelingMode)
        callbacks.queueCommanad(
            Command.Write(packet.bytes()),
        )
    }

    override fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        if (!_stateFlow.value.equalizerConfiguration().contentEquals(equalizerConfiguration)) {
            queueSetEqualizerConfiguration(equalizerConfiguration)
            _stateFlow.value = _stateFlow.value.withEqualizerConfiguration(equalizerConfiguration)
        }
    }

    private fun queueSetEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        val packet = SetEqualizerPacket(equalizerConfiguration)
        callbacks.queueCommanad(
            Command.Write(packet.bytes()),
        )
    }
}
