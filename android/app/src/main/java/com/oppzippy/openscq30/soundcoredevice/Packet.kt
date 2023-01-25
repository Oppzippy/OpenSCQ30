package com.oppzippy.openscq30.soundcoredevice

import com.oppzippy.openscq30.lib.AmbientSoundModeUpdatePacket
import com.oppzippy.openscq30.lib.SetAmbientModeOkPacket
import com.oppzippy.openscq30.lib.SetEqualizerOkPacket
import com.oppzippy.openscq30.lib.StateUpdatePacket

sealed class Packet {
    class AmbientSoundModeUpdate(val packet: AmbientSoundModeUpdatePacket) : Packet()
    class StateUpdate(val packet: StateUpdatePacket) : Packet()
    class SetAmbientModeOk(val packet: SetAmbientModeOkPacket) : Packet()
    class SetEqualizerOk(val packet: SetEqualizerOkPacket) : Packet()
}