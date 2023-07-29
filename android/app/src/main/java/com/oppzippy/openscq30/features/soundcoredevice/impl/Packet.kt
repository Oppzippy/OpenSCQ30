package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.util.Log
import com.oppzippy.openscq30.lib.bindings.InboundPacket
import com.oppzippy.openscq30.lib.bindings.SetEqualizerOkPacket
import com.oppzippy.openscq30.lib.bindings.SetSoundModeOkPacket
import com.oppzippy.openscq30.lib.bindings.SoundModeUpdatePacket
import com.oppzippy.openscq30.lib.bindings.StateUpdatePacket
import kotlin.jvm.optionals.getOrNull

sealed class Packet {
    companion object {
        fun fromBytes(input: ByteArray): Packet? {
            val inboundPacket = try {
                InboundPacket(input)
            } catch (ex: Exception) {
                Log.w("Packet", "received unknown or invalid packet", ex)
                return null
            }
            return inboundPacket.soundModeUpdate().getOrNull()?.let { SoundModeUpdate(it) }
                ?: inboundPacket.stateUpdate().getOrNull()?.let { StateUpdate(it) }
                ?: inboundPacket.setSoundModeOk().getOrNull()?.let { SetSoundModeOk(it) }
                ?: inboundPacket.setEqualizerOk().getOrNull()?.let { SetEqualizerOk(it) }
        }
    }

    class SoundModeUpdate(val packet: SoundModeUpdatePacket) : Packet()
    class StateUpdate(val packet: StateUpdatePacket) : Packet()
    class SetSoundModeOk(val packet: SetSoundModeOkPacket) : Packet()
    class SetEqualizerOk(val packet: SetEqualizerOkPacket) : Packet()
}
