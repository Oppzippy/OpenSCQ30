package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.annotation.SuppressLint
import android.bluetooth.BluetoothGatt
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.SetEqualizerPacket
import com.oppzippy.openscq30.lib.bindings.SetSoundModePacket
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlin.jvm.optionals.getOrNull

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
    override val stateFlow: Flow<SoundcoreDeviceState> = _stateFlow.asStateFlow()
    override val isDisconnected = callbacks.isDisconnected

    override val name: String = gatt.device.name
    override val macAddress: String = gatt.device.address

    init {
        scope.launch {
            callbacks.packetsFlow.collect { packet ->
                when (packet) {
                    is Packet.SoundModeUpdate -> {
                        _stateFlow.value =
                            _stateFlow.value.copy(
                                soundModes =
                                SoundModes(
                                    packet.inner.ambientSoundMode(),
                                    packet.inner.noiseCancelingMode(),
                                    packet.inner.transparencyMode(),
                                    packet.inner.customNoiseCanceling(),
                                ),
                            )
                    }

                    is Packet.StateUpdate ->
                        _stateFlow.value =
                            _stateFlow.value.copy(
                                featureFlags = packet.inner.featureFlags(),
                                firmwareVersion = if (packet.inner.firmwareVersion().isPresent) {
                                    packet.inner.firmwareVersion().asInt
                                } else {
                                    null
                                },
                                equalizerConfiguration = packet.inner.equalizerConfiguration(),
                                serialNumber = packet.inner.serialNumber().getOrNull(),
                                soundModes = packet.inner.soundModes().getOrNull(),
                            )

                    is Packet.SetSoundModeOk -> {}
                    is Packet.SetEqualizerOk -> {}
                }
            }
        }
    }

    override fun destroy() {
        gatt.close()
    }

    override fun setSoundModes(newSoundModes: SoundModes) {
        val prevSoundModes = _stateFlow.value.soundModes ?: return
        if (prevSoundModes.innerEquals(newSoundModes)) return

        val needsNoiseCanceling =
            prevSoundModes.ambientSoundMode() != AmbientSoundMode.NoiseCanceling &&
                prevSoundModes.noiseCancelingMode() != newSoundModes.noiseCancelingMode()

        if (needsNoiseCanceling) {
            queueSetSoundMode(
                SoundModes(
                    AmbientSoundMode.NoiseCanceling,
                    newSoundModes.noiseCancelingMode(),
                    newSoundModes.transparencyMode(),
                    newSoundModes.customNoiseCanceling(),
                ),
            )
        }
        queueSetSoundMode(newSoundModes)

        _stateFlow.value = _stateFlow.value.copy(
            soundModes = newSoundModes,
        )
    }

    private fun queueSetSoundMode(soundModes: SoundModes) {
        val packet = SetSoundModePacket(soundModes)
        callbacks.queueCommanad(
            Command.Write(packet.bytes()),
        )
    }

    override fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        if (_stateFlow.value.equalizerConfiguration != equalizerConfiguration) {
            val packet =
                if (_stateFlow.value.featureFlags and DeviceFeatureFlags.twoChannelEqualizer() != 0) {
                    SetEqualizerPacket(equalizerConfiguration, equalizerConfiguration)
                } else {
                    SetEqualizerPacket(equalizerConfiguration, null)
                }
            callbacks.queueCommanad(Command.Write(packet.bytes()))
            _stateFlow.value =
                _stateFlow.value.copy(equalizerConfiguration = equalizerConfiguration)
        }
    }
}
