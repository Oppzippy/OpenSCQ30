package com.oppzippy.openscq30.soundcoredevice

import android.annotation.SuppressLint
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.content.Context
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.lib.SetAmbientSoundModePacket
import com.oppzippy.openscq30.lib.SetEqualizerPacket
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

@SuppressLint("MissingPermission")
class SoundcoreDevice(context: Context, bluetoothDevice: BluetoothDevice) {
    private val gatt: BluetoothGatt
    private var callbacks: SoundcoreDeviceCallbacks? = null

    private val mutableState: MutableStateFlow<SoundcoreDeviceState?> = MutableStateFlow(null)
    val state: StateFlow<SoundcoreDeviceState?>
        get() {
            return mutableState
        }

    init {
        callbacks = SoundcoreDeviceCallbacks(mutableState)
        gatt = bluetoothDevice.connectGatt(context, true, callbacks, BluetoothDevice.TRANSPORT_LE)
        gatt.connect()
    }

    fun setAmbientSoundMode(ambientSoundMode: AmbientSoundMode) {
        val callbacks = callbacks
        val state = state.value
        if (callbacks != null && state != null) {
            val packet = SetAmbientSoundModePacket(ambientSoundMode, state.noiseCancelingMode())
            callbacks.queueCommanad(
                gatt, Command.Write(packet.bytes())
            )
        }
    }

    fun setNoiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
        val callbacks = callbacks
        val state = state.value
        if (callbacks != null && state != null) {
            val currentAmbientSoundMode = state.ambientSoundMode()
            val packet = SetAmbientSoundModePacket(AmbientSoundMode.NoiseCanceling, noiseCancelingMode)
            callbacks.queueCommanad(gatt, Command.Write(packet.bytes()))
            if (currentAmbientSoundMode != AmbientSoundMode.NoiseCanceling) {
                val packet = SetAmbientSoundModePacket(currentAmbientSoundMode, noiseCancelingMode)
                callbacks.queueCommanad(gatt, Command.Write(packet.bytes()))
            }
        }
    }

    fun setEqualizerConfiguration(configuration: EqualizerConfiguration) {
        val packet = SetEqualizerPacket(configuration)
        callbacks?.queueCommanad(
            gatt, Command.Write(packet.bytes())
        )
    }
}