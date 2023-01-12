package com.oppzippy.openscq30.soundcoredevice

import com.oppzippy.openscq30.lib.AmbientSoundModeUpdatePacket
import com.oppzippy.openscq30.lib.OkPacket
import com.oppzippy.openscq30.lib.StateUpdatePacket

sealed class Packet {
    class AmbientSoundModeUpdate(val packet: AmbientSoundModeUpdatePacket) : Packet()
    class StateUpdate(val packet: StateUpdatePacket) : Packet()
    class Ok(val packet: OkPacket) : Packet()
}