package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.annotation.SuppressLint
import android.bluetooth.BluetoothGatt
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.SetEqualizerAndCustomHearIdPacket
import com.oppzippy.openscq30.lib.bindings.SetEqualizerPacket
import com.oppzippy.openscq30.lib.bindings.SetEqualizerWithDrcPacket
import com.oppzippy.openscq30.lib.bindings.SetSoundModePacket
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import java.util.UUID
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

    // SoundcoreDeviceCallbackHandler should have already received a packet by the time we reach
    // this point, so serviceUuid should never be null
    override val bleServiceUuid: UUID = callbacks.serviceUuid.value!!

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
                            _stateFlow.value.let { state ->
                                state.copy(
                                    featureFlags = packet.inner.featureFlags(),
                                    leftFirmwareVersion = packet.inner.firmwareVersion().getOrNull()
                                        ?: state.leftFirmwareVersion,
                                    equalizerConfiguration = packet.inner.equalizerConfiguration(),
                                    serialNumber = packet.inner.serialNumber().getOrNull()
                                        ?: state.serialNumber,
                                    soundModes = packet.inner.soundModes().getOrNull()
                                        ?: state.soundModes,
                                    dynamicRangeCompressionMinFirmwareVersion = packet.inner.dynamicRangeCompressionMinFirmwareVersion()
                                        .getOrNull()
                                        ?: state.dynamicRangeCompressionMinFirmwareVersion,
                                )
                            }

                    is Packet.SetSoundModeOk -> {}
                    is Packet.SetEqualizerOk -> {}
                    is Packet.BatteryChargingUpdate -> {
                        _stateFlow.value = _stateFlow.value.copy(
                            isLeftBatteryCharging = packet.inner.isLeftCharging,
                            isRightBatteryCharging = packet.inner.isRightCharging,
                        )
                    }

                    is Packet.BatteryLevelUpdate -> {
                        _stateFlow.value = _stateFlow.value.copy(
                            leftBatteryLevel = packet.inner.leftLevel(),
                            rightBatteryLevel = packet.inner.rightLevel(),
                        )
                    }

                    is Packet.ChineseVoicePromptStateUpdate -> {
                        // no point in tracking this
                    }

                    is Packet.FirmwareVersionUpdate -> {
                        _stateFlow.value =
                            _stateFlow.value.copy(
                                leftFirmwareVersion = packet.inner.leftFirmwareVersion(),
                                rightFirmwareVersion = packet.inner.rightFirmwareVersion(),
                            )
                    }

                    is Packet.LdacStateUpdate -> {
                        // TODO implement
                    }

                    is Packet.SetEqualizerWithDrcOk -> {}
                    is Packet.TwsStatusUpdate -> {
                        // TODO implement
                    }
                }
            }
        }
    }

    override fun destroy() {
        gatt.close()
    }

    override fun setSoundModes(newSoundModes: SoundModes) {
        val state = _stateFlow.value
        val prevSoundModes = state.soundModes ?: return
        if (prevSoundModes == newSoundModes) return

        val needsNoiseCanceling =
            state.featureFlags and DeviceFeatureFlags.noiseCancelingMode() != 0 &&
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
        val filteredSoundModes =
            filterSoundModeChanges(state.featureFlags, prevSoundModes, newSoundModes)
        queueSetSoundMode(filteredSoundModes)

        _stateFlow.value = state.copy(
            soundModes = filteredSoundModes,
        )
    }

    private fun queueSetSoundMode(soundModes: SoundModes) {
        val packet = SetSoundModePacket(soundModes)
        callbacks.queueCommanad(
            Command.Write(packet.bytes()),
        )
    }

    override fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        val state = _stateFlow.value
        if (state.featureFlags and DeviceFeatureFlags.equalizer() != 0 && state.equalizerConfiguration != equalizerConfiguration) {
            val featureFlags = state.featureFlags
            val packet =
                if (state.customHearId != null && state.gender != null && state.ageRange != null) {
                    SetEqualizerAndCustomHearIdPacket(
                        equalizerConfiguration,
                        state.gender,
                        state.ageRange,
                        state.customHearId,
                    ).bytes()
                } else if (state.supportsDynamicRangeCompression()) {
                    SetEqualizerWithDrcPacket(
                        equalizerConfiguration,
                        equalizerConfiguration,
                    ).bytes()
                } else {
                    SetEqualizerPacket(
                        equalizerConfiguration,
                        if (featureFlags and DeviceFeatureFlags.twoChannelEqualizer() != 0) {
                            equalizerConfiguration
                        } else {
                            null
                        },
                    ).bytes()
                }
            callbacks.queueCommanad(Command.Write(packet))
            _stateFlow.value =
                _stateFlow.value.copy(equalizerConfiguration = equalizerConfiguration)
        }
    }
}
