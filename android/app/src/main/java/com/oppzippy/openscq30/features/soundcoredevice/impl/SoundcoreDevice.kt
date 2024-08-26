package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.annotation.SuppressLint
import com.oppzippy.openscq30.lib.bindings.NativeDeviceStateObserver
import com.oppzippy.openscq30.lib.bindings.NativeSoundcoreDevice
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.lib.wrapper.CustomButtonModel
import com.oppzippy.openscq30.lib.wrapper.DeviceState
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.HearId
import com.oppzippy.openscq30.lib.wrapper.SoundModes
import java.util.UUID
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

@SuppressLint("MissingPermission")
class SoundcoreDevice(
    val name: String,
    val macAddress: String,
    val bleServiceUuid: UUID,
    private val cleanUp: () -> Unit,
    private val nativeDevice: NativeSoundcoreDevice,
    coroutineScope: CoroutineScope,
    initialState: DeviceState,
) : AutoCloseable {
    private val _stateFlow = MutableStateFlow(initialState)
    val stateFlow: StateFlow<DeviceState> = _stateFlow.asStateFlow()
    private val _isDisconnected = MutableStateFlow(false)
    val isDisconnected = _isDisconnected.asStateFlow()

    init {
        coroutineScope.launch {
            nativeDevice.subscribeToStateUpdates(object : NativeDeviceStateObserver {
                override fun onStateChanged(deviceState: DeviceState) {
                    _stateFlow.value = deviceState
                }
            })
        }
    }

    override fun close() {
        _isDisconnected.value = true
        nativeDevice.close()
        cleanUp()
    }

    suspend fun setSoundModes(newSoundModes: SoundModes) {
        nativeDevice.setSoundModes(newSoundModes)
    }

    suspend fun setAmbientSoundModeCycle(cycle: AmbientSoundModeCycle) {
        nativeDevice.setAmbientSoundModeCycle(cycle)
    }

    suspend fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        nativeDevice.setEqualizerConfiguration(equalizerConfiguration)
    }

    suspend fun setHearId(hearId: HearId) {
        nativeDevice.setHearId(hearId)
    }

    suspend fun setCustomButtonModel(customButtonModel: CustomButtonModel) {
        nativeDevice.setCustomButtonModel(customButtonModel)
    }
}
