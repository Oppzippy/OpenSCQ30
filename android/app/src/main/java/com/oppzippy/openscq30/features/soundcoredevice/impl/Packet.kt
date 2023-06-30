package com.oppzippy.openscq30.features.soundcoredevice.impl

import com.oppzippy.openscq30.lib.AmbientSoundModeUpdatePacket
import com.oppzippy.openscq30.lib.SetAmbientModeOkPacket
import com.oppzippy.openscq30.lib.SetEqualizerOkPacket
import com.oppzippy.openscq30.lib.StateUpdatePacket
import kotlin.jvm.optionals.getOrNull

sealed class Packet {
    companion object {
        fun fromBytes(bytes: ByteArray): Packet? {
            return AmbientSoundModeUpdatePacket.fromBytes(bytes).getOrNull()?.toPacket()
                ?: StateUpdatePacket.fromBytes(bytes).getOrNull()?.toPacket()
                ?: SetAmbientModeOkPacket.fromBytes(bytes).getOrNull()?.toPacket()
                ?: SetEqualizerOkPacket.fromBytes(bytes).getOrNull()?.toPacket()
        }
    }

    class AmbientSoundModeUpdate(val packet: AmbientSoundModeUpdatePacket) : Packet()
    class StateUpdate(val packet: StateUpdatePacket) : Packet()
    class SetAmbientModeOk(val packet: SetAmbientModeOkPacket) : Packet()
    class SetEqualizerOk(val packet: SetEqualizerOkPacket) : Packet()
}

private fun AmbientSoundModeUpdatePacket.toPacket(): Packet.AmbientSoundModeUpdate {
    return Packet.AmbientSoundModeUpdate(this)
}

private fun StateUpdatePacket.toPacket(): Packet.StateUpdate {
    return Packet.StateUpdate(this)
}

private fun SetAmbientModeOkPacket.toPacket(): Packet.SetAmbientModeOk {
    return Packet.SetAmbientModeOk(this)
}

private fun SetEqualizerOkPacket.toPacket(): Packet.SetEqualizerOk {
    return Packet.SetEqualizerOk(this)
}
